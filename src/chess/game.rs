use std::{sync::Arc, time::Duration};

use futures_util::{Future, FutureExt};
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use tokio::{sync::broadcast, time::sleep};

use self::{
    board::Board,
    network::{ActionInterface, ApprovedChatMessage, MessageInterface, PlayerInterface},
    pieces::ValidMove,
};

pub mod board;
pub mod network;
pub mod pieces;
pub mod player;

pub struct Game<S> {
    black: PlayerInterface,
    white: PlayerInterface,

    board: board::Board,
    move_history: Vec<ValidMove>,

    actions: ActionInterface,

    pub messenger: Arc<MessageInterface>,

    state: S,
}

impl<S> Game<S> {
    pub fn spectate(&self) -> broadcast::Receiver<ApprovedChatMessage> {
        self.messenger.spectate()
    }
}

pub struct InactiveGame {
    config: GameConfig,
    player1: PlayerInterface,

    actions: ActionInterface,

    pub messenger: Arc<MessageInterface>,
}

impl InactiveGame {
    pub fn new(interface: PlayerInterface, actions: ActionInterface, config: GameConfig) -> Self {
        Self {
            config,
            player1: interface,
            messenger: MessageInterface::create(),
            actions,
        }
    }

    pub fn start(self, interface: PlayerInterface) {
        tokio::task::spawn((Game::<PlayerTurn>::from((self, interface))).wait_for_player());
    }
}

impl From<(InactiveGame, PlayerInterface)> for Game<PlayerTurn> {
    fn from((value, player2): (InactiveGame, PlayerInterface)) -> Self {
        // TODO: Add timed games
        // match value.config.time {
        //     TimeConfig::NotTimed => (),
        //     TimeConfig::Timed { limit, added } => (),
        // }

        let (black, white) = match value.config.player1_color {
            TeamConfig::White => (player2, value.player1),
            TeamConfig::Black => (value.player1, player2),
            TeamConfig::Random => {
                let rand: &'static bool = [true, false].choose(&mut thread_rng()).unwrap();

                match rand {
                    &true => (value.player1, player2),
                    &false => (player2, value.player1),
                }
            }
        };

        Self {
            black,
            white,

            board: Board::new(value.config.starting_fen, value.config.height, value.config.width),
            move_history: vec![],

            messenger: value.messenger,

            actions: value.actions,

            state: PlayerTurn {
                turn: match value.config.white_starts {
                    true => Turn::White,
                    false => Turn::Black,
                },
            },
        }
    }
}

impl Game<PlayerTurn> {
    async fn get_valid_move(&self) -> Result<ValidMove, ()> {
        match self.state.turn {
            Turn::White => &self.white,
            Turn::Black => &self.black,
        }
        .valid_move(&self.board)
        .await
    }
    fn wait_for_player(mut self) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send>> {
        async move {
            let turn_event = tokio::select! {
                m = self.get_valid_move() => TurnEvent::Move(m.unwrap())
                // TODO: Manage undo requests here
                // TODO: Manage draw offer requests here
                // TODO: Manage timeout here
                // TODO: Manage resignation here
            };

            match turn_event {
                TurnEvent::Move(vm) => {
                    self.board.make_move(vm);
                    Game::<Calculating>::from(self).calculate().await;
                }
                TurnEvent::Undo => (),
                TurnEvent::OfferDraw => (),
                TurnEvent::GameEnd(..) => (),
            }
        }
        .boxed()
    }
}

enum TurnEvent {
    Move(ValidMove),
    Undo,
    OfferDraw,
    GameEnd(Winner, EndState),
}

impl From<Game<PlayerTurn>> for Game<Calculating> {
    fn from(value: Game<PlayerTurn>) -> Self {
        Self {
            black: value.black,
            white: value.white,

            board: value.board,
            move_history: value.move_history,

            messenger: value.messenger,

            actions: value.actions,

            state: Calculating {
                last_turn: value.state.turn,
            },
        }
    }
}

impl Game<Calculating> {
    fn calculate(self) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send>> {
        async move {
            // Calculate things like check, checkmate, move lists, etc.

            Game::<PlayerTurn>::from(self).wait_for_player().await;
        }
        .boxed()
    }
}

impl From<Game<Calculating>> for Game<PlayerTurn> {
    fn from(value: Game<Calculating>) -> Self {
        Self {
            black: value.black,
            white: value.white,

            board: value.board,
            move_history: value.move_history,

            messenger: value.messenger,

            actions: value.actions,

            state: PlayerTurn {
                turn: value.state.last_turn.switch(),
            },
        }
    }
}

impl From<(Game<Calculating>, Winner, EndState)> for Game<Ended> {
    fn from(value: (Game<Calculating>, Winner, EndState)) -> Self {
        Self {
            black: value.0.black,
            white: value.0.white,
            board: value.0.board,
            move_history: value.0.move_history,

            messenger: value.0.messenger,

            actions: value.0.actions,

            state: Ended {
                winner: value.1,
                state: value.2,
            },
        }
    }
}

impl Game<Ended> {
    async fn end_game(self) {
        sleep(Duration::from_secs(300)).await;
        self.messenger
            .stop()
            .await
            .expect("Messenger should always successfully shut down");
    }
}

enum Turn {
    White,
    Black,
}

impl Turn {
    fn switch(self) -> Self {
        match self {
            Turn::White => Turn::Black,
            Turn::Black => Turn::White,
        }
    }
}

struct PlayerTurn {
    turn: Turn,
}

struct Calculating {
    last_turn: Turn,
}

enum EndState {
    Checkmate,
    Resignation,
    Timeout,

    Stalemate,
    InsufficientMaterial,
    FiftyMove,
    RepeatThree,

    Agreement,
}

enum Winner {
    None,
    Black,
    White,
}

struct Ended {
    winner: Winner,
    state: EndState,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameConfig {
    white_starts: bool,
    player1_color: TeamConfig,

    starting_fen: String,

    width: u8,
    height: u8,

    time: TimeConfig,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            white_starts: true,
            player1_color: TeamConfig::Random,
            starting_fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(),
            time: TimeConfig::Timed {
                limit: Duration::from_secs(600),
                added: Duration::from_secs(5),
            },
            width: 8,
            height: 8,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TimeConfig {
    NotTimed,
    Timed { limit: Duration, added: Duration },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TeamConfig {
    White,
    Black,
    Random,
}

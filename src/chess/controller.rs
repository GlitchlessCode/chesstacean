use super::game::{
    network::{ActionInterface, PlayerInterface},
    GameConfig, InactiveGame,
};
use crate::{
    server::user::{interface::GameInterface, UserInfo},
    traits::ChooseTake,
};
use rand::thread_rng;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{
    sync::{oneshot, RwLock},
    time::{self, Instant},
};

pub struct GameControllerInterface {
    matchmaker: Arc<Matchmaker>,
    lobby_manager: RwLock<LobbyManager>,
}
// TODO: Convert a lot of the UserInfo parts to include Targets for transmission
impl GameControllerInterface {
    pub fn new() -> Self {
        let matchmaker = Matchmaker::new();
        Matchmaker::start(matchmaker.clone());

        let lobby_manager = RwLock::new(LobbyManager::new());

        Self {
            matchmaker,
            lobby_manager,
        }
    }

    pub fn join_queue(&self) {}
    pub fn leave_queue(&self) {}

    pub fn create_lobby(&self) {}
    pub fn close_lobby(&self) {}
    pub fn start_lobby(&self) {}

    pub fn join_lobby(&self) {}
    pub fn leave_lobby(&self) {}
}

struct LobbyManager {
    codes: HashMap<String, Arc<Lobby>>,
    users: HashMap<UserInfo, Arc<Lobby>>,
}

impl LobbyManager {
    fn new() -> Self {
        Self {
            codes: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn create_lobby(&self, user: &UserInfo) {}
    fn close_lobby(&self, user: &UserInfo) {}
    fn start_lobby(&self, user: &UserInfo, config: GameConfig) {}

    fn join_lobby(&self, user: &UserInfo) {}
    fn leave_lobby(&self, user: &UserInfo) {}
}

struct Lobby {
    host: UserInfo,
    client: Option<UserInfo>,
}

impl Lobby {
    fn new(host: UserInfo) -> Arc<Self> {
        Arc::new(Self { host, client: None })
    }

    fn is_host(&self, user: &UserInfo) -> bool {
        &self.host == user
    }
}

struct Matchmaker {
    queue: RwLock<Vec<UserInQueue>>,
}

impl Matchmaker {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: RwLock::new(vec![]),
        })
    }

    fn start(self: Arc<Self>) {
        tokio::task::spawn(self.run());
    }
    async fn run(self: Arc<Self>) {
        let mut interval = time::interval(Duration::from_secs(2));
        loop {
            interval.tick().await;

            let mut writer = self.queue.write().await;
            Matchmaker::make_matches(&mut writer);
            drop(writer);
        }
    }

    /// ### Matchmaking algorithm
    ///
    /// Currently does not implement rating
    ///
    /// Currently does not implement timestamp
    ///
    /// Currently does not implement any statistic analysis
    fn make_matches(queue: &mut Vec<UserInQueue>) {
        while queue.len() > 2 {
            let mut rng = thread_rng();
            // Get random players
            let player1 = queue.take_random(&mut rng).unwrap();
            let player2 = queue.take_random(&mut rng).unwrap();

            // Create player interfaces
            let (player1_interface, p1_move_rx, p1_event_rx) = PlayerInterface::create(player1.user.clone());
            let (player2_interface, p2_move_rx, p2_event_rx) = PlayerInterface::create(player2.user.clone());

            // Create actions interface
            let (actions, action_rx) = ActionInterface::create();

            // Create game
            let game = InactiveGame::new(player1_interface, actions, GameConfig::default());

            // TODO: Create game interface
            let p1_msg = game.messenger.channel();
            let player1_game_interface =
                GameInterface::new(p1_move_rx, action_rx.clone(), p1_event_rx, p1_msg.0, p1_msg.1);
            player1.reply(player1_game_interface);

            let p2_msg = game.messenger.channel();
            let player2_game_interface = GameInterface::new(p2_move_rx, action_rx, p2_event_rx, p2_msg.0, p2_msg.1);
            player2.reply(player2_game_interface);

            // Start game
            game.start(player2_interface);
        }
    }
}

pub struct UserInQueue {
    user: UserInfo,
    rating: u16,
    timestamp: Instant,

    reply_to: oneshot::Sender<GameInterface>,
    // Stats can go here to help with matchmaking, if necessary
    // stats: STUFF
}

impl UserInQueue {
    fn new(user: &UserInfo, reply_to: oneshot::Sender<GameInterface>) -> Self {
        let now = Instant::now();
        Self {
            user: user.clone(),
            timestamp: now, // Currently timestamp is not used, but is included for if we change the matchmaking algo
            rating: 1200, // Currently Rating does not change, and is just a static value applied to every player. Additionally rating is not used for matchmaking currently
            reply_to,
        }
    }
    pub fn create(user: &UserInfo) -> (Self, oneshot::Receiver<GameInterface>) {
        let (reply_to, reply_rx) = oneshot::channel();
        (Self::new(user, reply_to), reply_rx)
    }
    fn reply(self, interface: GameInterface) {
        self.reply_to.send(interface).unwrap();
    }
}

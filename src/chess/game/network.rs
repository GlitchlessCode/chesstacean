use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc, oneshot, watch, RwLock};

use crate::server::user::UserInfo;

use super::{
    board::Board,
    pieces::{Move, ValidMove},
};

pub struct PlayerInterface {
    user: UserInfo,
    reciever_tx: mpsc::Sender<oneshot::Sender<(Move, oneshot::Sender<bool>)>>,

    event_interface: EventInterface,
}

impl PlayerInterface {
    fn new(
        user: UserInfo,
        tx: mpsc::Sender<oneshot::Sender<(Move, oneshot::Sender<bool>)>>,
        event_tx: mpsc::Sender<Event>,
    ) -> Self {
        Self {
            user: user.clone(),
            reciever_tx: tx,
            event_interface: EventInterface {
                user,
                transmitter: event_tx,
            },
        }
    }
    pub fn create(
        user: UserInfo,
    ) -> (
        Self,
        mpsc::Receiver<oneshot::Sender<(Move, oneshot::Sender<bool>)>>,
        mpsc::Receiver<Event>,
    ) {
        let (tx, rx) = mpsc::channel(2);
        let (e_tx, e_rx) = mpsc::channel(2);
        let this = Self::new(user, tx, e_tx);
        (this, rx, e_rx)
    }
    pub async fn valid_move(&self, board: &Board) -> Result<ValidMove, ()> {
        loop {
            let (tx, rx) = oneshot::channel();

            if self.reciever_tx.send(tx).await.is_err() {
                return Err(());
            }

            let (movement, result_tx) = match rx.await {
                Ok(mrtx) => mrtx,
                Err(_) => return Err(()),
            };

            let move_result = board.validate_move(movement);
            if let Some(vm) = move_result {
                if result_tx.send(true).is_err() {
                    return Err(());
                }
                return Ok(vm);
            } else {
                if result_tx.send(false).is_err() {
                    return Err(());
                }
                // * Loop again
            }
        }
    }
}

pub struct MessageInterface {
    transmitter: broadcast::Sender<ApprovedChatMessage>,

    reciever_tx: mpsc::Sender<ChatMessage>,
    transmitter_rx: broadcast::Receiver<ApprovedChatMessage>,

    halter: RwLock<Option<oneshot::Sender<()>>>,
}

impl MessageInterface {
    fn new(
        transmitter: broadcast::Sender<ApprovedChatMessage>,
        reciever_tx: mpsc::Sender<ChatMessage>,
        transmitter_rx: broadcast::Receiver<ApprovedChatMessage>,
        halter: oneshot::Sender<()>,
    ) -> Arc<Self> {
        Arc::new(Self {
            transmitter,
            reciever_tx,
            transmitter_rx,
            halter: RwLock::new(Some(halter)),
        })
    }

    pub fn create() -> Arc<Self> {
        // Create channels
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (msg_tx, msg_rx) = mpsc::channel(4);
        let (app_msg_tx, app_msg_rx) = broadcast::channel(4);

        // Create this
        let this = Self::new(app_msg_tx, msg_tx, app_msg_rx, shutdown_tx);

        // Spawn task
        tokio::task::spawn(MessageInterface::run(this.clone(), msg_rx, shutdown_rx));

        this
    }

    pub async fn stop(&self) -> Result<(), ()> {
        if let Some(tx) = self.halter.write().await.take() {
            tx.send(())
        } else {
            Err(())
        }
    }

    pub fn channel(&self) -> (mpsc::Sender<ChatMessage>, broadcast::Receiver<ApprovedChatMessage>) {
        (self.reciever_tx.clone(), self.transmitter_rx.resubscribe())
    }

    pub fn spectate(&self) -> broadcast::Receiver<ApprovedChatMessage> {
        self.transmitter_rx.resubscribe()
    }

    async fn run(self: Arc<Self>, mut msg_rx: mpsc::Receiver<ChatMessage>, mut stop_rx: oneshot::Receiver<()>) {
        loop {
            let message = tokio::select! {

                biased;

                _ = (&mut stop_rx) => return,
                msg = msg_rx.recv() => match msg {
                    Some(msg) => msg,
                    None => return,
                }
            };

            match message.try_into() {
                Ok(approved) => {
                    if self.transmitter.send(approved).is_err() {
                        return;
                    }
                }
                Err(_) => (),
            }
        }
    }
}

#[derive(Deserialize)]
pub struct ChatMessage {
    sender: UserInfo,
    message: String,
    timestamp: u128,
}

impl TryFrom<ChatMessage> for ApprovedChatMessage {
    type Error = ();
    fn try_from(value: ChatMessage) -> Result<Self, Self::Error> {
        // TODO: Verify chat messages here

        Ok(Self {
            sender: value.sender,
            message: value.message,
            timestamp: value.timestamp,
        })
    }
}

#[derive(Clone, Serialize)]
pub struct ApprovedChatMessage {
    sender: UserInfo,
    message: String,
    timestamp: u128,
}

pub struct ActionInterface {
    reciever_tx: watch::Sender<Option<mpsc::Sender<Action>>>,
}

impl ActionInterface {
    fn new(reciever_tx: watch::Sender<Option<mpsc::Sender<Action>>>) -> Self {
        Self { reciever_tx }
    }
    pub fn create() -> (Self, watch::Receiver<Option<mpsc::Sender<Action>>>) {
        let (tx, rx) = watch::channel(None);
        (Self::new(tx), rx)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    sender: UserInfo,
    kind: ActionType,
}

pub struct EventInterface {
    user: UserInfo,
    transmitter: mpsc::Sender<Event>,
}

#[derive(Serialize)]
pub enum Event {
    OfferDraw,

    RequestUndo,
    MoveWasUndone(ValidMove),

    GameEnd(),

    YourTurn,
    YourTurnEnded,

    ValidMove(ValidMove),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ActionType {
    OfferDraw,
    AcceptDraw,

    RequestUndo,
    AcceptUndo,

    Resign,
}

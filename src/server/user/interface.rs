use std::{error::Error, fmt::Display, sync::Arc};

use crate::{
    chess::game::{
        network::{Action, ApprovedChatMessage, ChatMessage, Event},
        pieces::Move,
    },
    server::ws::{GameEvent, SentMessage},
};
use tokio::sync::{
    broadcast::{self, error::RecvError},
    mpsc, oneshot, watch, RwLock,
};

use super::{Sender, UserConnection};

type MoveRx = mpsc::Receiver<oneshot::Sender<(Move, oneshot::Sender<bool>)>>;

#[derive(Debug)]
pub struct GameInterface {
    move_target: MoveRx,
    action_target: watch::Receiver<Option<mpsc::Sender<Action>>>,

    event_rx: RwLock<mpsc::Receiver<Event>>,

    code: String,

    message_target: mpsc::Sender<ChatMessage>,
    message_rx: RwLock<broadcast::Receiver<ApprovedChatMessage>>,
}

impl GameInterface {
    pub async fn send_move(&mut self, movement: Move) -> Result<(), InterfaceError> {
        let target = match self.move_target.try_recv() {
            Ok(t) => t,
            Err(_) => return Err(InterfaceError::NotYourTurn),
        };

        let (tx, rx) = oneshot::channel();

        if let Err(_) = target.send((movement, tx)) {
            return Err(InterfaceError::UnknownError);
        }

        let result = rx.await?;

        match result {
            true => Ok(()),
            false => Err(InterfaceError::InvalidMove),
        }
    }

    pub fn new(
        move_target: MoveRx,
        action_target: watch::Receiver<Option<mpsc::Sender<Action>>>,
        event_rx: mpsc::Receiver<Event>,
        code: String,
        message_target: mpsc::Sender<ChatMessage>,
        message_rx: broadcast::Receiver<ApprovedChatMessage>,
    ) -> Arc<Self> {
        Arc::new(Self {
            move_target,
            action_target,
            event_rx: RwLock::new(event_rx),
            code,
            message_target,
            message_rx: RwLock::new(message_rx),
        })
    }

    pub fn start(self: Arc<Self>, conn: Arc<UserConnection>) {
        tokio::task::spawn(GameInterface::run(Arc::clone(&self), conn));
    }
    async fn run(self: Arc<Self>, conn: Arc<UserConnection>) {
        // These writers should never drop
        let mut events = self.event_rx.write().await;
        let mut messages = self.message_rx.write().await;
        loop {
            let result = tokio::select! {
                e = events.recv() => {
                    if let Some(e) = e {
                        InterfaceResult::Event(e)
                    } else {
                        InterfaceResult::ChannelClose
                    }
                },
                m = messages.recv() => {
                    match m {
                        Ok(m) => InterfaceResult::Message(m),
                        Err(RecvError::Lagged(u)) => InterfaceResult::MessagesLagged(u),
                        Err(RecvError::Closed) => InterfaceResult::ChannelClose,
                    }
                }
            };

            match result {
                InterfaceResult::ChannelClose => (), // TODO: Something here
                InterfaceResult::Event(event) => {
                    conn.send(
                        GameEvent::Event {
                            code: self.code.clone(),
                            event,
                        }
                        .into(),
                    )
                    .await;
                }
                InterfaceResult::Message(msg) => {
                    conn.send(
                        GameEvent::Message {
                            code: self.code.clone(),
                            msg,
                        }
                        .into(),
                    )
                    .await;
                }
                InterfaceResult::MessagesLagged(count) => {
                    conn.send(
                        GameEvent::MessagesLagged {
                            code: self.code.clone(),
                            count,
                        }
                        .into(),
                    )
                    .await;
                }
            }
        }
    }
}

enum InterfaceResult {
    Event(Event),
    Message(ApprovedChatMessage),
    MessagesLagged(u64),
    ChannelClose,
}

#[derive(Debug)]
pub enum InterfaceError {
    NotYourTurn,
    InvalidMove,
    UnknownError,
}

impl Display for InterfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidMove => "InvalidMove: Attempted move was not valid",
                Self::NotYourTurn => "NotYourTurn: It is not this player's turn",
                Self::UnknownError => "UnknownError",
            }
        )
    }
}

impl Error for InterfaceError {}

impl From<oneshot::error::RecvError> for InterfaceError {
    fn from(_value: oneshot::error::RecvError) -> Self {
        Self::UnknownError
    }
}

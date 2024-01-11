use std::{error::Error, fmt::Display};

use crate::chess::game::{
    network::{Action, ApprovedChatMessage, ChatMessage, Event},
    pieces::Move,
};
use tokio::sync::{broadcast, mpsc, oneshot, watch};

type MoveRx = mpsc::Receiver<oneshot::Sender<(Move, oneshot::Sender<bool>)>>;

#[derive(Debug)]
pub struct GameInterface {
    move_target: MoveRx,
    action_target: watch::Receiver<Option<mpsc::Sender<Action>>>,

    event_rx: mpsc::Receiver<Event>,

    message_target: mpsc::Sender<ChatMessage>,
    message_rx: broadcast::Receiver<ApprovedChatMessage>,
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
        message_target: mpsc::Sender<ChatMessage>,
        message_rx: broadcast::Receiver<ApprovedChatMessage>,
    ) -> Self {
        Self {
            move_target,
            action_target,
            event_rx,
            message_target,
            message_rx,
        }
    }
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

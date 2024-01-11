use anyhow::Result;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicU64, Arc};
use tokio::sync::{oneshot::Sender, RwLock};
use warp::{
    filters::ws::{Message, WebSocket},
    Error,
};

use crate::chess::game::{
    network::{ActionType, ApprovedChatMessage},
    pieces::Move,
    GameConfig,
};

static ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct Connection {
    pub stream: RwLock<SplitStream<WebSocket>>,
    sink: RwLock<SplitSink<WebSocket, Message>>,
    closer: Sender<()>,
    pub id: u64,
}

impl Connection {
    pub fn new(websocket: WebSocket, closer: Sender<()>) -> Self {
        let (sink, stream) = websocket.split();
        Self {
            stream: RwLock::new(stream),
            sink: RwLock::new(sink),
            closer,
            id: ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }

    pub async fn close(self) -> Result<(), ()> {
        let mut writer = self.sink.write().await;
        match (*writer).close().await {
            Result::Err(_) => return Err(()),
            Result::Ok(_) => (),
        };
        self.closer.send(())
    }

    pub async fn listen(self: Arc<Self>) -> ListenerResult {
        let mut writer = self.stream.write().await;
        let msg = (*writer).next().await;
        drop(writer);
        match msg {
            None => ListenerResult::Disconnected(self.id),
            Some(Err(e)) => ListenerResult::Error(e),
            Some(Ok(m)) => {
                if m.is_text() {
                    ListenerResult::Text(m.to_str().unwrap().to_string())
                } else {
                    ListenerResult::Ignore
                }
            }
        }
    }

    pub async fn send_serde(&self, msg: impl Serialize) {
        let text = match serde_json::to_string(&msg) {
            Ok(s) => s,
            Err(e) => {
                eprint!("\rFailed to serialize message because of error: {e}\n > ");
                return;
            }
        };
        let mut writer = self.sink.write().await;
        match writer.send(Message::text(text)).await {
            Ok(_) => (),
            Err(e) => eprint!("\rFailed to send message because of error: {e}\n > "),
        };
    }

    pub async fn send(&self, msg: &String) {
        let mut writer = self.sink.write().await;
        match writer.send(Message::text(msg)).await {
            Ok(_) => (),
            Err(e) => eprint!("\rFailed to send message because of error: {e}\n > "),
        }
    }
}

pub enum ListenerResult {
    Text(String),
    Error(Error),
    Disconnected(u64),
    Ignore,
}

#[derive(Serialize)]
pub enum SentMessage {
    // * Status
    WsError { context: String },
    WsConnected { display: String },

    // * Control Events
    WsEvent { event: ControlEvent },

    // * In game
    WsGameEvent { event: GameEvent },
}

impl SentMessage {
    pub fn error(context: impl ToString) -> Self {
        Self::WsError {
            context: context.to_string(),
        }
    }
}

impl From<ControlEvent> for SentMessage {
    fn from(event: ControlEvent) -> Self {
        Self::WsEvent { event }
    }
}

impl From<GameEvent> for SentMessage {
    fn from(event: GameEvent) -> Self {
        Self::WsGameEvent { event }
    }
}

#[derive(Serialize)]
pub enum GameEvent {
    GameStart { code: String },
    Message { msg: ApprovedChatMessage },
}

#[derive(Serialize)]
pub enum ControlEvent {
    // * Host responses
    LobbyCreated { code: String },
    LobbyClosed { code: String },

    // * Client responses
    JoinedLobby { code: String },
    LeftLobby { code: String },

    // * Lobby Starting
    LobbyStarted { code: String },

    // * Matchmaking responses
    JoinedQueue,
    LeftQueue,

    // * Spectators
    JoinedAsSpectator { code: String },
}

#[derive(Deserialize)]
pub enum RecievedMessage {
    // * Ingame controls
    GameAction { action: GameAction },

    // * Out of game controls
    ControlAction { action: ControlAction },
}

#[derive(Deserialize)]
pub enum ControlAction {
    // * Host controls
    CreateLobby,
    CloseLobby { code: String },
    StartLobby { config: GameConfig },

    // * Client controls
    JoinLobby { code: String },
    LeaveLobby { code: String },

    // * Matchmaking controls
    JoinQueue,
    LeaveQueue,

    // * Spectators
    JoinAsSpectator { code: String },
}

#[derive(Deserialize)]
pub enum GameAction {
    Message { msg: String },
    Turn { turn: Move },
    Action { action: ActionType },
}

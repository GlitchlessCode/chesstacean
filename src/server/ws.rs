use anyhow::Result;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::Serialize;
use std::sync::{atomic::AtomicU64, Arc};
use tokio::sync::{oneshot::Sender, RwLock};
use warp::{
    filters::ws::{Message, WebSocket},
    Error,
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
            Some(Ok(m)) => ListenerResult::Message(m),
        }
    }

    pub async fn send(&self, msg: impl Serialize) {
        let text = match serde_json::to_string(&msg) {
            Ok(s) => s,
            Err(e) => {
                eprint!("\rFailed to serialize message because of error: {e}\n > ");
                return;
            }
        };
        let mut writer = self.sink.write().await;
        match (*writer).send(Message::text(text)).await {
            Ok(_) => (),
            Err(e) => eprint!("\rFailed to send message because of error: {e}\n > "),
        };
    }
}

pub enum ListenerResult {
    Message(Message),
    Error(Error),
    Disconnected(u64),
}

#[derive(Serialize)]
pub enum ChessMessage {
    Error { context: String },
}

impl ChessMessage {
    pub fn error(context: impl ToString) -> Self {
        Self::Error {
            context: context.to_string(),
        }
    }
}

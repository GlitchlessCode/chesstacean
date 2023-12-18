use anyhow::{Ok, Result};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::Serialize;
use tokio::sync::oneshot::Sender;
use warp::filters::ws::{Message, WebSocket};

#[derive(Debug)]
pub struct Connection {
    pub stream: SplitStream<WebSocket>,
    sink: SplitSink<WebSocket, Message>,
    closer: Sender<()>,
}

impl Connection {
    pub fn new(websocket: WebSocket, closer: Sender<()>) -> Self {
        let (sink, stream) = websocket.split();
        Self { stream, sink, closer }
    }

    pub async fn close(mut self) -> Result<(), ()> {
        match self.sink.close().await {
            Result::Err(_) => return Err(()),
            Result::Ok(_) => (),
        };
        self.closer.send(())
    }

    pub async fn send(&mut self, msg: impl Serialize) -> Result<()> {
        let text = serde_json::to_string(&msg)?;
        self.sink.send(Message::text(text)).await?;
        Ok(())
    }
}

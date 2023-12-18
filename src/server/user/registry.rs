use super::*;
use crate::server::ws::Connection;
use futures_util::StreamExt;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc::Receiver, RwLock};

pub struct Registry {
    pub users: Arc<RwLock<HashMap<u64, Connection>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start(self, mut ws_rx: Receiver<Connection>) -> ! {
        while let Some(conn) = ws_rx.recv().await {
            tokio::task::spawn(auth(conn, self.users.clone()));
        }
        panic!("ws_rx mspc channel was closed: this channel should never close");
    }
}

async fn auth(mut conn: Connection, users: Arc<RwLock<HashMap<u64, Connection>>>) {
    let thing = conn.stream.next().await;
    println!("{thing:?}");
    if let Some(msg) = thing {
        let msg = msg.unwrap();
        let num: u64 = msg.to_str().unwrap().parse().unwrap();
        let mut locked = users.write().await;
        (*locked).insert(num, conn);
    } else {
        conn.close().await.unwrap();
    }
}

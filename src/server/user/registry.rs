use super::*;
use crate::{
    server::{database::DatabaseMessage, tokens::TokenManager, ws::SentMessage},
    word_loader::WordList,
};
use futures_util::StreamExt;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Registry {
    pub users: RwLock<HashMap<String, UserConnection>>,
    active_sessions: RwLock<HashSet<String>>,

    controller: Arc<GameControllerInterface>,
}

impl Registry {
    pub async fn new(word_list: WordList, db_tx: &Sender<DatabaseMessage>) -> Arc<Self> {
        Arc::new(Self {
            users: RwLock::new(HashMap::new()),
            active_sessions: RwLock::new(HashSet::new()),
            controller: GameControllerInterface::new(word_list, db_tx.clone()).await,
        })
    }

    pub async fn start(self: Arc<Self>, mut ws_rx: Receiver<Connection>, token_man: Arc<TokenManager>) -> ! {
        while let Some(conn) = ws_rx.recv().await {
            tokio::task::spawn(Arc::clone(&self).begin_connection(conn, token_man.clone()));
        }
        panic!("ws_rx mspc channel was closed: this channel should never close");
    }

    pub async fn end_session(&self, session: String) {
        let mut session_writer = self.active_sessions.write().await;
        session_writer.remove(&session);
        drop(session_writer);

        let user_reader = self.users.read().await;
        'inner: for user in user_reader.values() {
            if user.end_session(&session).await {
                break 'inner;
            }
        }
    }

    pub async fn get_session(&self, session: &String) -> Option<UserInfo> {
        let session_reader = self.active_sessions.read().await;
        if session_reader.get(session).is_some() {
            drop(session_reader);

            let user_reader = self.users.read().await;
            for user in user_reader.values() {
                if user.has_session(session).await {
                    let info_reader = user.info.read().await;
                    return Some(info_reader.clone());
                }
            }
        }

        None
    }

    async fn begin_connection(self: Arc<Self>, conn: Connection, token_man: Arc<TokenManager>) {
        let mut stream_writer = conn.stream.write().await;
        let first_msg = (*stream_writer).next().await;
        drop(stream_writer);

        if let Some(msg) = first_msg {
            if msg.is_err() {
                conn.close().await.unwrap();
                return;
            }

            if let Ok(string) = msg.unwrap().to_str() {
                let parse = token_man.parse_ws_token(string.to_string());
                eprint!("\r{parse:?}\n\n > ");

                if let Ok(parse) = parse {
                    if !parse.valid() {
                        conn.send_serde(SentMessage::error("Token expired")).await;
                        return;
                    }

                    let mut session_writer = self.active_sessions.write().await;
                    session_writer.insert(parse.sub.clone());
                    drop(session_writer);

                    let mut user_writer = self.users.write().await;
                    let key = match parse.us.get_handle() {
                        Some(handle) => handle,
                        None => format!("&{}", parse.us.get_display()),
                    };

                    if let Some(user_conn) = user_writer.get(&key) {
                        conn.send_serde(SentMessage::WsConnected {
                            display: parse.us.get_display(),
                        })
                        .await;
                        user_conn.add_connection(conn, parse.sub).await;
                    } else {
                        conn.send_serde(SentMessage::WsConnected {
                            display: parse.us.get_display(),
                        })
                        .await;
                        let user_conn = UserConnection::from((parse.us, self.controller.clone()));
                        user_conn.add_connection(conn, parse.sub).await;
                        user_writer.insert(key, user_conn);
                    }
                } else {
                    conn.send_serde(SentMessage::error("Unauthorized to connect")).await;
                }
            }
        } else {
            conn.close().await.unwrap();
        }
    }
}

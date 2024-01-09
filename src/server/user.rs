use super::{
    utils::{ArcLock, ArcLockTrait},
    ws,
};
use crate::{chess::controller::GameControllerInterface, server::ws::Connection};
use anyhow::Result;
use interface::GameInterface;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};
use tokio::{
    sync::{mpsc, RwLock},
    task::JoinSet,
};

pub mod interface;
pub mod registry;

pub struct UserConnection {
    pub info: RwLock<UserInfo>,
    connections: ArcLock<HashMap<String, SessionConnections>>,
    listener: Arc<ConnectionListener>,
}

impl UserConnection {
    /// To be used when a guest signs up, and a handle and display name are assigned
    ///
    /// Returns `Err(())` if this `UserConnection` is already a User
    ///
    /// Returns `Ok(())` if this `UserConnection` is a Guest
    pub async fn upgrade(&self, handle: impl ToString, display: impl ToString) -> Result<(), ()> {
        let read_info = self.info.read().await;
        if let UserInfo::Guest { .. } = *read_info {
            let mut write_info = self.info.write().await;
            *write_info = UserInfo::new_user(handle, display);
            Ok(())
        } else {
            Err(())
        }
    }
    pub async fn add_connection(&self, conn: Connection, session: String) {
        let mut writer = self.connections.write().await;
        if let Some(session_connection) = writer.get_mut(&session) {
            session_connection.add_connection(conn);
        } else {
            let mut session_connection = SessionConnections::new();
            session_connection.add_connection(conn);
            writer.insert(session, session_connection);
        }
        drop(writer);
        self.listener.interrupt().await.unwrap();
    }
    pub async fn end_session(&self, session: &String) -> bool {
        let mut writer = self.connections.write().await;
        if let Some(conn) = writer.remove(session) {
            conn.close().await;
            self.listener.interrupt().await.unwrap();
            true
        } else {
            false
        }
    }
    pub async fn has_session(&self, session: &String) -> bool {
        let reader = self.connections.read().await;
        reader.contains_key(session)
    }
    pub async fn send(&self) {}
}

impl From<(UserInfo, Arc<GameControllerInterface>)> for UserConnection {
    fn from((value, controller): (UserInfo, Arc<GameControllerInterface>)) -> Self {
        let (tx, rx) = mpsc::channel(2);
        let connections = ArcLock::new_arclock(HashMap::new());
        let listener = Arc::new(ConnectionListener::new(Arc::clone(&connections), tx, controller));
        let this = Self {
            info: RwLock::new(value),
            connections,
            listener: Arc::clone(&listener),
        };
        tokio::task::spawn(listener.listen(rx));
        this
    }
}

struct ConnectionListener {
    connections: ArcLock<HashMap<String, SessionConnections>>,
    targets: RwLock<HashMap<String, GameInterface>>,
    controller: Arc<GameControllerInterface>,
    interrupt: mpsc::Sender<()>,
}

impl ConnectionListener {
    async fn listen(self: Arc<Self>, mut interrupt: mpsc::Receiver<()>) {
        loop {
            let mut futures = JoinSet::new();

            let hash_map = self.connections.read().await;

            for (_, session) in (*hash_map).iter() {
                session.listen(&mut futures)
            }

            drop(hash_map);

            let result = tokio::select! {
                Some(m) = futures.join_next() => {
                    futures.abort_all();
                    ListenerResult::Result(m.unwrap())
                },
                Some(_) = interrupt.recv() => ListenerResult::Interrupted,
            };

            if let ListenerResult::Result(r) = result {
                match r {
                    ws::ListenerResult::Disconnected(id) => {
                        eprint!("\rUser {} has disconnected\n > ", id);
                        let mut hash_map = self.connections.write().await;
                        'inner: for (_, session) in (*hash_map).iter_mut() {
                            if session.close_connection(id).await {
                                break 'inner;
                            }
                        }
                        drop(hash_map)
                    }
                    ws::ListenerResult::Error(err) => {
                        eprint!("\rNew Error: {err}\n > ");
                    }
                    ws::ListenerResult::Message(msg) => {
                        eprint!("\rNew Message: {msg:?}\n > ");
                    }
                    ws::ListenerResult::Ignore => (),
                }
            }

            // futures.join_next().await;

            // drop(futures);
        }
    }

    fn new(
        connections: ArcLock<HashMap<String, SessionConnections>>,
        interrupt: mpsc::Sender<()>,
        controller: Arc<GameControllerInterface>,
    ) -> Self {
        Self {
            connections,
            targets: RwLock::new(HashMap::new()),
            interrupt,
            controller,
        }
    }

    async fn interrupt(&self) -> Result<()> {
        self.interrupt.send(()).await?;
        Ok(())
    }
}

enum ListenerResult {
    Interrupted,
    Result(ws::ListenerResult),
}

struct SessionConnections {
    connections: Vec<Arc<Connection>>,
}

impl SessionConnections {
    fn new() -> Self {
        Self {
            connections: Vec::new(),
        }
    }
    fn add_connection(&mut self, conn: Connection) {
        self.connections.push(Arc::new(conn))
    }
    fn listen(&self, joins_set: &mut JoinSet<ws::ListenerResult>) {
        for conn in self.connections.iter() {
            joins_set.spawn(conn.clone().listen());
        }
    }
    async fn close_connection(&mut self, id: u64) -> bool {
        for (idx, conn) in self.connections.iter().enumerate() {
            if conn.id == id {
                let conn = self.connections.swap_remove(idx);
                if let Some(conn) = Arc::into_inner(conn) {
                    conn.close()
                        .await
                        .unwrap_or_else(|_| eprint!("\rCould not close connection, encountered error\n > "));
                }
                return true;
            }
        }
        false
    }
    async fn close(self) {
        for conn in self.connections.into_iter() {
            if let Some(conn) = Arc::into_inner(conn) {
                conn.close()
                    .await
                    .unwrap_or_else(|_| eprint!("\rCould not close connection, encountered error\n > "));
            }
        }
    }
}

static GUEST_COUNT: AtomicU32 = AtomicU32::new(1);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UserInfo {
    User { handle: String, display: String },
    Guest { guest_num: u32 },
}

impl UserInfo {
    pub fn new_user(handle: impl ToString, display: impl ToString) -> Self {
        Self::User {
            handle: handle.to_string(),
            display: display.to_string(),
        }
    }
    pub fn new_guest() -> Self {
        Self::Guest {
            guest_num: GUEST_COUNT.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn get_handle(&self) -> Option<String> {
        match self {
            Self::Guest { .. } => None,
            Self::User { handle, .. } => Some(handle.clone()),
        }
    }
    pub fn get_display(&self) -> String {
        match self {
            Self::Guest { guest_num } => format!("Guest{guest_num}"),
            Self::User { display, .. } => display.clone(),
        }
    }
}

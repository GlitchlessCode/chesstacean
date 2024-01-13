use super::{
    utils::{ArcLock, ArcLockTrait},
    ws,
};
use crate::{
    chess::controller::GameControllerInterface,
    server::ws::{Connection, ControlEvent, RecievedMessage, SentMessage},
};
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

#[allow(async_fn_in_trait)]
pub trait Sender {
    async fn connections(&self) -> tokio::sync::RwLockReadGuard<'_, HashMap<String, SessionConnections>>;

    /// ### Serializes the message, and sends it to all connected sessions
    async fn send(&self, msg: SentMessage) {
        let msg = match serde_json::to_string(&msg) {
            Ok(msg) => msg,
            Err(e) => {
                return eprint!("\rCould not serialize message when sending to sockets with error: {e}\n\n > ");
            }
        };
        eprint!("\rSending Message: {msg}\n\n > ");
        for conn in self.connections().await.values() {
            conn.send(msg.clone()).await;
        }
    }
}

pub struct ConnectionExtension {
    connections: ArcLock<HashMap<String, SessionConnections>>,
}

impl Sender for ConnectionExtension {
    async fn connections(&self) -> tokio::sync::RwLockReadGuard<'_, HashMap<String, SessionConnections>> {
        self.connections.read().await
    }
}

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
            let user = UserInfo::new_user(handle, display);

            // Update Connection
            let mut write_info = self.info.write().await;
            *write_info = user.clone();
            drop(write_info);

            // Update Listener
            self.listener.upgrade(user).await;

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
}

impl Sender for UserConnection {
    async fn connections(&self) -> tokio::sync::RwLockReadGuard<HashMap<String, SessionConnections>> {
        self.connections.read().await
    }
}

impl From<(UserInfo, Arc<GameControllerInterface>)> for UserConnection {
    fn from((value, controller): (UserInfo, Arc<GameControllerInterface>)) -> Self {
        let (tx, rx) = mpsc::channel(2);
        let connections = ArcLock::new_arclock(HashMap::new());
        let listener = Arc::new(ConnectionListener::new(
            Arc::clone(&connections),
            tx,
            controller,
            &value.clone(),
        ));
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
    targets: Targets,
    controller: Arc<GameControllerInterface>,
    interrupt: mpsc::Sender<()>,

    info: RwLock<UserInfo>,
}

impl ConnectionListener {
    async fn upgrade(&self, user: UserInfo) {
        let mut write_info = self.info.write().await;
        let original = write_info.clone();
        *write_info = user.clone();
        drop(write_info);

        self.controller.upgrade(original, user.clone()).await;
        self.targets.upgrade(user).await;
    }

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
                        let mut hash_map = self.connections.write().await;
                        'inner: for (_, session) in (*hash_map).iter_mut() {
                            if session.close_connection(id).await {
                                break 'inner;
                            }
                        }
                        drop(hash_map)
                    }
                    ws::ListenerResult::Error(err) => {
                        eprint!("\rNew Error: {err}\n\n > ");
                        self.send(SentMessage::error(
                            "The server experienced an error handling your request",
                        ))
                        .await;
                    }
                    ws::ListenerResult::Text(msg) => {
                        eprint!("\rNew Message: {msg:?}\n\n > ");
                        let result = serde_json::from_str::<'_, RecievedMessage>(&msg);
                        match result {
                            Err(err) => {
                                eprint!("\rError deserializing request: {err}\n\n > ");
                                self.send(SentMessage::error(
                                    "The server experienced an error handling your request",
                                ))
                                .await
                            }
                            Ok(action) => {
                                tokio::task::spawn(ConnectionListener::handle_message(self.clone(), action));
                            }
                        }
                    }
                    ws::ListenerResult::Ignore => (),
                }
            }
        }
    }

    fn new(
        connections: ArcLock<HashMap<String, SessionConnections>>,
        interrupt: mpsc::Sender<()>,
        controller: Arc<GameControllerInterface>,
        info: &UserInfo,
    ) -> Self {
        Self {
            connections,
            targets: Targets::new(),
            interrupt,
            controller,
            info: RwLock::new(info.clone()),
        }
    }

    async fn interrupt(&self) -> Result<()> {
        self.interrupt.send(()).await?;
        Ok(())
    }

    async fn handle_message(self: Arc<Self>, action: RecievedMessage) {
        match action {
            RecievedMessage::ControlAction { action } => {
                use ws::ControlAction::*;
                let controller = &self.controller;
                let reader = &self.info.read().await;
                match action {
                    CreateLobby => {
                        let code = controller.create_lobby(&reader, (&self.connections).into()).await;
                        match code {
                            Ok(code) => self.send(ControlEvent::LobbyCreated { code }.into()).await,
                            Err(e) => {
                                eprint!("\rExperienced an error creating a lobby: {e:?}\n\n > ");
                                self.send(SentMessage::error(e)).await;
                            }
                        }
                    }
                    CloseLobby { code } => {
                        if let Err(e) = controller.close_lobby(&reader, code).await {
                            self.send(SentMessage::error(e)).await;
                        }
                    }
                    StartLobby { config } => (),

                    JoinLobby { code } => {
                        let result = controller.join_lobby(&code, &reader, (&self.connections).into()).await;
                        if let Err(e) = result {
                            self.send(SentMessage::error(e)).await;
                        } else {
                            self.send(ControlEvent::JoinedLobby { code }.into()).await;
                        }
                    }
                    LeaveLobby { code } => {
                        let result = controller.leave_lobby(&code, &reader).await;
                        if let Err(e) = result {
                            self.send(SentMessage::error(e)).await;
                        } else {
                            self.send(ControlEvent::LeftLobby { code }.into()).await;
                        }
                    }

                    JoinQueue => (),
                    LeaveQueue => (),

                    JoinAsSpectator { code } => (),
                }
            }
            RecievedMessage::GameAction { action } => {
                use ws::GameAction::*;
                match action {
                    Turn { .. } => {}
                    _ => (),
                }
            }
        }
    }
}

impl Sender for ConnectionListener {
    async fn connections(&self) -> tokio::sync::RwLockReadGuard<'_, HashMap<String, SessionConnections>> {
        self.connections.read().await
    }
}

impl From<&ArcLock<HashMap<String, SessionConnections>>> for ConnectionExtension {
    fn from(value: &ArcLock<HashMap<String, SessionConnections>>) -> Self {
        Self {
            connections: value.clone(),
        }
    }
}

struct Targets {
    inner: RwLock<HashMap<String, GameInterface>>,
}

impl Targets {
    fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    async fn upgrade(&self, user: UserInfo) {}
}

enum ListenerResult {
    Interrupted,
    Result(ws::ListenerResult),
}

pub struct SessionConnections {
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
                        .unwrap_or_else(|_| eprint!("\rCould not close connection, encountered error\n\n > "));
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
                    .unwrap_or_else(|_| eprint!("\rCould not close connection, encountered error\n\n > "));
            }
        }
    }
    async fn send(&self, msg: String) {
        for conn in self.connections.iter() {
            conn.send(&msg).await;
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

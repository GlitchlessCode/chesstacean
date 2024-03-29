use super::game::{
    network::{ActionInterface, PlayerInterface},
    GameConfig, InactiveGame,
};
use crate::{
    server::{
        database::{Database, DatabaseMessage, DatabaseResult},
        user::{interface::GameInterface, ConnectionExtension, Sender, UserInfo},
        utils::{ArcLock, ArcLockTrait},
        ws::ControlEvent,
    },
    traits::ChooseTake,
    word_loader::WordList,
};
use anyhow::{bail, Result};
use rand::rngs::OsRng;
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Display,
    sync::{Arc, Weak},
    time::Duration,
};
use tokio::{
    sync::{mpsc, oneshot, RwLock},
    time::{self, Instant},
};

pub struct GameControllerInterface {
    matchmaker: Arc<Matchmaker>,
    lobby_manager: RwLock<LobbyManager>,
    active_game_codes: RwLock<Vec<String>>,

    word_list: Arc<WordList>,

    db_tx: mpsc::Sender<DatabaseMessage>,
}
// TODO: Convert a lot of the UserInfo parts to include Targets for transmission
impl GameControllerInterface {
    pub async fn new(word_list: WordList, db_tx: mpsc::Sender<DatabaseMessage>) -> Arc<Self> {
        let matchmaker = Matchmaker::new();
        Matchmaker::start(matchmaker.clone());

        let lobby_manager = RwLock::new(LobbyManager::new());

        let this = Arc::new(Self {
            matchmaker: matchmaker.clone(),
            lobby_manager,

            active_game_codes: RwLock::new(vec![]),

            word_list: Arc::new(word_list),

            db_tx,
        });

        let mut writer = matchmaker.controller_ref.write().await;
        *writer = Arc::downgrade(&this);

        this
    }

    pub async fn join_queue(&self, user: &UserInfo) -> Result<oneshot::Receiver<Option<Arc<GameInterface>>>, ()> {
        self.matchmaker.join_queue(user).await
    }
    pub async fn leave_queue(&self, user: &UserInfo) {
        self.matchmaker.leave_queue(user).await
    }

    pub async fn create_lobby(&self, user: &UserInfo, user_conn: ConnectionExtension) -> Result<String> {
        let code = self.create_new_game().await?;

        let mut lobbies = self.lobby_manager.write().await;
        lobbies.create_lobby(&code, &user, user_conn);

        Ok(code)
    }
    pub async fn close_lobby(&self, user: &UserInfo, code: String) -> Result<()> {
        let mut lobbies = self.lobby_manager.write().await;
        lobbies.close_lobby(user, code).await
    }
    pub async fn start_lobby(&self) {}

    pub async fn join_lobby(&self, code: &String, user: &UserInfo, user_conn: ConnectionExtension) -> Result<()> {
        self.lobby_manager.write().await.join_lobby(code, user, user_conn).await
    }
    pub async fn leave_lobby(&self, code: &String, user: &UserInfo) -> Result<()> {
        self.lobby_manager.read().await.leave_lobby(code, user).await
    }

    async fn create_new_game(&self) -> Result<String> {
        let id = loop {
            let word = self.word_list.combo(&mut OsRng);

            let result = {
                let word = word.clone();
                let func = move |db: &Database| DatabaseResult::from(db.games().game_exists(word));
                DatabaseMessage::send(func, &self.db_tx).await?
            };

            let already_exists = match result {
                DatabaseResult::ResultBool(rb) => rb?,
                _ => bail!(ControllerError::InternalError),
            };

            if !already_exists {
                break word;
            }
        };

        let mut writer = self.active_game_codes.write().await;
        writer.push(id.clone());
        drop(writer);

        Ok(id)
    }

    pub async fn upgrade(&self, original: UserInfo, user: UserInfo) {
        self.matchmaker.upgrade(original.clone(), user.clone()).await;
        self.lobby_manager.write().await.upgrade(original, user).await;
    }
}

#[derive(Debug)]
pub enum ControllerError {
    InternalError,

    NoSuchLobby,
    NotLobbyHost,
    IsAlreadyHost,
    FullLobby,
    NotInLobby,
}

impl Display for ControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InternalError => format!("InternalError: Ran into an unknown internal error"),
                Self::NoSuchLobby => format!("NoSuchLobby: The requested lobby does not exist"),
                Self::NotLobbyHost => format!("NotLobbyHost: You do not own this lobby"),
                Self::IsAlreadyHost => format!("IsAlreadyHost: You cannot join a game with yourself"),
                Self::FullLobby => format!("FullLobby: This lobby is already full"),
                Self::NotInLobby => format!("NotInLobby: You are not in this lobby"),
            }
        )
    }
}

impl Error for ControllerError {}

struct LobbyManager {
    codes: BTreeMap<String, ArcLock<Lobby>>,
    users: BTreeMap<UserInfo, Vec<ArcLock<Lobby>>>,
}

impl LobbyManager {
    fn new() -> Self {
        Self {
            codes: BTreeMap::new(),
            users: BTreeMap::new(),
        }
    }

    async fn upgrade(&mut self, original: UserInfo, user: UserInfo) {}

    fn create_lobby(&mut self, code: &String, user: &UserInfo, user_conn: ConnectionExtension) {
        let lobby = Lobby::new(code.clone(), user.clone(), user_conn);
        self.codes.insert(code.clone(), Arc::clone(&lobby));

        match self.users.get_mut(user) {
            None => {
                self.users.insert(user.clone(), vec![lobby]);
            }
            Some(vec) => {
                vec.push(lobby);
            }
        }
    }
    async fn close_lobby(&mut self, user: &UserInfo, code: String) -> Result<()> {
        let lobby = match self.codes.get(&code) {
            Some(l) => l,
            None => bail!(ControllerError::NoSuchLobby),
        };
        if lobby.read().await.is_host(user) {
            match self.users.get_mut(user) {
                None => bail!(ControllerError::NoSuchLobby),
                Some(vec) => {
                    let lobby = match self.codes.remove(&code) {
                        None => bail!(ControllerError::NoSuchLobby),
                        Some(l) => l,
                    };
                    let mut vec_iter = vec.iter().enumerate();
                    loop {
                        if let Some((i, l)) = vec_iter.next() {
                            if *(l.read().await.code) == *(lobby.read().await.code) {
                                vec.remove(i);
                                lobby.read().await.alert_close().await;
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(())
        } else {
            bail!(ControllerError::NotLobbyHost)
        }
    }
    fn start_lobby(&mut self, user: &UserInfo, config: GameConfig) {
        // TODO
    }

    async fn join_lobby(&mut self, code: &String, user: &UserInfo, user_conn: ConnectionExtension) -> Result<()> {
        let lobby = match self.codes.get(code) {
            Some(l) => l,
            None => bail!(ControllerError::NoSuchLobby),
        };
        let mut writer = lobby.write().await;
        if writer.is_host(user) {
            bail!(ControllerError::IsAlreadyHost)
        } else {
            writer.join(user.clone(), user_conn)?;
            Ok(())
        }
    }
    async fn leave_lobby(&self, code: &String, user: &UserInfo) -> Result<()> {
        let lobby = match self.codes.get(code) {
            Some(l) => l,
            None => bail!(ControllerError::NoSuchLobby),
        };
        lobby.write().await.leave(user)
    }
}

struct Lobby {
    host: LobbyUser,
    client: Option<LobbyUser>,
    code: String,
}

impl Lobby {
    fn new(code: String, host_info: UserInfo, host: ConnectionExtension) -> ArcLock<Self> {
        ArcLock::new_arclock(Self {
            code,
            host: LobbyUser {
                info: host_info,
                conn: host,
            },
            client: None,
        })
    }

    fn is_host(&self, user: &UserInfo) -> bool {
        &self.host.info == user
    }

    fn is_client(&self, user: &UserInfo) -> bool {
        if self.client.is_some() {
            &self.client.as_ref().unwrap().info == user
        } else {
            false
        }
    }

    fn join(&mut self, client_info: UserInfo, client: ConnectionExtension) -> Result<()> {
        if self.client.is_some() {
            bail!(ControllerError::FullLobby)
        }
        self.client = Some(LobbyUser {
            info: client_info,
            conn: client,
        });
        Ok(())
    }

    fn leave(&mut self, client_info: &UserInfo) -> Result<()> {
        if self.is_client(client_info) {
            self.client = None;
            Ok(())
        } else {
            bail!(ControllerError::NotInLobby)
        }
    }

    async fn alert_close(&self) {
        self.host
            .conn
            .send(
                ControlEvent::LobbyClosed {
                    code: self.code.clone(),
                }
                .into(),
            )
            .await;

        if self.client.is_some() {
            self.client
                .as_ref()
                .unwrap()
                .conn
                .send(
                    ControlEvent::LobbyClosed {
                        code: self.code.clone(),
                    }
                    .into(),
                )
                .await;
        }
    }
}

struct LobbyUser {
    info: UserInfo,
    conn: ConnectionExtension,
}

struct Matchmaker {
    queue: RwLock<Vec<UserInQueue>>,
    in_queue: RwLock<BTreeSet<UserInfo>>,

    controller_ref: RwLock<Weak<GameControllerInterface>>,
}

impl Matchmaker {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: RwLock::new(vec![]),
            in_queue: RwLock::new(BTreeSet::new()),
            controller_ref: RwLock::new(Weak::new()),
        })
    }

    async fn upgrade(&self, original: UserInfo, user: UserInfo) {}

    async fn join_queue(&self, user: &UserInfo) -> Result<oneshot::Receiver<Option<Arc<GameInterface>>>, ()> {
        let mut in_queue = self.in_queue.write().await;
        let mut queue = self.queue.write().await;

        if in_queue.insert(user.clone()) {
            let (uiq, rx) = UserInQueue::create(user);

            queue.push(uiq);

            Ok(rx)
        } else {
            Err(())
        }
    }

    async fn leave_queue(&self, user: &UserInfo) {
        let mut in_queue = self.in_queue.write().await;
        let mut queue = self.queue.write().await;

        if in_queue.remove(user) {
            let index = queue.iter().position(|u| &u.user == user).unwrap();
            let uiq = queue.remove(index);
            uiq.reply(None)
        }
    }

    fn start(self: Arc<Self>) {
        tokio::task::spawn(self.run());
    }

    /// Start looping
    async fn run(self: Arc<Self>) {
        let mut interval = time::interval(Duration::from_secs(2));
        loop {
            interval.tick().await;

            let mut queue = self.queue.write().await;
            let mut in_queue = self.in_queue.write().await;
            let controller = self
                .controller_ref
                .read()
                .await
                .upgrade()
                .expect("This should never be None");
            Matchmaker::make_matches(&mut queue, &mut in_queue, &controller).await;
            drop(queue);
            drop(in_queue);
            drop(controller);
        }
    }

    /// ### Matchmaking algorithm
    ///
    /// Currently does not implement rating
    ///
    /// Currently does not implement timestamp
    ///
    /// Currently does not implement any statistic analysis
    async fn make_matches(
        queue: &mut Vec<UserInQueue>,
        in_queue: &mut BTreeSet<UserInfo>,
        controller: &Arc<GameControllerInterface>,
    ) {
        'while_loop: while queue.len() > 2 {
            // Get random players
            let player1 = queue.take_random(&mut OsRng).unwrap();
            let player2 = queue.take_random(&mut OsRng).unwrap();

            let game_code = controller.create_new_game().await;

            let game_code = match game_code {
                Ok(gc) => gc,
                Err(e) => {
                    eprint!("\rRan into error ({e:?}) creating game code during matchmaking\n\n > ");
                    player1.reply(None);
                    player2.reply(None);
                    continue 'while_loop;
                }
            };

            in_queue.remove(&player1.user);
            in_queue.remove(&player2.user);

            // Create player interfaces
            let (player1_interface, p1_move_rx, p1_event_rx) = PlayerInterface::create(player1.user.clone());
            let (player2_interface, p2_move_rx, p2_event_rx) = PlayerInterface::create(player2.user.clone());

            // Create actions interface
            let (actions, action_rx) = ActionInterface::create();

            // Create game
            let game = InactiveGame::new(player1_interface, actions, GameConfig::default());

            // Create game interfaces
            let p1_msg = game.messenger.channel();
            let player1_game_interface = GameInterface::new(
                p1_move_rx,
                action_rx.clone(),
                p1_event_rx,
                game_code.clone(),
                p1_msg.0,
                p1_msg.1,
            );
            player1.reply(Some(player1_game_interface));

            let p2_msg = game.messenger.channel();
            let player2_game_interface =
                GameInterface::new(p2_move_rx, action_rx, p2_event_rx, game_code, p2_msg.0, p2_msg.1);
            player2.reply(Some(player2_game_interface));

            // Start game
            game.start(player2_interface);
        }
    }
}

#[allow(dead_code)]
struct UserInQueue {
    user: UserInfo,
    rating: u16,        // Currently not used
    timestamp: Instant, // Currently not used

    reply_to: oneshot::Sender<Option<Arc<GameInterface>>>,
    // Stats can go here to help with matchmaking, if necessary
    // stats: STUFF
}

impl UserInQueue {
    fn new(user: &UserInfo, reply_to: oneshot::Sender<Option<Arc<GameInterface>>>) -> Self {
        let now = Instant::now();
        Self {
            user: user.clone(),
            timestamp: now, // Currently timestamp is not used, but is included for if we change the matchmaking algo
            rating: 1200, // Currently Rating does not change, and is just a static value applied to every player. Additionally rating is not used for matchmaking currently
            reply_to,
        }
    }
    fn create(user: &UserInfo) -> (Self, oneshot::Receiver<Option<Arc<GameInterface>>>) {
        let (reply_to, reply_rx) = oneshot::channel();
        (Self::new(user, reply_to), reply_rx)
    }
    fn reply(self, interface: Option<Arc<GameInterface>>) {
        self.reply_to.send(interface).unwrap();
    }
}

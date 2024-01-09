use super::game::{network::PlayerInterface, GameConfig, InactiveGame};
use crate::{server::user::UserInfo, traits::ChooseTake};
use rand::{thread_rng, Rng};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{
    sync::RwLock,
    time::{self, Instant},
};

pub struct GameControllerInterface {
    matchmaker: Arc<Matchmaker>,
    lobby_manager: RwLock<LobbyManager>,
}
// TODO: Convert a lot of the UserInfo parts to include Targets for transmission
impl GameControllerInterface {
    pub fn new() -> Self {
        let matchmaker = Matchmaker::new();
        Matchmaker::start(matchmaker.clone());

        let lobby_manager = RwLock::new(LobbyManager::new());

        Self {
            matchmaker,
            lobby_manager,
        }
    }

    pub fn join_queue(&self) {}
    pub fn leave_queue(&self) {}

    pub fn create_lobby(&self) {}
    pub fn close_lobby(&self) {}
    pub fn start_lobby(&self) {}

    pub fn join_lobby(&self) {}
    pub fn leave_lobby(&self) {}
}

struct LobbyManager {
    code_lobbies: HashMap<String, Arc<Lobby>>,
    user_lobbies: HashMap<UserInfo, Arc<Lobby>>,
}

impl LobbyManager {
    fn new() -> Self {
        Self {
            code_lobbies: HashMap::new(),
            user_lobbies: HashMap::new(),
        }
    }

    fn create_lobby(&self, user: &UserInfo) {}
    fn close_lobby(&self, user: &UserInfo) {}
    fn start_lobby(&self, user: &UserInfo, config: GameConfig) {}

    fn join_lobby(&self, user: &UserInfo) {}
    fn leave_lobby(&self, user: &UserInfo) {}
}

struct Lobby {
    host: UserInfo,
    client: Option<UserInfo>,
}

impl Lobby {
    fn new(host: UserInfo) -> Arc<Self> {
        Arc::new(Self { host, client: None })
    }

    fn is_host(&self, user: &UserInfo) -> bool {
        &self.host == user
    }
}

struct Matchmaker {
    queue: RwLock<Vec<UserInQueue>>,
}

impl Matchmaker {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: RwLock::new(vec![]),
        })
    }

    fn start(self: Arc<Self>) {
        tokio::task::spawn(self.run());
    }
    async fn run(self: Arc<Self>) {
        let mut interval = time::interval(Duration::from_secs(2));
        loop {
            interval.tick().await;

            let mut writer = self.queue.write().await;
            Matchmaker::make_matches(&mut writer);
            drop(writer);
        }
    }

    /// ### Matchmaking algorithm
    ///
    /// Currently does not implement rating
    ///
    /// Currently does not implement timestamp
    ///
    /// Currently does not implement any statistic analysis
    fn make_matches(queue: &mut Vec<UserInQueue>) {
        while queue.len() > 2 {
            let mut rng = thread_rng();
            // Get random players
            let player1 = queue.take_random(&mut rng).unwrap();
            let player2 = queue.take_random(&mut rng).unwrap();

            // Create player interfaces
            let (player1, p1_rx) = PlayerInterface::create(player1.user);
            let (player2, p2_rx) = PlayerInterface::create(player2.user);

            // TODO: Create game interfaces

            // Start game
            let game = InactiveGame::new(player1, GameConfig::default());
            game.start(player2);
        }
    }
}

pub struct UserInQueue {
    user: UserInfo,
    rating: u16,
    timestamp: Instant,
    // Stats can go here to help with matchmaking, if necessary
    // stats: STUFF
}

impl UserInQueue {
    pub fn new(user: &UserInfo) -> Self {
        let now = Instant::now();
        Self {
            user: user.clone(),
            timestamp: now, // Currently timestamp is not used, but is included for if we change the matchmaking algo
            rating: 1200, // Currently Rating does not change, and is just a static value applied to every player. Additionally rating is not used for matchmaking currently
        }
    }
}

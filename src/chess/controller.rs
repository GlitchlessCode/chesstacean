use super::game::{
    network::{ActionInterface, PlayerInterface},
    GameConfig, InactiveGame,
};
use crate::{
    server::user::{interface::GameInterface, UserInfo},
    traits::ChooseTake,
};
use rand::thread_rng;
use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{oneshot, RwLock},
    time::{self, Instant},
};

pub struct GameControllerInterface {
    matchmaker: Arc<Matchmaker>,
    lobby_manager: RwLock<LobbyManager>,
}
// TODO: Convert a lot of the UserInfo parts to include Targets for transmission
impl GameControllerInterface {
    pub fn new() -> Arc<Self> {
        let matchmaker = Matchmaker::new();
        Matchmaker::start(matchmaker.clone());

        let lobby_manager = RwLock::new(LobbyManager::new());

        Arc::new(Self {
            matchmaker,
            lobby_manager,
        })
    }

    pub async fn join_queue(&self, user: &UserInfo) -> Result<oneshot::Receiver<Option<GameInterface>>, ()> {
        self.matchmaker.join_queue(user).await
    }
    pub async fn leave_queue(&self, user: &UserInfo) {
        self.matchmaker.leave_queue(user).await
    }

    pub async fn create_lobby(&self) {}
    pub async fn close_lobby(&self) {}
    pub async fn start_lobby(&self) {}

    pub fn join_lobby(&self) {}
    pub fn leave_lobby(&self) {}
}

struct LobbyManager {
    codes: HashMap<String, Arc<Lobby>>,
    users: HashMap<UserInfo, Arc<Lobby>>,
}

impl LobbyManager {
    fn new() -> Self {
        Self {
            codes: HashMap::new(),
            users: HashMap::new(),
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
    in_queue: RwLock<BTreeSet<UserInfo>>,
}

impl Matchmaker {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: RwLock::new(vec![]),
            in_queue: RwLock::new(BTreeSet::new()),
        })
    }

    async fn join_queue(&self, user: &UserInfo) -> Result<oneshot::Receiver<Option<GameInterface>>, ()> {
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
            Matchmaker::make_matches(&mut queue, &mut in_queue);
            drop(queue);
            drop(in_queue);
        }
    }

    /// ### Matchmaking algorithm
    ///
    /// Currently does not implement rating
    ///
    /// Currently does not implement timestamp
    ///
    /// Currently does not implement any statistic analysis
    fn make_matches(queue: &mut Vec<UserInQueue>, in_queue: &mut BTreeSet<UserInfo>) {
        while queue.len() > 2 {
            let mut rng = thread_rng();
            // Get random players
            let player1 = queue.take_random(&mut rng).unwrap();
            let player2 = queue.take_random(&mut rng).unwrap();

            in_queue.remove(&player1.user);
            in_queue.remove(&player2.user);

            // Create player interfaces
            let (player1_interface, p1_move_rx, p1_event_rx) = PlayerInterface::create(player1.user.clone());
            let (player2_interface, p2_move_rx, p2_event_rx) = PlayerInterface::create(player2.user.clone());

            // Create actions interface
            let (actions, action_rx) = ActionInterface::create();

            // Create game
            let game = InactiveGame::new(player1_interface, actions, GameConfig::default());

            // TODO: Create game interface
            let p1_msg = game.messenger.channel();
            let player1_game_interface =
                GameInterface::new(p1_move_rx, action_rx.clone(), p1_event_rx, p1_msg.0, p1_msg.1);
            player1.reply(Some(player1_game_interface));

            let p2_msg = game.messenger.channel();
            let player2_game_interface = GameInterface::new(p2_move_rx, action_rx, p2_event_rx, p2_msg.0, p2_msg.1);
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

    reply_to: oneshot::Sender<Option<GameInterface>>,
    // Stats can go here to help with matchmaking, if necessary
    // stats: STUFF
}

impl UserInQueue {
    fn new(user: &UserInfo, reply_to: oneshot::Sender<Option<GameInterface>>) -> Self {
        let now = Instant::now();
        Self {
            user: user.clone(),
            timestamp: now, // Currently timestamp is not used, but is included for if we change the matchmaking algo
            rating: 1200, // Currently Rating does not change, and is just a static value applied to every player. Additionally rating is not used for matchmaking currently
            reply_to,
        }
    }
    fn create(user: &UserInfo) -> (Self, oneshot::Receiver<Option<GameInterface>>) {
        let (reply_to, reply_rx) = oneshot::channel();
        (Self::new(user, reply_to), reply_rx)
    }
    fn reply(self, interface: Option<GameInterface>) {
        self.reply_to.send(interface).unwrap();
    }
}

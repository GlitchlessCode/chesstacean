use super::game::{network::PlayerInterface, InactiveGame};
use crate::{server::user::UserInfo, traits::ChooseTake};
use rand::{thread_rng, Rng};
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::RwLock,
    time::{self, Instant},
};

pub struct GameControllerInterface {
    matchmaker: Arc<Matchmaker>,
}

impl GameControllerInterface {
    pub fn new() -> Self {
        let matchmaker = Matchmaker::new();
        Matchmaker::start(matchmaker.clone());
        Self { matchmaker }
    }

    pub fn join_queue(&self) {}
    pub fn leave_queue(&self) {}
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

            // Create game interfaces

            // Start game
            let game = InactiveGame::new(player1, rng.gen());
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
    pub fn new(user: UserInfo) -> Self {
        let now = Instant::now();
        Self {
            user,
            timestamp: now, // Currently timestamp is not used, but is included for if we change the matchmaking algo
            rating: 1200, // Currently Rating does not change, and is just a static value applied to every player. Additionally rating is not used for matchmaking currently
        }
    }
}

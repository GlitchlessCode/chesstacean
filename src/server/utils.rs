use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::sync::RwLock;

pub mod input;

pub fn get_timestamp() -> u128 {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards, should not be possible");
    since_epoch.as_millis()
}

pub type ArcLock<T> = Arc<RwLock<T>>;

pub trait ArcLockTrait<T> {
    fn new_arclock(value: T) -> ArcLock<T>;
}

impl<T> ArcLockTrait<T> for ArcLock<T> {
    fn new_arclock(value: T) -> ArcLock<T> {
        Arc::new(RwLock::new(value))
    }
}

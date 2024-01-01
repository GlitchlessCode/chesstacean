// use std::{error, fmt::Display, sync::Arc};
// use tokio::sync::RwLock;

// pub fn create_link() {}

// pub struct GameInterface {}

// pub struct ClientInterface {}

// #[derive(Debug)]
// pub enum Error {
//     InvalidStateStep,
// }

// impl Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "")
//     }
// }

// impl error::Error for Error {}

// pub struct Interface {
//     state: Mutex<Box<dyn InterfaceStateMachine>>,
// }

// impl Interface {
//     pub fn new() -> Arc<Self> {
//         Arc::new(Self {
//             state: Mutex::new(Box::new(NotStarted {})),
//         })
//     }
// }

// trait InterfaceStateMachine {
//     fn step(self) -> Result<Box<dyn InterfaceStateMachine>, Box<dyn InterfaceStateMachine>>;
// }

// struct NotStarted {}

// impl InterfaceStateMachine for NotStarted {
//     fn step(self) -> Result<Box<dyn InterfaceStateMachine>, Box<dyn InterfaceStateMachine>> {
//         Err(Box::new(NotStarted {}))
//     }
// }

// struct Calculating {}

// struct ThisTurn {}

// struct OpponentTurn {}

// struct GameOver {}

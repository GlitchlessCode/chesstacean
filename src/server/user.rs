use crate::server::ws::Connection;

pub mod registry;

pub struct User {
    pub handle: String,
    pub display: String,
    connections: Vec<Connection>,
}

use std::net::SocketAddrV4;

use tokio::sync::oneshot::{self, Sender};
use warp::Filter;

pub async fn start(config: ServerConfig) -> Sender<()> {
    let routes = warp::any().map(|| "Hello, World!");

    let (tx, rx) = oneshot::channel::<()>();

    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(config.addr, async {
        rx.await.ok();
    });

    // Spawn the server into a runtime
    tokio::task::spawn(server);

    tx
}

pub struct ServerConfig {
    pub addr: SocketAddrV4,
}

impl ServerConfig {
    pub fn new() {}
}

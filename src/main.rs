use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4},
};

use chesstacean::server;

#[tokio::main]
async fn main() {
    let tx = server::start(server::ServerConfig {
        addr: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3000),
    })
    .await;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    tx.send(()).unwrap();
}

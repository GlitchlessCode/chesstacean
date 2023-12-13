use chesstacean::server::{self, database, ServerConfig};
use std::env;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    eprint!("\x1b[2J");

    let mut args = env::args();
    args.next();

    let config = ServerConfig::build(args).unwrap_or(ServerConfig::new([127, 0, 0, 1], 3000, None));

    let (tx, _rx) = mpsc::channel(1);

    let routes = server::ws_make(server::page_make(server::static_make()), tx);

    database::start();

    match config.tls {
        Some(_) => {
            let tls_svr = server::run_tls_server(&config, routes).unwrap();
            tokio::task::spawn(tls_svr);
        }
        None => {
            let svr = server::run_server(&config, routes);
            tokio::task::spawn(svr);
        }
    }

    server::console::start(config);
}

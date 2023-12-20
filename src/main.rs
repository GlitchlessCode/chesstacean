use chesstacean::server::{
    self,
    database::{self, Database, DatabaseMessage, DatabaseResult},
    routes,
    user::registry::Registry,
    ServerConfig,
};
use std::{env, net::SocketAddr};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    eprint!("\x1b[2J");

    let (database, db_tx) = database::init();

    tokio::task::spawn(database);
    // "127.0.0.1:50240"
    let ip = SocketAddr::from(([127, 0, 0, 1], 50240));

    let thing = move |db: &Database| DatabaseResult::from(db.sessions().create_new_session(ip));

    let result = DatabaseMessage::send(thing, &db_tx).await.unwrap();

    eprintln!("{result}");

    // Create routes and mpsc for WebSockets
    let (ws_tx, ws_rx) = mpsc::channel(1);

    let user_registry = Registry::new();
    tokio::task::spawn(user_registry.start(ws_rx));

    let routes = routes::attach_404(routes::ws_make(routes::page_make(routes::static_make()), ws_tx));

    let mut args = env::args();
    args.next();

    let config = ServerConfig::build(args).unwrap_or(ServerConfig::new([127, 0, 0, 1], 3000, None));
    match config.tls {
        Some(_) => {
            let tls_svr = server::run_tls_server(&config, routes).expect("Could not start tls server successfully");
            tokio::task::spawn(tls_svr);
        }
        None => {
            let svr = server::run_server(&config, routes);
            tokio::task::spawn(svr);
        }
    }

    server::console::start(config);
}

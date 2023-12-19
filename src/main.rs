use chesstacean::server::{self, database, routes, user::registry::Registry, ServerConfig};
use std::env;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    eprint!("\x1b[2J");

    let db = database::init();

    let (tx, rx) = mpsc::channel(1);

    let user_registry = Registry::new();
    tokio::task::spawn(user_registry.start(rx));

    let routes = routes::attach_404(routes::ws_make(routes::page_make(routes::static_make()), tx));
    // let routes = routes::ws_make(routes::static_make(), tx);

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

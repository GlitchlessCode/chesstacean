use chesstacean::{
    server::{self, database, routes, tokens::TokenManager, user::registry::Registry, ServerConfig},
    word_loader,
};
use std::{env, sync::Arc};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    eprint!("\x1b[2J");

    let words = word_loader::load().await;

    // Create TokenManager
    let token_manager = Arc::new(TokenManager::new());

    // Create mpsc for WebSockets
    let (ws_tx, ws_rx) = mpsc::channel(10);

    // Create mpsc for DB
    let (db_tx, db_rx) = mpsc::channel(10);

    // Create and start user registry thread
    let user_registry = Registry::new(words, &db_tx);
    tokio::task::spawn(Registry::start(user_registry.clone(), ws_rx, token_manager.clone()));

    // Create and start database thread, and session flusher
    let (database, flusher) = database::init(db_rx, &db_tx, user_registry.clone());
    tokio::task::spawn(database);
    tokio::task::spawn(flusher);

    // Create routes
    let routes = routes::attach_404(routes::ws_make(
        routes::post_make(
            routes::page_make(routes::static_make(), &db_tx),
            &db_tx,
            user_registry.clone(),
        ),
        ws_tx,
        &db_tx,
        token_manager.clone(),
        user_registry,
    ));

    // Read Args
    let mut args = env::args();
    // Ignore first arg (represents name of program)
    args.next();

    // Create config, and start server using config and routes
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

    // Start console interface, consuming config
    server::console::start(config);
}

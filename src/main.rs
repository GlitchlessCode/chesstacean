use chesstacean::server::{self, ServerConfig};
use std::env;

#[tokio::main]
async fn main() {
    let mut args = env::args();
    args.next();

    let config = ServerConfig::build(args).unwrap_or(ServerConfig::new([127, 0, 0, 1], 3000, None));

    let routes = server::ws_make(server::static_make());

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

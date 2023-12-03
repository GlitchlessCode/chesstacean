use std::env;
use chesstacean::server::{ self, ServerConfig };

#[tokio::main]
async fn main() {
    let mut args = env::args();
    args.next();

    let config = ServerConfig::build(args).unwrap_or(ServerConfig::new([127, 0, 0, 1], 3000, None));

    match config.tls {
        Some(_) => {
            tokio::task::spawn(server::run_tls_server(&config).unwrap());
        }
        None => {
            tokio::task::spawn(server::run_server(&config));
        }
    }

    server::console::start(config);
}

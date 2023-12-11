use futures_util::Future;
use std::fmt::Display;
use std::str::FromStr;
use warp::{ reject::Rejection, Filter, filters::ws::WebSocket };
use tokio::sync::{mpsc, oneshot};

use std::net::{ Ipv4Addr, SocketAddrV4 };

pub mod console;
pub mod database;

pub fn static_make() -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> +
    Clone +
    Send +
    Sync {
    let images = warp::path("img").and(warp::fs::dir("./public/img"));
    let css = warp::path("css").and(warp::fs::dir("./public/css"));
    let js = warp::path("js").and(warp::fs::dir("./public/js"));
    images
        .or(css)
        .or(js)
        .or(warp::any().and(warp::fs::file("./public/pages/index.html"))) // Temporary static file serving
}

pub fn ws_make(
    routes: impl Filter<Extract = (impl warp::Reply,), Error = Rejection> +
        Clone +
        Send +
        Sync +
        'static,
    ws_target: mpsc::Sender<(warp::filters::ws::WebSocket, oneshot::Sender<()>)>
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync {
    let ws_route = warp
        ::path!("ws" / "echo")
        .and(warp::ws())
        .map(move |ws: warp::filters::ws::Ws| {
            let new_target = ws_target.clone();
            ws.on_upgrade(move |websocket| user_connected(websocket, new_target))
        });
    ws_route.or(routes)
}

async fn user_connected(websocket: WebSocket, ws_target: mpsc::Sender<(warp::filters::ws::WebSocket, oneshot::Sender<()>)>) {
    let (tx, rx) = oneshot::channel::<()>();
    ws_target.send((websocket, tx)).await.expect("This channel should never be closed");
    rx.await.ok();
}

pub fn run_server(
    config: &ServerConfig,
    routes: impl Filter<Extract = (impl warp::Reply,), Error = Rejection> +
        Clone +
        Send +
        Sync +
        'static
) -> impl Future<Output = ()> {
    warp::serve(routes).run(config.addr)
}

pub fn run_tls_server(
    config: &ServerConfig,
    routes: impl Filter<Extract = (impl warp::Reply,), Error = Rejection> +
        Clone +
        Send +
        Sync +
        'static
) -> Result<impl Future<Output = ()>, ()> {
    match &config.tls {
        Some(tls) =>
            Ok(warp::serve(routes).tls().cert_path(&tls.cert).key_path(&tls.key).run(config.addr)),
        None => Err(()),
    }
}

#[derive(Debug)]
pub struct ServerConfig {
    pub addr: SocketAddrV4,
    pub tls: Option<Tls>,
}

fn parse_arg<T: FromStr>(arg: Option<String>) -> Result<T, ()> {
    match arg {
        Some(arg) =>
            match arg.parse() {
                Ok(val) => Ok(val),
                Err(_) => Err(()),
            }
        None => Err(()),
    }
}

impl ServerConfig {
    pub fn new(ipv4: [u8; 4], port: u16, tls: Option<Tls>) -> Self {
        Self {
            addr: SocketAddrV4::new(Ipv4Addr::new(ipv4[0], ipv4[1], ipv4[2], ipv4[3]), port),
            tls,
        }
    }
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, ()> {
        let ipv4: [u8; 4] = [
            parse_arg(args.next())?,
            parse_arg(args.next())?,
            parse_arg(args.next())?,
            parse_arg(args.next())?,
        ];

        let port: u16 = parse_arg(args.next())?;

        let tls: Option<Tls> = if let Some(cert_path) = args.next() {
            if let Some(key_path) = args.next() {
                Some(Tls::new(cert_path, key_path))
            } else {
                None
            }
        } else {
            None
        };

        Ok(Self::new(ipv4, port, tls))
    }
}

impl Display for ServerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IPv4  {0}\nPort  {1}\nTls?  {2}", self.addr.ip(), self.addr.port(), match
            &self.tls
        {
            None => "No",
            Some(_) => "Yes",
        })
    }
}

#[derive(Debug)]
pub struct Tls {
    cert: String,
    key: String,
}

impl Tls {
    pub fn new(cert: String, key: String) -> Self {
        Self { cert, key }
    }
}

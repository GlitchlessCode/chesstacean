use futures_util::{FutureExt, SinkExt, StreamExt, TryFutureExt};
use warp::{
    filters::ws::{Message, WebSocket},
    http::Response,
    Filter,
};

#[tokio::main]
async fn main() {
    let first = warp::get().and(warp::path("game").and(warp::path::end()).map(|| "Hello!"));
    let second = warp::get().and(warp::any().map(|| {
        Response::builder().status(404).body(
            "<!DOCTYPE html><html><head></head><body><h1>404 Page Not Found</h1></body></html>",
        )
    }));

    let ws = warp::path("echo")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(move |socket| handle_connected(socket)));

    let routes = ws.or(first).or(second);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

async fn handle_connected(socket: WebSocket) {
    let (mut tx, mut rx) = socket.split();
    tx.send(Message::text("Connected!"))
        .unwrap_or_else(|e| eprintln!("Websocket Send Error: {}", e))
        .await
}

use futures_util::{FutureExt, StreamExt};
use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    // let first = warp::path("game").and(warp::path::end()).map(|| "Hello!");
    // let second = warp::any().map(|| {
    //     Response::builder().status(404).body(
    //         "<!DOCTYPE html><html><head></head><body><h1>404 Page Not Found</h1></body></html>",
    //     )
    // });
    // let test = first.or(second);

    // warp::serve(test).run(([127, 0, 0, 1], 3000)).await;

    let routes = warp::path("echo").and(warp::ws()).map(|ws: warp::ws::Ws| {
        ws.on_upgrade(|websocket| {
            let (tx, rx) = websocket.split();
            rx.forward(tx).map(|result| {
                if let Err(e) = result {
                    eprintln!("websocket error: {:?}", e)
                }
            })
        })
    });

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

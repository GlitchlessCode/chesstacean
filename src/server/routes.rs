use http::Response;
use warp::{filters::fs::File, reply::Reply};

use super::*;

/// Creates the server's static pages
///
/// Keep in mind, the values are hardcoded into this function (at least for now),
/// because they really just won't be changing.
///
/// Returns a `warp::Filter`, which can be subsquently chained into other filters
pub fn static_make() -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync {
    let images = warp::path("img").and(warp::fs::dir("./public/img"));
    let css = warp::path("css").and(warp::fs::dir("./public/css"));
    let js = warp::path("js").and(warp::fs::dir("./public/js"));
    images.or(css).or(js)
}

/// Creates the server's ws endpoints
///
/// Keep in mind, the values are hardcoded into this function (at least for now),
/// because they really just won't be changing.
///
/// Takes in a route to chain onto, as well as a `mspc::Sender<Connection>` in
/// order to pass off WebSocket management to an external source
///
/// Returns a `warp::Filter`, which can be subsquently chained into other filters
pub fn ws_make(
    routes: impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync + 'static,
    ws_target: mpsc::Sender<Connection>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync {
    let ws_route = warp::path!("ws" / "connect")
        .and(warp::ws())
        .map(move |ws: warp::filters::ws::Ws| {
            let new_target = ws_target.clone();
            ws.on_upgrade(move |websocket| ws_connected(websocket, new_target))
        });

    let token_route = warp::path!("ws" / "token")
        .and(warp::header("cookie"))
        .map(|cookie: String| Response::builder().body(cookie));

    ws_route.or(token_route).or(routes)
}

pub fn page_make(
    routes: impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync + 'static,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync {
    let home_route = warp::path::end()
        .and(warp::get())
        .and(warp::cookie::optional("auth"))
        .and(warp::fs::file("./public/pages/index.html"))
        .map(has_auth_cookie);

    home_route.or(routes)
}

fn has_auth_cookie(cookie: Option<String>, file: File) -> impl Reply {
    match cookie {
        None => warp::reply::with_header(file, "set-cookie", "auth=xyz; HttpOnly; SameSite=Strict"),
        Some(_cookie) => warp::reply::with_header(file, "set-cookie", "auth=xyz; HttpOnly; SameSite=Strict"),
    }
}

async fn ws_connected(websocket: WebSocket, ws_target: mpsc::Sender<Connection>) {
    let (tx, rx) = oneshot::channel::<()>();
    ws_target
        .send(Connection::new(websocket, tx))
        .await
        .expect("This channel should never be closed");
    rx.await.ok();
}

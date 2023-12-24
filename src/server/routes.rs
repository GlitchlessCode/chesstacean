use std::net::{IpAddr, SocketAddr};

use http::{Response, StatusCode};
use rand::{thread_rng, Rng};
use warp::{filters::fs::File, reply::Reply};

use self::reply::{Message, Status::Success};

use super::{
    database::{Database, DatabaseMessage, DatabaseResult},
    *,
};

pub mod reply;

/// ### Creates the server's static files
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

/// ### Creates the server's ws endpoints
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

/// ### Creates the server's main html pages. Most are still retrieved statically.
///
/// Some pages additionally have the optional or required `auth` cookie.
///
/// Keep in mind, the values are hardcoded into this function (at least for now),
/// because they really just won't be changing.
///
/// Returns a `warp::Filter`, which can be subsquently chained into other filters
pub fn page_make(
    routes: impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync + 'static,
    db_tx: &mpsc::Sender<DatabaseMessage>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync {
    let db_tx = db_tx.clone();
    let home_tx = db_tx.clone();
    let home_route = warp::path::end()
        .and(warp::get())
        .and(warp::cookie::optional("auth"))
        .and(warp::fs::file("./public/pages/index.html"))
        .and(warp::filters::addr::remote())
        .and_then(move |cookie, file, ip| {
            let tx = home_tx.clone();
            async move { auth_cookie(cookie, file, ip, tx).await }
        });

    let login_route = warp::path("login")
        .and(warp::get())
        .and(warp::cookie::optional("auth"))
        .and(warp::fs::file("./public/pages/login/index.html"))
        .and(warp::filters::addr::remote())
        .and_then(move |cookie, file, ip| {
            let tx = db_tx.clone();
            async move { auth_cookie(cookie, file, ip, tx).await }
        });

    home_route.or(login_route).or(routes)
}

/// ### Creates the server's POST request recievers
///
/// Some additionally **require** the `auth` cookie.
pub fn post_make(
    routes: impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync + 'static,
    db_tx: &mpsc::Sender<DatabaseMessage>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync {
    let auth_base = warp::post().and(warp::path("auth"));

    let login = warp::path("login")
        .and(warp::filters::cookie::cookie("auth"))
        .map(|cookie: String| {
            serde_json::to_string(&Message::Login {
                status: Success {
                    context: Some("Login".to_string()),
                },
            })
            .unwrap_or("Error serializing response".to_string())
        });

    let signup = warp::path("signup")
        .and(warp::filters::cookie::cookie("auth"))
        .map(|cookie: String| format!("Sign Up; Cookie:{}", cookie));

    routes.or(auth_base.and(login.or(signup)))
}

/// ### Creates the server's 404 page.
///
/// Keep in mind, the values are hardcoded into this function (at least for now),
/// because they really just won't be changing.
///
/// Returns a `warp::Filter`, which can be subsquently chained into other filters,
/// though this filter is intended be the last filter applied in a chain, as it
/// acts as a catchall
pub fn attach_404(
    routes: impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync + 'static,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync {
    let none_found_route = warp::get()
        .and(warp::any())
        .and(warp::fs::file("./public/404.html"))
        .map(|file: File| {
            let status = StatusCode::from_u16(404).expect("Hardcoded value, should be fine");
            warp::reply::with_status(file, status)
        });

    routes.or(none_found_route)
}

async fn auth_cookie(
    cookie: Option<String>,
    file: File,
    ip: Option<SocketAddr>,
    db_tx: mpsc::Sender<DatabaseMessage>,
) -> Result<impl Reply, warp::Rejection> {
    match cookie {
        None => Ok(warp::reply::with_header(
            file,
            "set-cookie",
            format!("auth={}; HttpOnly; SameSite=Strict", create_cookie(&db_tx, ip).await),
        )),
        Some(cookie) => {
            if validate_cookie(&db_tx, cookie.to_owned()).await {
                Ok(warp::reply::with_header(
                    file,
                    "set-cookie",
                    format!("auth={}; HttpOnly; SameSite=Strict", cookie),
                ))
            } else {
                Ok(warp::reply::with_header(
                    file,
                    "set-cookie",
                    format!("auth={}; HttpOnly; SameSite=Strict", create_cookie(&db_tx, ip).await),
                ))
            }
        }
    }
}

async fn create_cookie(db_tx: &mpsc::Sender<DatabaseMessage>, ip: Option<SocketAddr>) -> String {
    let func =
        move |db: &Database| DatabaseResult::from(db.sessions().create_new_session(ip.unwrap_or_else(random_ip)));
    let result = DatabaseMessage::send(func, &db_tx)
        .await
        .expect("Should panic if no session cookie could be created");

    match result {
        DatabaseResult::ResultString(r) => r.expect("Should panic if no session cookie could be created"),
        _ => panic!("Should always be a Result<String>"),
    }
}

async fn validate_cookie(db_tx: &mpsc::Sender<DatabaseMessage>, cookie: String) -> bool {
    let func = move |db: &Database| DatabaseResult::from(db.sessions().validate_session(cookie.as_str()));
    let result = DatabaseMessage::send(func, &db_tx)
        .await
        .expect("Should panic if no session cookie could be created");

    match result {
        DatabaseResult::Bool(b) => b,
        _ => panic!("Should always be a bool"),
    }
}

fn random_ip() -> SocketAddr {
    let mut rand = [0u8; 5];
    thread_rng().fill(&mut rand);
    SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(rand[0], rand[1], rand[2], rand[3])),
        rand[4] as u16,
    )
}

async fn ws_connected(websocket: WebSocket, ws_target: mpsc::Sender<Connection>) {
    let (tx, rx) = oneshot::channel::<()>();
    ws_target
        .send(Connection::new(websocket, tx))
        .await
        .expect("This channel should never be closed");
    rx.await.ok();
}

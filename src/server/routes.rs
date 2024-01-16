use http::{Response, StatusCode};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use warp::{filters::fs::File, reply::Reply};

use crate::server::{
    database::{auth::ArgonError, SQLError},
    utils::input::{validate_display, validate_handle, validate_password},
};

use self::reply::Message;

use super::{
    database::{Database, DatabaseMessage, DatabaseResult},
    tokens::TokenManager,
    user::{registry::Registry, UserInfo},
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
    db_tx: &mpsc::Sender<DatabaseMessage>,
    token_man: Arc<TokenManager>,
    user_reg: Arc<Registry>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync {
    let db_tx = db_tx.clone();
    let ws_route = warp::path!("ws" / "connect")
        .and(warp::ws())
        .map(move |ws: warp::filters::ws::Ws| {
            let new_target = ws_target.clone();
            ws.on_upgrade(move |websocket| ws_connected(websocket, new_target))
        });

    let token_route = warp::path!("ws" / "token")
        .and(warp::cookie::cookie("auth"))
        .and_then(move |cookie: String| {
            let tx = db_tx.clone();
            let token_man = Arc::clone(&token_man);
            let user_reg = Arc::clone(&user_reg);
            async move { ws_token(cookie, tx, token_man, user_reg).await }
        });

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
    user_reg: Arc<Registry>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone + Send + Sync {
    let auth_base = warp::post().and(warp::path("auth"));

    let login = {
        let db_tx = db_tx.clone();
        let user_reg = user_reg.clone();
        warp::path("login")
            .and(warp::cookie::cookie("auth"))
            .and(warp::body::json())
            .and_then(move |cookie: String, json: Login| {
                let db_tx = db_tx.clone();
                let user_reg = user_reg.clone();
                async move { log_in(cookie, json, db_tx, user_reg).await }
            })
    };

    let logout = {
        let db_tx = db_tx.clone();
        let user_reg = user_reg.clone();
        warp::path("logout")
            .and(warp::cookie::cookie("auth"))
            .and_then(move |cookie: String| {
                let db_tx = db_tx.clone();
                let user_reg = user_reg.clone();
                async move { log_out(cookie, db_tx, user_reg).await }
            })
    };

    let signup = {
        let db_tx = db_tx.clone();
        let user_reg = user_reg.clone();
        warp::path("signup")
            .and(warp::cookie::cookie("auth"))
            .and(warp::body::json())
            .and_then(move |cookie: String, json: SignUp| {
                let db_tx = db_tx.clone();
                let user_reg = user_reg.clone();
                async move { sign_up(cookie, json, db_tx, user_reg).await }
            })
    };

    routes.or(auth_base.and(login.or(logout).or(signup)))
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

async fn log_in(
    cookie: String,
    data: Login,
    db_tx: mpsc::Sender<DatabaseMessage>,
    user_reg: Arc<Registry>,
) -> Result<impl Reply, Rejection> {
    let user_info = match get_user_info(&cookie, &db_tx, &user_reg).await {
        Ok(ui) => ui,
        Err(()) => return fetching_handle_error(),
    };

    if let UserInfo::User { .. } = user_info {
        return Ok(error_message("Cannot log in while in an active session", None));
    }

    let Login { handle, password } = data;

    let func = {
        let handle = handle.clone();
        move |db: &Database| DatabaseResult::from(db.auth().validate_user(handle, password))
    };

    let result = DatabaseMessage::send(func, &db_tx).await;

    let result = match result {
        Ok(DatabaseResult::ResultBool(o)) => o,
        _ => return Ok(server_error("Error validating user")),
    };

    match result {
        Err(e) => {
            if let Some(err) = e.downcast_ref::<ArgonError>() {
                return Ok(server_error(err));
            }
            return Ok(server_error("Unknown Error Encountered"));
        }
        Ok(ok) => {
            if !ok {
                return Ok(error_message(
                    "ValidationError: Username or password is not valid",
                    None,
                ));
            }
        }
    }

    assign_session(cookie, handle, db_tx).await
}

async fn log_out(
    cookie: String,
    db_tx: mpsc::Sender<DatabaseMessage>,
    user_reg: Arc<Registry>,
) -> Result<impl Reply, Rejection> {
    let user_info = match get_user_info(&cookie, &db_tx, &user_reg).await {
        Ok(ui) => ui,
        Err(()) => return fetching_handle_error(),
    };

    if let UserInfo::Guest { .. } = user_info {
        return Ok(error_message("Cannot log out of guest session", None));
    }

    let func = move |db: &Database| DatabaseResult::from(db.sessions().end_session(&cookie));

    let result = DatabaseMessage::send(func, &db_tx).await;

    let result = match result {
        Ok(DatabaseResult::Bool(o)) => o,
        _ => return Ok(server_error("Error ending session")),
    };

    if !result {
        Ok(server_error(SQLError.to_string()))
    } else {
        Ok(Response::builder()
            .status(303)
            .header("Location", "/")
            .body("".to_string())
            .unwrap())
    }
}

async fn sign_up(
    cookie: String,
    data: SignUp,
    db_tx: mpsc::Sender<DatabaseMessage>,
    user_reg: Arc<Registry>,
) -> Result<impl Reply, Rejection> {
    let user_info = match get_user_info(&cookie, &db_tx, &user_reg).await {
        Ok(ui) => ui,
        Err(()) => return fetching_handle_error(),
    };

    if let UserInfo::User { .. } = user_info {
        return Ok(error_message("Cannot sign up while in an active session", None));
    }

    if let Err(e) = validate_handle(&data.handle) {
        return Ok(error_message(format!("{e}"), Some(Affects::Handle)));
    }
    if let Err(e) = validate_display(&data.display) {
        return Ok(error_message(format!("{e}"), Some(Affects::Display)));
    }
    if let Err(e) = validate_password(&data.password) {
        return Ok(error_message(format!("{e}"), Some(Affects::Password)));
    }

    let SignUp {
        handle,
        display,
        password,
    } = data;

    let func = {
        let handle = handle.clone();
        move |db: &Database| DatabaseResult::from(db.auth().create_user(handle, display, password))
    };

    let result = DatabaseMessage::send(func, &db_tx).await;

    let result = match result {
        Ok(DatabaseResult::ResultBool(o)) => o,
        _ => return Ok(server_error("Error creating user")),
    };

    match result {
        Err(e) => {
            if let Some(err) = e.downcast_ref::<ArgonError>() {
                return Ok(server_error(err));
            }
            if let Some(err) = e.downcast_ref::<SQLError>() {
                return Ok(server_error(err));
            }
            return Ok(server_error("Unknown Error Encountered"));
        }
        Ok(ok) => {
            if !ok {
                return Ok(error_message(
                    "UniquenessError: Handle must be unique",
                    Some(Affects::Handle),
                ));
            }
        }
    }

    assign_session(cookie, handle, db_tx).await
}

async fn assign_session(
    cookie: String,
    handle: String,
    db_tx: mpsc::Sender<DatabaseMessage>,
) -> Result<Response<String>, Rejection> {
    let func = move |db: &Database| DatabaseResult::from(db.sessions().assign_session_user(&cookie, handle));

    let result = DatabaseMessage::send(func, &db_tx).await;

    let result = match result {
        Ok(DatabaseResult::ResultBool(o)) => o,
        _ => return Ok(server_error("Error assigning session")),
    };

    match result {
        Err(_) => Ok(server_error(SQLError.to_string())),
        Ok(ok) => {
            if ok {
                Ok(Response::builder()
                    .status(303)
                    .header("Location", "/")
                    .body("".to_string())
                    .unwrap())
            } else {
                Ok(server_error("Session already has an assigned user"))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct SignUp {
    handle: String,
    display: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct Login {
    handle: String,
    password: String,
}

fn error_message(msg: impl ToString, affects: Option<Affects>) -> Response<String> {
    let ser = serde_json::to_string(&Message::Error {
        message: msg.to_string(),
        affects,
    });

    match ser {
        Ok(msg) => Response::builder().status(400).body(msg).unwrap(),
        Err(_) => server_error("Could not serialize error message"),
    }
}

#[derive(Serialize)]
enum Affects {
    Handle,
    Display,
    Password,
}

fn server_error(msg: impl ToString) -> Response<String> {
    Response::builder().status(500).body(msg.to_string()).unwrap()
}

async fn ws_token(
    cookie: String,
    db_tx: mpsc::Sender<DatabaseMessage>,
    token_man: Arc<TokenManager>,
    user_reg: Arc<Registry>,
) -> Result<impl Reply, Rejection> {
    let user_info = match get_user_info(&cookie, &db_tx, &user_reg).await {
        Ok(ui) => ui,
        Err(()) => return fetching_handle_error(),
    };

    let token = token_man.create_ws_token(user_info, cookie, "connect".to_string());
    let token = match token {
        Ok(t) => t,
        Err(_) => {
            return Ok(Response::builder()
                .status(500)
                .body("Failed to fetch to create token".to_string())
                .unwrap())
        }
    };

    Ok(Response::builder().body(token).unwrap())
}

async fn get_user_info(
    cookie: &String,
    db_tx: &mpsc::Sender<DatabaseMessage>,
    user_reg: &Arc<Registry>,
) -> Result<UserInfo, ()> {
    if let Some(info) = user_reg.get_session(&cookie).await {
        Ok(info)
    } else {
        let session_id = cookie.clone();
        let func = move |db: &Database| DatabaseResult::from(db.sessions().user_info_from_cookie(&session_id));
        let result = DatabaseMessage::send(func, &db_tx).await;
        let db_result = match result {
            Err(_) => {
                return Err(());
            }
            Ok(h) => h,
        };
        match db_result {
            DatabaseResult::UserInfo(Some(ui)) => Ok(ui),
            _ => return Err(()),
        }
    }
}

fn fetching_handle_error() -> Result<Response<String>, Rejection> {
    Ok(Response::builder()
        .status(500)
        .body("Failed to fetch session info".to_string())
        .unwrap())
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

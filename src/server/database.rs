extern crate sha2;

use super::{
    user::{registry::Registry, UserInfo},
    utils::get_timestamp,
};
use anyhow::{bail, Result};
use argon2::{
    password_hash::{self, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{engine::general_purpose, Engine};
use futures_util::Future;
use rand::{thread_rng, Rng};
use rusqlite::{config::DbConfig, params, Connection, Row};
use sha2::{Digest, Sha512};
use std::{error::Error, fmt::Display, fs, net::SocketAddr, sync::Arc};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    time::{self, Duration},
};

pub mod auth;
pub mod sessions;

use auth::Auth;
use sessions::Sessions;

pub fn init(
    registry: Arc<Registry>,
) -> (
    impl Future<Output = ()>,
    Sender<DatabaseMessage>,
    impl Future<Output = ()>,
) {
    let path = "./db/";

    fs::create_dir_all(path.to_owned()).expect(&format!("Failed to open or create directory at {path}"));
    let conn = Connection::open(path.to_owned() + "chesstacean.db3")
        .expect(&format!("Failed to open or create database at {path}chesstacean.db3"));
    conn.set_prepared_statement_cache_capacity(32);

    let database = Database::new(conn);

    let (tx, rx) = mpsc::channel(10);

    (database.start(rx), tx.clone(), flusher(tx, registry))
}

pub struct Database {
    conn: Connection,
}

impl Database {
    fn new(database: Connection) -> Self {
        database
            .set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)
            .expect("Failed to configure database");

        match create_tables(&database) {
            Err(e) => eprintln!("\x1b[1;31m{e}\x1b[0m\n"),
            Ok(_) => (),
        }
        Self { conn: database }
    }

    async fn start(self, mut db_rx: Receiver<DatabaseMessage>) -> () {
        while let Some(db_msg) = db_rx.recv().await {
            db_msg.run(&self);
        }
        panic!("db_rx mspc channel was closed: this channel should never close");
    }

    pub fn sessions<'a>(&'a self) -> Sessions<'a> {
        Sessions::new(&self.conn)
    }

    pub fn auth<'a>(&'a self) -> Auth<'a> {
        Auth::new(&self.conn, Argon2::default())
    }

    pub fn flush(&self, timestamp: u64) -> Result<Vec<String>> {
        let mut stmnt = self
            .conn
            .prepare_cached("SELECT cookie FROM sessions WHERE expiry <= ?1 OR invalid = 1")
            .expect("Should be a valid sql statement");

        let cookies: Vec<String> = stmnt
            .query_map(params![timestamp], |row| row.get(0))?
            .filter(|r: &Result<String, rusqlite::Error>| {
                if !r.is_ok() {
                    eprint!("\rProblem unwrapping cookie\n > ")
                };
                r.is_ok()
            })
            .map(|r| r.unwrap())
            .collect();

        let mut stmnt = self
            .conn
            .prepare_cached("DELETE FROM sessions WHERE expiry <= ?1 OR invalid = 1")
            .expect("Should be a valid sql statement");

        stmnt.execute(params![timestamp])?;

        Ok(cookies)
    }
}

async fn flusher(tx: Sender<DatabaseMessage>, registry: Arc<Registry>) {
    let mut interval = time::interval(Duration::from_secs(60));
    loop {
        let timestamp = get_timestamp();
        let result = DatabaseMessage::send(
            move |db: &Database| DatabaseResult::from(db.flush(timestamp as u64)),
            &tx,
        )
        .await;

        'inner: {
            let string_vec = match result {
                Ok(DatabaseResult::FlushResult(Ok(sv))) => sv,
                _ => {
                    eprint!("\rSession flush failed at {timestamp}\n > ");
                    break 'inner;
                }
            };
            for session in string_vec {
                registry.end_session(session).await;
            }
        }

        interval.tick().await;
    }
}

#[derive(Debug)]
pub struct SQLError;

impl Display for SQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SQLError")
    }
}

impl Error for SQLError {}

pub enum DatabaseResult {
    Bool(bool),
    String(String),
    ResultString(Result<String>),
    ResultBool(Result<bool>),
    UserInfo(Option<UserInfo>),
    FlushResult(Result<Vec<String>>),
}

impl From<bool> for DatabaseResult {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for DatabaseResult {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for DatabaseResult {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<Result<String>> for DatabaseResult {
    fn from(value: Result<String>) -> Self {
        Self::ResultString(value)
    }
}

impl From<Result<bool>> for DatabaseResult {
    fn from(value: Result<bool>) -> Self {
        Self::ResultBool(value)
    }
}

impl From<Option<UserInfo>> for DatabaseResult {
    fn from(value: Option<UserInfo>) -> Self {
        Self::UserInfo(value)
    }
}

impl From<Result<Vec<String>>> for DatabaseResult {
    fn from(value: Result<Vec<String>>) -> Self {
        Self::FlushResult(value)
    }
}

impl Display for DatabaseResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(b) => b.to_string(),
                Self::String(s) => s.to_string(),
                Self::ResultString(r) => format!("{r:?}"),
                Self::ResultBool(b) => format!("{b:?}"),
                Self::UserInfo(ui) => format!("{ui:?}"),
                Self::FlushResult(sv) => format!("{sv:?}"),
            }
        )
    }
}

pub struct DatabaseMessage {
    result: tokio::sync::oneshot::Sender<DatabaseResult>,
    func: Box<dyn FnOnce(&Database) -> DatabaseResult + Send>,
}

impl DatabaseMessage {
    pub fn new(
        func: impl Fn(&Database) -> DatabaseResult + Send + 'static,
        tx: tokio::sync::oneshot::Sender<DatabaseResult>,
    ) -> Self {
        Self {
            result: tx,
            func: Box::new(func),
        }
    }

    fn run(self, database: &Database) {
        let result = (self.func)(database);
        match self.result.send(result) {
            Ok(_) => (),
            Err(_) => eprint!("\x1b[1;31mFailed to return Database result to source\x1b[0m\n > "),
        }
    }

    pub async fn send(
        func: impl FnOnce(&Database) -> DatabaseResult + Send + 'static,
        db_tx: &Sender<DatabaseMessage>,
    ) -> Result<DatabaseResult> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        db_tx
            .send(Self {
                result: tx,
                func: Box::new(func),
            })
            .await
            .expect("db_tx mspc channel closed: this channel should never close");

        Ok(rx.await?)
    }
}

fn create_tables(database: &Connection) -> Result<()> {
    attempt_create(database)?;
    for table in get_tables().iter() {
        verify_table(database, table)?;
    }
    attempt_create(database)?;

    Ok(())
}

/// Attempts to create tables in sqlite database
///
/// Keep in mind, table definitions are hardcoded values
fn attempt_create(database: &Connection) -> Result<()> {
    database.execute(
        "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        handle TEXT NOT NULL UNIQUE,
        display TEXT NOT NULL,
        phc TEXT NOT NULL
   );",
        [],
    )?;

    database.execute(
        "CREATE TABLE IF NOT EXISTS games (
        id INTEGER PRIMARY KEY,
        name TEXT UNIQUE NOT NULL,
        black INTEGER NOT NULL,
        white INTEGER NOT NULL,
        moves TEXT,
        CONSTRAINT fk_black FOREIGN KEY (black) REFERENCES users(id),
        CONSTRAINT fk_white FOREIGN KEY (white) REFERENCES users(id)
       );",
        [],
    )?;

    database.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY,
            cookie TEXT NOT NULL UNIQUE,
            user INTEGER,
            expiry INTEGER NOT NULL DEFAULT(ROUND((julianday('now') - 2440587.5)*86400000) + 14400000),
            invalid INTEGER NOT NULL DEFAULT 0,
            CONSTRAINT fk_user FOREIGN KEY (user) REFERENCES users(id)
       );",
        [],
    )?;

    Ok(())
}

/// Creates `TableInfo` definitions
///
/// Keep in mind, table definitions are hardcoded values
fn get_tables() -> Vec<TableInfo> {
    let mut tables = Vec::new();

    tables.push(TableInfo {
        name: "users".to_owned(),
        columns: vec![
            ColumnInfo::default().name("id").kind("INTEGER").primary_key(true),
            ColumnInfo::default().name("handle").not_null(true),
            ColumnInfo::default().name("display").not_null(true),
            ColumnInfo::default().name("phc").not_null(true),
        ],
    });

    tables.push(TableInfo {
        name: "games".to_owned(),
        columns: vec![
            ColumnInfo::default().name("id").kind("INTEGER").primary_key(true),
            ColumnInfo::default().name("name").kind("TEXT").not_null(true),
            ColumnInfo::default().name("black").kind("INTEGER").not_null(true),
            ColumnInfo::default().name("white").kind("INTEGER").not_null(true),
            ColumnInfo::default().name("moves"),
        ],
    });

    tables.push(TableInfo {
        name: "sessions".to_owned(),
        columns: vec![
            ColumnInfo::default().name("id").kind("INTEGER").primary_key(true),
            ColumnInfo::default().name("cookie").not_null(true),
            ColumnInfo::default().name("user").kind("INTEGER"),
            ColumnInfo::default()
                .name("expiry")
                .kind("INTEGER")
                .not_null(true)
                .default_value(Some(
                    "ROUND((julianday('now') - 2440587.5)*86400000) + 14400000".to_owned(),
                )),
            ColumnInfo::default()
                .name("invalid")
                .kind("INTEGER")
                .not_null(true)
                .default_value(Some("0".to_owned())),
        ],
    });

    tables
}

fn verify_table(database: &Connection, template: &TableInfo) -> Result<(), rusqlite::Error> {
    let table = TableInfo::from_query(database, &template.name)?;
    if template == &table {
        Ok(())
    } else {
        database.execute(&format!("DROP TABLE {};", table.name), [])?;
        eprintln!("\x1b[1;31mDROPPED TABLE {}\x1b[0m\n", &template.name);
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TableInfo {
    name: String,
    columns: Vec<ColumnInfo>,
}

impl TableInfo {
    fn from_query(db: &Connection, query_str: &str) -> Result<Self, rusqlite::Error> {
        let mut statement = db.prepare_cached("SELECT * FROM pragma_table_info(?1)").unwrap();
        let query = statement.query([query_str]);
        let mapped_query = query?.mapped(|row| Ok(ColumnInfo::from_row(row)?));
        let mut vec = Vec::new();

        for info in mapped_query {
            vec.push(info?)
        }

        Ok(Self {
            name: query_str.to_owned(),
            columns: vec,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ColumnInfo {
    name: String,
    kind: String,
    not_null: bool,
    default_value: Option<String>,
    primary_key: bool,
}

impl Default for ColumnInfo {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            kind: "TEXT".to_owned(),
            not_null: false,
            default_value: None,
            primary_key: false,
        }
    }
}

impl ColumnInfo {
    fn from_row(row: &'_ Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            name: row.get(1)?,
            kind: row.get(2)?,
            not_null: row.get(3)?,
            default_value: row.get(4)?,
            primary_key: row.get(5)?,
        })
    }

    fn name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    fn kind(mut self, kind: &str) -> Self {
        self.kind = kind.to_owned();
        self
    }

    fn not_null(mut self, not_null: bool) -> Self {
        self.not_null = not_null;
        self
    }

    fn default_value(mut self, default: Option<String>) -> Self {
        self.default_value = default;
        self
    }

    fn primary_key(mut self, primary_key: bool) -> Self {
        self.primary_key = primary_key;
        self
    }
}

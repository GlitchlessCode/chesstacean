use super::user::User;
use anyhow::Result;
use rusqlite::{config::DbConfig, params, Connection, Row};
use std::fs;

pub fn init() -> Database {
    let path = "./db/";

    fs::create_dir_all(path.to_owned()).expect(&format!("Failed to open or create directory at {path}"));
    let database = Connection::open(path.to_owned() + "chesstacean.db3")
        .expect(&format!("Failed to open or create database at {path}chesstacean.db3"));

    Database::new(database)

    // let mut stmnt = database
    //     .prepare("INSERT INTO games(black, white, moves) VALUES (?1, ?2, ?3)")
    //     .unwrap();
    // stmnt.execute(rusqlite::params![1, 2, "7,6>7,4;4,1>4,3;"]).unwrap(); // DOES NOT FAIL
    // match stmnt.execute(rusqlite::params![1, 5, "7,6>7,4;"]) { // FAILS BECAUSE OF FK CONSTRAINT
    //     Ok(_) => (),
    //     Err(e) => eprintln!("E: {}", e),
    // };

    // This was succesfully sanitized
    // let params = rusqlite::params![
    //     "glitchlesscode",
    //     "Timothy",
    //     "1234",
    //     "password1234); DROP TABLE users; --",
    // ];

    // database
    //     .execute(
    //         "INSERT INTO users(handle, display, salt, digest) VALUES (?1, ?2, ?3, ?4)",
    //         params,
    //     )
    //     .unwrap();
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

    pub fn sessions<'a>(&'a self) -> Sessions<'a> {
        Sessions { conn: &self.conn }
    }
}

pub struct Sessions<'a> {
    conn: &'a Connection,
}

impl<'a> Sessions<'a> {
    pub fn validate_session(&self, cookie: &str) -> bool {
        let mut stmnt = self
            .conn
            .prepare_cached("SELECT invalid FROM sessions WHERE cookie = ?1")
            .expect("Should be a valid sql statement");
        let result: Vec<Result<bool, rusqlite::Error>> =
            match stmnt.query_map(params![cookie], |row| row.get::<usize, bool>(0)) {
                Ok(mapped) => mapped.collect(),
                Err(_) => return false,
            };

        if result.len() != 1 {
            return false;
        } else {
            let value = match result.into_iter().next() {
                Some(res) => res.expect("Should never be err"),
                None => return false,
            };
            !value
        }
    }
    pub fn create_new_session(user: Option<User>) {}
}

fn create_tables(database: &Connection) -> Result<(), rusqlite::Error> {
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
fn attempt_create(database: &Connection) -> Result<(), rusqlite::Error> {
    database.execute(
        "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        handle TEXT NOT NULL UNIQUE,
        display TEXT NOT NULL,
        salt TEXT NOT NULL,
        digest TEXT NOT NULL
   );",
        [],
    )?;

    database.execute(
        "CREATE TABLE IF NOT EXISTS games (
        id INTEGER PRIMARY KEY,
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
            expiry INTEGER NOT NULL DEFAULT(ROUND((julianday('now') - 2440587.5)*86400000)),
            invalid INTEGER NOT NULL DEFAULT 0,
            CONSTRAINT fk_user FOREIGN KEY (user) REFERENCES users(id)
       );",
        [],
    )?;

    database.execute(
        "CREATE TABLE IF NOT EXISTS tokens (
            id INTEGER PRIMARY KEY,
            token TEXT NOT NULL UNIQUE,
            session INTEGER NOT NULL,
            expiry INTEGER NOT NULL DEFAULT(ROUND((julianday('now') - 2440587.5)*86400000)),
            invalid INTEGER NOT NULL DEFAULT 0,
            CONSTRAINT fk_session FOREIGN KEY (session) REFERENCES sessions(id)
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
            ColumnInfo::default().name("salt").not_null(true),
            ColumnInfo::default().name("digest").not_null(true),
        ],
    });

    tables.push(TableInfo {
        name: "games".to_owned(),
        columns: vec![
            ColumnInfo::default().name("id").kind("INTEGER").primary_key(true),
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
                .default_value(Some("ROUND((julianday('now') - 2440587.5)*86400000)".to_owned())),
            ColumnInfo::default()
                .name("invalid")
                .kind("INTEGER")
                .not_null(true)
                .default_value(Some("0".to_owned())),
        ],
    });

    tables.push(TableInfo {
        name: "tokens".to_owned(),
        columns: vec![
            ColumnInfo::default().name("id").kind("INTEGER").primary_key(true),
            ColumnInfo::default().name("token").not_null(true),
            ColumnInfo::default().name("session").kind("INTEGER").not_null(true),
            ColumnInfo::default()
                .name("expiry")
                .kind("INTEGER")
                .not_null(true)
                .default_value(Some("ROUND((julianday('now') - 2440587.5)*86400000)".to_owned())),
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

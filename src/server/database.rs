use rusqlite::{Connection};

pub fn start() {
    let path = "./db/chesstacean.db3";
    Connection::open(path).expect(&format!("Failed to open or create database at {path}"));
}
use rusqlite::Connection;
use std::fs;

pub fn start() {
    let path = "./db/";

    fs::create_dir_all(path.to_owned()).expect(&format!("Failed to open or create directory at {path}"));
    Connection::open(path.to_owned() + "chesstacean.db3")
        .expect(&format!("Failed to open or create database at {path}chesstacean.db3"));
}

pub mod books;
pub mod chapters;
pub mod progress;
pub mod settings;
pub mod search;

use crate::error::AppResult;
use rusqlite::Connection;
use std::path::Path;
use std::sync::Mutex;

pub struct Db(pub Mutex<Connection>);

impl Db {
    pub fn open(path: &Path) -> AppResult<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(include_str!("../../migrations/001_init.sql"))?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        Ok(Db(Mutex::new(conn)))
    }
}

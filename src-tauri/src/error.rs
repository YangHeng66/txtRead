use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io: {0}")] Io(#[from] std::io::Error),
    #[error("db: {0}")] Db(#[from] rusqlite::Error),
    #[error("epub: {0}")] Epub(String),
    #[error("parse: {0}")] Parse(String),
    #[error("not found")] NotFound,
    #[error("{0}")] Other(String),
}

impl Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

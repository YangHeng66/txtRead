pub mod txt;
pub mod epub;

use crate::error::{AppError, AppResult};
use std::path::Path;

pub struct ParsedBook {
    pub title: String,
    pub author: Option<String>,
    pub format: &'static str,
    pub chapters: Vec<ParsedChapter>,
    pub cover: Option<Vec<u8>>,
}

pub struct ParsedChapter {
    pub title: String,
    pub content: String,
}

pub fn parse(path: &Path) -> AppResult<ParsedBook> {
    match path.extension().and_then(|s| s.to_str()).map(|s| s.to_lowercase()) {
        Some(ref e) if e == "txt" => txt::parse(path),
        Some(ref e) if e == "epub" => epub::parse(path),
        _ => Err(AppError::Parse("unsupported format".into())),
    }
}

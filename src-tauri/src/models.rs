use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author: Option<String>,
    pub format: String,
    pub source_path: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChapterMeta {
    pub idx: i64,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub idx: i64,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Progress {
    pub book_id: i64,
    pub chapter_idx: i64,
    pub scroll_ratio: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchHit {
    pub book_id: i64,
    pub book_title: String,
    pub chapter_idx: i64,
    pub chapter_title: String,
    pub snippet: String,
}

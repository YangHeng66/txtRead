use crate::db::{books, chapters, progress, search, settings, Db};
use crate::error::{AppError, AppResult};
use crate::models::{Book, Chapter, ChapterMeta, Progress, SearchHit};
use crate::parser;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub fn import_book(db: State<'_, Db>, path: String) -> AppResult<Book> {
    let p = PathBuf::from(&path);
    let parsed = parser::parse(&p)?;

    let mut conn = db.0.lock().map_err(|_| AppError::Other("db lock".into()))?;
    let id = books::insert(
        &conn,
        &parsed.title,
        parsed.author.as_deref(),
        parsed.format,
        &path,
        parsed.cover.as_deref(),
    )?;
    let rows: Vec<(String, String)> = parsed
        .chapters
        .into_iter()
        .map(|c| (c.title, c.content))
        .collect();
    chapters::insert_many(&mut conn, id, &rows)?;

    let book = books::list(&conn)?
        .into_iter()
        .find(|b| b.id == id)
        .ok_or(AppError::NotFound)?;
    Ok(book)
}

#[tauri::command]
pub fn list_books(db: State<'_, Db>) -> AppResult<Vec<Book>> {
    let c = db.0.lock().map_err(|_| AppError::Other("db lock".into()))?;
    books::list(&c)
}

#[tauri::command]
pub fn delete_book(db: State<'_, Db>, id: i64) -> AppResult<()> {
    let c = db.0.lock().map_err(|_| AppError::Other("db lock".into()))?;
    books::delete(&c, id)
}

#[tauri::command]
pub fn list_chapters(db: State<'_, Db>, book_id: i64) -> AppResult<Vec<ChapterMeta>> {
    let c = db.0.lock().map_err(|_| AppError::Other("db lock".into()))?;
    chapters::list_meta(&c, book_id)
}

#[tauri::command]
pub fn get_chapter(db: State<'_, Db>, book_id: i64, idx: i64) -> AppResult<Chapter> {
    let c = db.0.lock().map_err(|_| AppError::Other("db lock".into()))?;
    chapters::get_one(&c, book_id, idx)
}

#[tauri::command]
pub fn save_progress(db: State<'_, Db>, progress: Progress) -> AppResult<()> {
    let c = db.0.lock().map_err(|_| AppError::Other("db lock".into()))?;
    self::progress::upsert(&c, &progress)
}

#[tauri::command]
pub fn get_progress(db: State<'_, Db>, book_id: i64) -> AppResult<Option<Progress>> {
    let c = db.0.lock().map_err(|_| AppError::Other("db lock".into()))?;
    progress::get(&c, book_id)
}

#[tauri::command]
pub fn set_setting(db: State<'_, Db>, key: String, value: String) -> AppResult<()> {
    let c = db.0.lock().map_err(|_| AppError::Other("db lock".into()))?;
    settings::set(&c, &key, &value)
}

#[tauri::command]
pub fn get_setting(db: State<'_, Db>, key: String) -> AppResult<Option<String>> {
    let c = db.0.lock().map_err(|_| AppError::Other("db lock".into()))?;
    settings::get(&c, &key)
}

#[tauri::command]
pub fn search_text(
    db: State<'_, Db>,
    query: String,
    book_id: Option<i64>,
) -> AppResult<Vec<SearchHit>> {
    let c = db.0.lock().map_err(|_| AppError::Other("db lock".into()))?;
    search::query(&c, book_id, &query, 50)
}

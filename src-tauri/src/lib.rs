mod commands;
mod db;
mod error;
mod models;
mod parser;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let dir = app.path().app_data_dir().expect("app data dir");
            std::fs::create_dir_all(&dir).ok();
            let db = db::Db::open(&dir.join("library.sqlite")).expect("db init");
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::import_book,
            commands::list_books,
            commands::delete_book,
            commands::list_chapters,
            commands::get_chapter,
            commands::save_progress,
            commands::get_progress,
            commands::set_setting,
            commands::get_setting,
            commands::search_text,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

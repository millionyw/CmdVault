pub mod commands;
pub mod db;

use crate::db::Database;
use std::sync::Arc;
use tauri::Manager;

pub type DbState = Arc<Database>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize database in app data directory
    let app_data_dir = dirs::data_local_dir()
        .expect("Failed to get app data directory")
        .join("command-repo");
    std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");

    let db_path = app_data_dir.join("commands.db");
    let db = Database::new(&db_path).expect("Failed to initialize database");
    let db_state = Arc::new(db);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(db_state)
        .setup(|app| {
            let _window = app.get_webview_window("main").unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_all_commands,
            commands::create_command,
            commands::update_command,
            commands::delete_command,
            commands::copy_command,
            commands::export_commands,
            commands::import_commands,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
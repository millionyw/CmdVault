pub mod commands;
pub mod db;
pub mod tray;

use crate::db::Database;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

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
        .plugin(tauri_plugin_global_shortcut::build())
        .manage(db_state)
        .setup(|app| {
            // Create system tray
            tray::create_tray(app.handle())?;

            // Register global shortcut: Win+Shift+C
            let shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyC);
            let app_handle = app.handle().clone();
            app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
                if let Some(window) = app_handle.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            })?;

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
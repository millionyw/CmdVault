pub mod commands;
pub mod db;
pub mod tray;

use crate::db::Database;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub type DbState = Arc<Database>;

fn parse_shortcut(shortcut_str: &str) -> Result<Shortcut, String> {
    let parts: Vec<&str> = shortcut_str.split('+').map(|s| s.trim()).collect();

    let mut modifiers = Modifiers::empty();
    let mut code = None;

    for part in parts {
        match part.to_lowercase().as_str() {
            "commandorcontrol" | "ctrl" | "cmd" => modifiers |= Modifiers::CONTROL,
            "shift" => modifiers |= Modifiers::SHIFT,
            "alt" => modifiers |= Modifiers::ALT,
            "super" | "win" | "meta" => modifiers |= Modifiers::SUPER,
            key => {
                code = Some(match key.to_uppercase().as_str() {
                    "A" => Code::KeyA,
                    "B" => Code::KeyB,
                    "C" => Code::KeyC,
                    "D" => Code::KeyD,
                    "E" => Code::KeyE,
                    "F" => Code::KeyF,
                    "G" => Code::KeyG,
                    "H" => Code::KeyH,
                    "I" => Code::KeyI,
                    "J" => Code::KeyJ,
                    "K" => Code::KeyK,
                    "L" => Code::KeyL,
                    "M" => Code::KeyM,
                    "N" => Code::KeyN,
                    "O" => Code::KeyO,
                    "P" => Code::KeyP,
                    "Q" => Code::KeyQ,
                    "R" => Code::KeyR,
                    "S" => Code::KeyS,
                    "T" => Code::KeyT,
                    "U" => Code::KeyU,
                    "V" => Code::KeyV,
                    "W" => Code::KeyW,
                    "X" => Code::KeyX,
                    "Y" => Code::KeyY,
                    "Z" => Code::KeyZ,
                    "0" => Code::Digit0,
                    "1" => Code::Digit1,
                    "2" => Code::Digit2,
                    "3" => Code::Digit3,
                    "4" => Code::Digit4,
                    "5" => Code::Digit5,
                    "6" => Code::Digit6,
                    "7" => Code::Digit7,
                    "8" => Code::Digit8,
                    "9" => Code::Digit9,
                    "," => Code::Comma,
                    "." => Code::Period,
                    "/" => Code::Slash,
                    ";" => Code::Semicolon,
                    "'" => Code::Quote,
                    "[" => Code::BracketLeft,
                    "]" => Code::BracketRight,
                    "\\" => Code::Backslash,
                    "-" => Code::Minus,
                    "=" => Code::Equal,
                    " " | "space" => Code::Space,
                    "enter" => Code::Enter,
                    "tab" => Code::Tab,
                    "escape" | "esc" => Code::Escape,
                    "backspace" => Code::Backspace,
                    "delete" => Code::Delete,
                    "insert" => Code::Insert,
                    "home" => Code::Home,
                    "end" => Code::End,
                    "pageup" => Code::PageUp,
                    "pagedown" => Code::PageDown,
                    "arrowup" | "up" => Code::ArrowUp,
                    "arrowdown" | "down" => Code::ArrowDown,
                    "arrowleft" | "left" => Code::ArrowLeft,
                    "arrowright" | "right" => Code::ArrowRight,
                    "f1" => Code::F1,
                    "f2" => Code::F2,
                    "f3" => Code::F3,
                    "f4" => Code::F4,
                    "f5" => Code::F5,
                    "f6" => Code::F6,
                    "f7" => Code::F7,
                    "f8" => Code::F8,
                    "f9" => Code::F9,
                    "f10" => Code::F10,
                    "f11" => Code::F11,
                    "f12" => Code::F12,
                    _ => return Err(format!("Unknown key: {}", key)),
                });
            }
        }
    }

    let code = code.ok_or("No key code found in shortcut")?;
    Ok(Shortcut::new(Some(modifiers), code))
}

fn register_global_shortcut(
    app: &tauri::AppHandle,
    shortcut_str: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let shortcut = parse_shortcut(shortcut_str)?;
    let app_handle = app.clone();

    // Try to unregister existing shortcut first (ignore errors)
    let _ = app.global_shortcut().unregister(shortcut);

    app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
        if event.state == ShortcutState::Pressed {
            if let Some(window) = app_handle.get_webview_window("main") {
                if window.is_visible().unwrap_or(false) {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }
    })?;

    Ok(())
}

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
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(db_state)
        .setup(|app| {
            // Create system tray
            tray::create_tray(app.handle())?;

            // Get db state
            let db_state = app.state::<DbState>();

            // Load shortcuts from database
            let shortcuts_json = db_state.get_setting("shortcuts")
                .ok()
                .flatten()
                .unwrap_or_else(|| {
                    serde_json::to_string(&commands::Shortcuts::default()).unwrap()
                });

            let shortcuts: commands::Shortcuts =
                serde_json::from_str(&shortcuts_json).unwrap_or_default();

            // Register global shortcut from settings
            if let Err(e) = register_global_shortcut(app.handle(), &shortcuts.global) {
                eprintln!("Failed to register global shortcut: {}", e);
            }

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
            commands::get_shortcuts,
            commands::update_shortcut,
            commands::reset_shortcuts,
            commands::get_sync_status,
            commands::connect_with_token,
            commands::disconnect,
            commands::push_to_gist,
            commands::pull_from_gist,
            commands::link_gist,
            commands::copy_gist_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// src-tauri/src/commands/shortcut.rs
use crate::DbState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcuts {
    pub global: String,
    #[serde(rename = "newCommand")]
    pub new_command: String,
    #[serde(rename = "editSelected")]
    pub edit_selected: String,
    #[serde(rename = "deleteSelected")]
    pub delete_selected: String,
    #[serde(rename = "openSettings")]
    pub open_settings: String,
    pub export: String,
    pub import: String,
    pub close: String,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            global: "CommandOrControl+Shift+C".to_string(),
            new_command: "CommandOrControl+N".to_string(),
            edit_selected: "CommandOrControl+E".to_string(),
            delete_selected: "Delete".to_string(),
            open_settings: "CommandOrControl+,".to_string(),
            export: "CommandOrControl+Shift+E".to_string(),
            import: "CommandOrControl+Shift+I".to_string(),
            close: "Escape".to_string(),
        }
    }
}

fn default_shortcuts_json() -> String {
    serde_json::to_string(&Shortcuts::default()).unwrap()
}

#[tauri::command]
pub fn get_shortcuts(db: State<DbState>) -> Result<Shortcuts, String> {
    let shortcuts_json = db
        .get_setting("shortcuts")
        .map_err(|e| e.to_string())?
        .unwrap_or_else(default_shortcuts_json);

    serde_json::from_str(&shortcuts_json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_shortcut(db: State<DbState>, key: String, value: String) -> Result<(), String> {
    let mut shortcuts = get_shortcuts(db.clone())?;

    // Validate key exists
    match key.as_str() {
        "global" => shortcuts.global = value,
        "newCommand" => shortcuts.new_command = value,
        "editSelected" => shortcuts.edit_selected = value,
        "deleteSelected" => shortcuts.delete_selected = value,
        "openSettings" => shortcuts.open_settings = value,
        "export" => shortcuts.export = value,
        "import" => shortcuts.import = value,
        "close" => shortcuts.close = value,
        _ => return Err(format!("Unknown shortcut key: {}", key)),
    }

    let json = serde_json::to_string(&shortcuts).map_err(|e| e.to_string())?;
    db.set_setting("shortcuts", &json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reset_shortcuts(db: State<DbState>) -> Result<(), String> {
    let json = default_shortcuts_json();
    db.set_setting("shortcuts", &json).map_err(|e| e.to_string())
}
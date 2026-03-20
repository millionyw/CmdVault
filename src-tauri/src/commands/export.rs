use crate::db::Database;
use std::sync::Arc;
use tauri::State;

pub type DbState = Arc<Database>;

/// Export all commands as JSON string
#[tauri::command]
pub fn export_commands(db: State<DbState>) -> Result<String, String> {
    db.export_to_json().map_err(|e| e.to_string())
}

/// Import commands from JSON string, returns count of imported commands
#[tauri::command]
pub fn import_commands(db: State<DbState>, json: String) -> Result<usize, String> {
    db.import_from_json(&json).map_err(|e| e.to_string())
}
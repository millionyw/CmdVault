use crate::db::schema::{Command, NewCommand, UpdateCommand};
use crate::db::Database;
use std::sync::Arc;
use tauri::State;

pub type DbState = Arc<Database>;

/// Get all commands sorted by usage count
#[tauri::command]
pub fn get_all_commands(db: State<DbState>) -> Result<Vec<Command>, String> {
    db.get_all_commands().map_err(|e| e.to_string())
}

/// Create a new command
#[tauri::command]
pub fn create_command(
    db: State<DbState>,
    name: String,
    content: String,
    description: Option<String>,
) -> Result<Command, String> {
    let new_cmd = NewCommand {
        name,
        content,
        description,
    };
    db.create_command(new_cmd).map_err(|e| e.to_string())
}

/// Update an existing command
#[tauri::command]
pub fn update_command(
    db: State<DbState>,
    id: String,
    name: String,
    content: String,
    description: Option<String>,
) -> Result<Command, String> {
    let update_cmd = UpdateCommand {
        id,
        name,
        content,
        description,
    };
    db.update_command(update_cmd).map_err(|e| e.to_string())
}

/// Delete a command by ID
#[tauri::command]
pub fn delete_command(db: State<DbState>, id: String) -> Result<(), String> {
    db.delete_command(&id).map_err(|e| e.to_string())
}

/// Copy command content to clipboard and increment usage count
#[tauri::command]
pub fn copy_command(db: State<DbState>, id: String) -> Result<(), String> {
    // Get the command first
    let command = db
        .get_command_by_id(&id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Command not found".to_string())?;

    // Copy content to clipboard
    let mut clipboard = clipboard::ClipboardContext::new().map_err(|e| e.to_string())?;
    clipboard
        .set_contents(command.content)
        .map_err(|e| e.to_string())?;

    // Increment usage count
    db.increment_usage(&id).map_err(|e| e.to_string())?;

    Ok(())
}
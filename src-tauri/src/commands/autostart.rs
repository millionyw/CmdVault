use tauri::State;
use tauri_plugin_autostart::AutoLaunchManager;

#[tauri::command]
pub fn get_autostart_enabled(manager: State<'_, AutoLaunchManager>) -> Result<bool, String> {
    manager.is_enabled().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_autostart_enabled(manager: State<'_, AutoLaunchManager>, enabled: bool) -> Result<(), String> {
    if enabled {
        manager.enable().map_err(|e| e.to_string())?;
    } else {
        manager.disable().map_err(|e| e.to_string())?;
    }
    Ok(())
}

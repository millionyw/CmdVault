use tauri::{
    AppHandle, Manager,
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    Emitter, Runtime,
};

pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> Result<TrayIcon<R>, Box<dyn std::error::Error>> {
    // Create menu items
    let open_item = MenuItem::with_id(app, "open", "打开主窗口", true, None::<&str>)?;
    let new_command_item = MenuItem::with_id(app, "new_command", "新建命令", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    // Build menu
    let menu = Menu::with_items(app, &[
        &open_item,
        &new_command_item,
        &settings_item,
        &quit_item,
    ])?;

    // Build tray icon
    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "open" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "new_command" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = app.emit("tray:new-command", ());
                    }
                }
                "settings" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = app.emit("tray:settings", ());
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            // Left-click to show window
            if let tauri::tray::TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(tray)
}
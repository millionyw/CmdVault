use tauri::{
    AppHandle, Manager, Emitter, Runtime,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{TrayIcon, TrayIconBuilder},
};

pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> Result<TrayIcon<R>, Box<dyn std::error::Error>> {
    // Create menu items
    let open_item = MenuItem::with_id(app, "open", "打开主窗口", true, None::<&str>)?;
    let new_command_item = MenuItem::with_id(app, "new_command", "新建命令", true, None::<&str>)?;
    let export_item = MenuItem::with_id(app, "export", "导出到文件...", true, None::<&str>)?;
    let import_item = MenuItem::with_id(app, "import", "从文件导入...", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    // Build menu with separators
    let separator1 = PredefinedMenuItem::separator(app)?;
    let separator2 = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(app, &[
        &open_item,
        &separator1,
        &new_command_item,
        &export_item,
        &import_item,
        &separator2,
        &settings_item,
        &quit_item,
    ])?;

    // Load icon from resource
    let icon = app.default_window_icon()
        .cloned()
        .ok_or("No default window icon configured")?;

    println!("Creating tray with menu...");

    // Build tray icon
    let tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("CommandRepo - 命令仓库")
        .on_menu_event(|app, event| {
            println!("Menu event: {:?}", event.id);
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
                "export" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = app.emit("tray:export", ());
                    }
                }
                "import" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = app.emit("tray:import", ());
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            println!("Tray event: {:?}", event);
            let app = tray.app_handle();
            match event {
                tauri::tray::TrayIconEvent::Click {
                    button: tauri::tray::MouseButton::Left,
                    ..
                } => {
                    // Left-click to show window
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                tauri::tray::TrayIconEvent::Click {
                    button: tauri::tray::MouseButton::Right,
                    ..
                } => {
                    // Right-click - menu should show automatically
                    println!("Right click detected");
                }
                _ => {}
            }
        })
        .build(app)?;

    println!("Tray created successfully");
    Ok(tray)
}
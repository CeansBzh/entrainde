use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_positioner::{Position, WindowExt};

pub fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.as_ref().window().move_window(Position::TrayCenter);
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub fn hide_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}

pub fn destroy_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.destroy();
    }
}

pub fn show_timeline_window(app: &AppHandle) {
    // Check if the timeline window exists
    if let Some(window) = app.get_webview_window("timeline") {
        // Window exists, just show it
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        // Window doesn't exist, create it
        match WebviewWindowBuilder::new(app, "timeline", WebviewUrl::App("timeline.html".into()))
            .title("Historique des tâches - Entrainde")
            .inner_size(800.0, 600.0)
            .resizable(true)
            .minimizable(true)
            .maximizable(true)
            .build()
        {
            Ok(window) => {
                let _ = window.show();
                let _ = window.set_focus();
            }
            Err(e) => {
                eprintln!("Failed to create timeline window: {}", e);
            }
        }
    }
}

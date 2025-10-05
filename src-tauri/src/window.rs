use tauri::{AppHandle, Manager};
use tauri_plugin_positioner::{WindowExt, Position};

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

pub fn toggle_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        match window.is_visible() {
            Ok(true) => {
                // Window is visible, check if it's minimized
                match window.is_minimized() {
                    Ok(true) => {
                        // Window is minimized, show it
                        self::show_main_window(app);
                    }
                    Ok(false) => {
                        // Window is visible and not minimized, hide it
                        self::hide_main_window(app);
                    }
                    Err(_) => {
                        // Error checking minimized state, just show the window
                        self::show_main_window(app);
                    }
                }
            }
            Ok(false) => {
                // Window is hidden, show it
                self::show_main_window(app);
            }
            Err(_) => {
                // Error checking visibility, just show the window
                self::show_main_window(app);
            }
        }
    }
}

pub fn destroy_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.destroy();
    }
}
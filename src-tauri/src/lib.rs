mod tray;
mod window;

use tauri::{Manager, Window, WindowEvent};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                let app_handle = app.app_handle();
                app_handle.plugin(tauri_plugin_positioner::init())?;

                tray::create(app_handle)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet,])
        .on_window_event(|window: &Window, event: &WindowEvent| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Minimize to tray instead of closing the window
                let app_handle = window.app_handle();
                window::hide_main_window(&app_handle);
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

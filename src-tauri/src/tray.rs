use std::sync::atomic::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use crate::{clear_all_tasks, window, AppState};

pub fn create(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Create the tray menu
    let open_i = MenuItem::with_id(app_handle, "open", "Ouvrir", true, None::<&str>)?;
    let clear_i = MenuItem::with_id(
        app_handle,
        "clear",
        "Effacer toutes les tâches",
        true,
        None::<&str>,
    )?;
    let quit_i = MenuItem::with_id(app_handle, "quit", "Quitter", true, None::<&str>)?;
    let menu = Menu::with_items(app_handle, &[&open_i, &clear_i, &quit_i])?;

    TrayIconBuilder::new()
        .icon(app_handle.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray_handle, event| {
            tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);

            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    let app_handle = tray_handle.app_handle();
                    let state = app_handle.state::<AppState>();

                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                    let last_minimize = state.last_focus_loss_minimize.load(Ordering::Acquire);
                    if last_minimize > 0 && (now - last_minimize) < 200 {
                        // Tray icon has been clicked less than 200ms after a focus loss minimize, ignore it
                        // This prevents immediate reopening when the window is minimized by focus loss
                        return;
                    }

                    window::show_main_window(&app_handle);
                }
                _ => {}
            }
        })
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app_handle, event| match event.id.as_ref() {
            "quit" => {
                window::destroy_main_window(app_handle);
                app_handle.exit(0);
            }
            "open" => {
                window::show_main_window(app_handle);
            }
            "clear" => {
                clear_all_tasks(app_handle.state::<crate::AppState>()).unwrap();
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app_handle)?;
    Ok(())
}

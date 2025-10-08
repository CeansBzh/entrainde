use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use crate::{clear_all_tasks, window};

pub fn create(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Create the tray menu
    let open_i = MenuItem::with_id(app_handle, "open", "Ouvrir", true, None::<&str>)?;
    let clear_i = MenuItem::with_id(app_handle, "clear", "Effacer toutes les tâches", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app_handle, "quit", "Quitter", true, None::<&str>)?;
    let menu = Menu::with_items(app_handle, &[&open_i, &clear_i, &quit_i])?;

    TrayIconBuilder::new()
        .icon(app_handle.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray_handle, event| {
            tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);

            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                // Handle left click on the tray icon to toggle the main window
                window::toggle_main_window(&tray_handle.app_handle());
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

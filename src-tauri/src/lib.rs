mod tray;
mod window;

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tauri::{Manager, State, Window, WindowEvent, Wry};
use tauri_plugin_store::{Store, StoreExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    name: String,
    timestamp: u64,
}

pub struct AppState {
    store: Arc<Store<Wry>>,
    last_focus_loss_minimize: Arc<AtomicU64>,
}

#[tauri::command]
async fn add_task(name: String, state: State<'_, AppState>) -> Result<(), String> {
    let task = Task {
        name: name.clone(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs(),
    };

    // Get existing tasks
    let mut tasks: Vec<Task> = state
        .store
        .get("tasks")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    // Add new task
    tasks.push(task);

    // Save tasks
    state.store.set(
        "tasks",
        serde_json::to_value(&tasks).map_err(|e| e.to_string())?,
    );
    state.store.save().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, String> {
    let tasks: Vec<Task> = state
        .store
        .get("tasks")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    Ok(tasks)
}

#[tauri::command]
async fn search_tasks(query: String, state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let tasks: Vec<Task> = state
        .store
        .get("tasks")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    let query_lower = query.to_lowercase();

    // Get unique task names that match the query
    let unique_tasks: std::collections::HashSet<String> = tasks
        .iter()
        .map(|t| t.name.clone())
        .filter(|name| name.to_lowercase().contains(&query_lower))
        .collect();

    let mut result: Vec<String> = unique_tasks.into_iter().collect();

    // Sort by relevance (exact matches first, then starts with, then contains)
    result.sort_by(|a, b| {
        let a_lower = a.to_lowercase();
        let b_lower = b.to_lowercase();

        if a_lower == query_lower {
            return std::cmp::Ordering::Less;
        }
        if b_lower == query_lower {
            return std::cmp::Ordering::Greater;
        }
        if a_lower.starts_with(&query_lower) && !b_lower.starts_with(&query_lower) {
            return std::cmp::Ordering::Less;
        }
        if b_lower.starts_with(&query_lower) && !a_lower.starts_with(&query_lower) {
            return std::cmp::Ordering::Greater;
        }

        a.cmp(b)
    });

    Ok(result)
}

pub fn clear_all_tasks(state: State<'_, AppState>) -> Result<(), String> {
    // Clear all tasks
    let empty_tasks: Vec<Task> = vec![];
    state.store.set(
        "tasks",
        serde_json::to_value(&empty_tasks).map_err(|e| e.to_string())?,
    );
    state.store.save().map_err(|e| e.to_string())?;

    Ok(())
}

// Cleanup tasks older than today
fn cleanup_old_tasks(store: &Store<Wry>) -> Result<(), String> {
    let tasks: Vec<Task> = store
        .get("tasks")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    // Get the start of today (midnight) in seconds since UNIX epoch
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    // Calculate seconds in a day
    let seconds_in_day = 24 * 60 * 60;

    // Get today's start timestamp (midnight)
    let today_start = now - (now % seconds_in_day);

    // Filter tasks to keep only those created today
    let today_tasks: Vec<Task> = tasks
        .into_iter()
        .filter(|task| task.timestamp >= today_start)
        .collect();

    // Save the filtered tasks
    store.set(
        "tasks",
        serde_json::to_value(&today_tasks).map_err(|e| e.to_string())?,
    );
    store.save().map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            #[cfg(desktop)]
            {
                let app_handle = app.app_handle();
                app_handle.plugin(tauri_plugin_positioner::init())?;

                tray::create(app_handle)?;
            }

            // Initialize the store
            let store = app.store("tasks.json")?;

            // Cleanup old tasks on startup
            if let Err(e) = cleanup_old_tasks(&store) {
                eprintln!("Failed to cleanup old tasks: {}", e);
            }

            app.manage(AppState {
                store,
                last_focus_loss_minimize: Arc::new(AtomicU64::new(0)),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_task, get_tasks, search_tasks])
        .on_window_event(|window: &Window, event: &WindowEvent| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    // Minimize to tray instead of closing the window
                    let app_handle = window.app_handle();
                    window::hide_main_window(&app_handle);
                    api.prevent_close();
                }
                tauri::WindowEvent::Focused(false) => {
                    let app_handle = window.app_handle();
                    let state = app_handle.state::<AppState>();
                    
                    // Record the current time when minimizing due to focus loss
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                    state.last_focus_loss_minimize.store(now, Ordering::Release);
                    
                    window::hide_main_window(&app_handle);
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

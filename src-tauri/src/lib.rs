pub mod configs;
pub mod files;
pub mod logger;
pub mod tray;

use configs::config::Config;
use tauri::tray::TrayIconEvent;

use crate::tray::handlers::menue_item_auth_handler;

#[tauri::command]
fn get_config() -> Result<Config, String> {
    let config = Config::fetch();
    match config {
        Ok(config) => Ok(config),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn get_finder_selection() -> Option<Vec<String>> {
    files::macos::get_finder_selection()
}

#[tauri::command]
fn get_finder_selection_single() -> Option<String> {
    files::macos::get_finder_selection_single()
}

#[tauri::command]
fn close_window(window: tauri::Window) {
    window.close().unwrap();
}

pub fn setup(app: &tauri::App) {
    logger::logger::setup_logger();
    match configs::setup::initial_setup() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    menue_item_auth_handler(&app.handle());
}

pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            setup(app);
            tray::tray::setup(app)
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            close_window,
            get_config,
            get_finder_selection,
            get_finder_selection_single
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        tauri::RunEvent::TrayIconEvent(tray_event) => match tray_event {
            TrayIconEvent::Click { .. } => {}
            _ => {}
        },
        _ => {}
    });
}

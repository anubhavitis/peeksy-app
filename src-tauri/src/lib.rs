pub mod config;
pub mod files;
pub mod tray;

use config::Config;

#[tauri::command]
fn get_config() -> Config {
    let config = Config::get();
    config
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

pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| tray::tray::setup(app))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            close_window,
            get_config,
            get_finder_selection,
            get_finder_selection_single
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}

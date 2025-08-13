pub mod files;
pub mod launchd;
pub mod logger;
pub mod store;
pub mod tray;

use std::path::PathBuf;
use tray::menue_item_handlers::menue_item_auth_handler;

use store::{
    auth::{AuthSession, AuthValidation},
    config::PeeksyConfig,
    Store,
};

use tauri::Manager;

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

#[tauri::command]
fn save_auth_session(app: tauri::AppHandle, session: AuthSession) -> Result<(), String> {
    Store::save_auth(app, &session)
}

#[tauri::command]
fn get_auth_session(app: tauri::AppHandle) -> Result<AuthSession, String> {
    Store::fetch_auth(app)
}

#[tauri::command]
fn validate_auth_session(app: tauri::AppHandle) -> Result<AuthValidation, String> {
    Store::is_authenticated(app)
}

#[tauri::command]
fn clear_auth_session(app: tauri::AppHandle) -> Result<(), String> {
    Store::clear_auth(app)
}

#[tauri::command]
fn get_peeksy_config(app: tauri::AppHandle) -> Result<PeeksyConfig, String> {
    Store::fetch_peeksy_config(app)
}

#[tauri::command]
fn save_peeksy_config(app: tauri::AppHandle, config: PeeksyConfig) -> Result<(), String> {
    Store::save_peeksy_config(app, &config)
}

#[tauri::command]
fn update_config_field(
    app: tauri::AppHandle,
    field: String,
    value: String,
) -> Result<PeeksyConfig, String> {
    Store::update_config_field(app, &field, &value)
}

#[tauri::command]
fn is_config_complete(app: tauri::AppHandle) -> Result<bool, String> {
    Store::is_config_complete(app)
}

#[tauri::command]
fn reset_peeksy_config(app: tauri::AppHandle) -> Result<PeeksyConfig, String> {
    Store::reset_peeksy_config(app)
}

#[tauri::command]
fn validate_peeksy_config(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let config = Store::fetch_peeksy_config(app)?;
    Ok(Store::validate_peeksy_config(&config))
}

// In a Tauri command or with app handle access:
#[tauri::command]
fn get_config(app: tauri::AppHandle) -> Result<PathBuf, String> {
    let config_dir = app.path().app_local_data_dir().unwrap();
    Ok(config_dir.join("config.json"))
}

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    logger::logger::setup_logger();
    // Initialize Peeksy configuration
    match Store::fetch_peeksy_config(app.handle().clone()) {
        Ok(_) => {
            println!("Peeksy configuration initialized successfully");
        }
        Err(e) => {
            eprintln!("Config initialization error: {}", e);
        }
    }
    // check if user auth is valid from store.bin, else open /auth window
    let auth_session = Store::fetch_auth(app.handle().clone());
    if auth_session.is_err() {
        let app_handle = app.handle().clone();
        menue_item_auth_handler(&app_handle);
    } else {
        let auth_validation = Store::is_authenticated(app.handle().clone());
        if auth_validation.is_err() {
            let app_handle = app.handle().clone();
            menue_item_auth_handler(&app_handle);
        }
    }

    tray::tray::setup(app)?;

    Ok(())
}

pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(Default::default(), None))
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| setup(app))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            close_window,
            get_config,
            get_finder_selection,
            get_finder_selection_single,
            save_auth_session,
            get_auth_session,
            validate_auth_session,
            clear_auth_session,
            get_peeksy_config,
            save_peeksy_config,
            update_config_field,
            is_config_complete,
            reset_peeksy_config,
            validate_peeksy_config
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

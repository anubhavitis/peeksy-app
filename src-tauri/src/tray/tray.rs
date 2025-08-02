use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

use crate::{
    launchd,
    tray::menue_item_handlers::{
        menue_item_auth_handler, menue_item_config_handler, menue_item_rename_handler,
        menue_item_status_handler,
    },
};
use tauri_plugin_positioner::{Position, WindowExt};

fn get_rename_label() -> (String, bool) {
    let file_count = crate::files::macos::get_finder_selection()
        .map(|files| files.len())
        .unwrap_or(0);
    let label = if file_count > 1 {
        format!("Rename {} files", file_count)
    } else if file_count == 1 {
        "Rename file".to_string()
    } else {
        "Rename".to_string()
    };

    (label, file_count > 0)
}

fn get_status_label() -> (String, bool) {
    // Perform a blocking operation to fetch the status synchronously
    let status = std::thread::spawn(|| {
        let launchd = launchd::launchd::LaunchD::new();
        // Assuming there is a public method to get status synchronously, e.g., get_status()
        // Since is_loaded() and is_running() are async, we need to block on them
        tauri::async_runtime::block_on(async {
            launchd.is_loaded().await && launchd.is_running().await
        })
    })
    .join()
    .unwrap_or_else(|_| false);

    let label = if status {
        "Running".to_string()
    } else {
        "Start".to_string()
    };

    (label, !status)
}

fn dynamic_menue_builder(app: &AppHandle) -> Result<(), anyhow::Error> {
    let (status_i_label, status_i_enabled) = get_status_label();
    let status_i = MenuItem::with_id(
        app,
        "status",
        status_i_label,
        status_i_enabled,
        None::<&str>,
    )?;

    // Always add configs and auth
    let configs_i = MenuItem::with_id(app, "configs", "Configs", true, None::<&str>)?;
    let auth_i = MenuItem::with_id(app, "auth", "Auth", true, None::<&str>)?;

    let (rename_i_label, rename_i_enabled) = get_rename_label();
    let rename_i = MenuItem::with_id(
        app,
        "rename",
        rename_i_label,
        rename_i_enabled,
        None::<&str>,
    )?;

    let menu = Menu::with_items(app, &[&status_i, &rename_i, &configs_i, &auth_i])?;
    let tray = app
        .tray_by_id("main-tray")
        .ok_or(anyhow::anyhow!("Tray not found"))?;
    tray.set_menu(Some(menu))?;
    Ok(())
}

fn tray_icon_event_handler(tray: &TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            // Get the app handle from the tray
            let app_handle = tray.app_handle();
            let app_handle_clone = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = dynamic_menue_builder(&app_handle_clone) {
                    eprintln!("Failed to update menue: {}", e);
                }
            });
        }
        _ => {
            // println!("Other button clicked: {:?}", event);
        }
    }
}

fn menue_event_handler(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "status" => {
            let app_clone = app.clone();
            tauri::async_runtime::spawn(async move {
                menue_item_status_handler(&app_clone).await;
            });
        }
        "configs" => menue_item_config_handler(app),
        "auth" => menue_item_auth_handler(app),
        "rename" => menue_item_rename_handler(app),
        _ => {
            // println!("Other menu item clicked: {:?}", event);
        }
    }
}

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let win = app.get_webview_window("main").unwrap();
    #[cfg(not(target_os = "linux"))]
    let _ = win.as_ref().window().move_window(Position::TopRight);

    #[cfg(target_os = "macos")]
    apply_vibrancy(&win, NSVisualEffectMaterial::HudWindow, None, Some(10.0))
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    apply_blur(&win, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    // Create tray without menu items initially
    // Menu will be built dynamically when tray is clicked
    TrayIconBuilder::with_id("main-tray")
        .show_menu_on_left_click(true)
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| tray_icon_event_handler(tray, event))
        .on_menu_event(|app, event| menue_event_handler(app, event))
        .build(app)?;

    Ok(())
}

use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

use crate::tray::handlers::{
    menue_item_auth_handler, menue_item_config_handler, menue_item_rename_handler,
};
use tauri_plugin_positioner::{Position, WindowExt};

fn tray_icon_event_handler(_: &TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            println!("Tray icon clicked");
        }
        _ => {
            // println!("Other button clicked: {:?}", event);
        }
    }
}

fn menue_event_handler(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
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
    let _ = win.as_ref().window().move_window(Position::TopRight);

    #[cfg(target_os = "macos")]
    apply_vibrancy(&win, NSVisualEffectMaterial::HudWindow, None, Some(10.0))
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    apply_blur(&win, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    let start_i = MenuItem::with_id(app, "start", "Start", true, None::<&str>)?;
    let stop_i = MenuItem::with_id(app, "stop", "Stop", true, None::<&str>)?;
    let rename_i = MenuItem::with_id(app, "rename", "Rename", true, None::<&str>)?;
    let configs_i = MenuItem::with_id(app, "configs", "Configs", true, None::<&str>)?;
    let auth_i = MenuItem::with_id(app, "auth", "Auth", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&start_i, &stop_i, &rename_i, &configs_i, &auth_i])?;

    TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(true)
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| tray_icon_event_handler(tray, event))
        .on_menu_event(|app, event| menue_event_handler(app, event))
        .build(app)?;
    Ok(())
}

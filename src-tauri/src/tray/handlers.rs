use std::process::Command;

use rayon::prelude::*;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_positioner::{Position, WindowExt};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

use crate::files;

pub fn webview_window_builder(app: &AppHandle, window_name: &str, url: &str) {
    if let Some(window) = app.get_webview_window(window_name) {
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    let window =
        WebviewWindowBuilder::new(app, window_name, WebviewUrl::External(url.parse().unwrap()))
            .title("Peeksy App")
            .transparent(true)
            .decorations(false)
            .inner_size(600.0, 300.0)
            .build()
            .expect("Failed to create window");

    // Apply vibrancy effect
    #[cfg(target_os = "macos")]
    let _ = apply_vibrancy(
        &window,
        NSVisualEffectMaterial::HudWindow,
        Some(NSVisualEffectState::Active),
        Some(10.0),
    );

    #[cfg(target_os = "windows")]
    let _ = apply_blur(&window, Some((18, 18, 18, 125)));

    // Position the window
    let _ = window.as_ref().window().move_window(Position::TopRight);
}

pub fn menue_item_auth_handler(app: &AppHandle) {
    webview_window_builder(app, "auth-web-view", "http://localhost:1420/auth");
}

pub fn menue_item_config_handler(app: &AppHandle) {
    webview_window_builder(app, "config-web-view", "http://localhost:1420/configs");
}

pub fn menue_item_rename_handler(_: &AppHandle) {
    let files = files::macos::get_finder_selection();
    println!("files: {:?}", files);

    match files {
        Some(files) => {
            files.par_iter().for_each(|file| {
                let cmd = Command::new("peeksy")
                    .arg("rename")
                    .arg(file)
                    .output()
                    .expect("Failed to rename files");
                println!("cmd: {:?}", cmd);
            });
        }
        None => {
            println!("No files selected");
        }
    }
}

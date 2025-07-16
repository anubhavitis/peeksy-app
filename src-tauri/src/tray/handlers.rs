use std::process::Command;

use rayon::prelude::*;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_positioner::{Position, WindowExt};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

use crate::files;

pub fn menue_item_config_handler(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        // Window exists, just show it
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        // Window was closed, create a new one
        let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
            .title("peeksy-app")
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

        // Show and focus the window
        let _ = window.show();
        let _ = window.set_focus();
    }
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

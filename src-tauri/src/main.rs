#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(windows)]
mod global_input;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                // Keep the full-screen transparent layer click-through so it cannot
                // intercept clicks intended for the user's apps.
                window.set_decorations(false)?;
                window.set_ignore_cursor_events(true)?;
            }

            #[cfg(windows)]
            global_input::start(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run Usagi desktop demo");
}

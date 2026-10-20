#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg(windows)]
mod global_input;
use tauri::{menu::{Menu, MenuItem}, tray::TrayIconBuilder, AppHandle, Manager, PhysicalPosition, WebviewUrl, WebviewWindowBuilder, WindowEvent};

#[tauri::command]
fn pet_mask(data: Vec<u8>) -> Result<(), String> {
    if data.len() != 96 * 96 { return Err("Invalid mask".into()); }
    #[cfg(windows)]
    { *global_input::MASK.lock().map_err(|e| e.to_string())? = data; }
    Ok(())
}
#[tauri::command]
fn watch_input(enabled: bool) {
    #[cfg(windows)]
    global_input::WATCHING.store(enabled, std::sync::atomic::Ordering::Relaxed);
}
#[tauri::command]
fn drag_pet(app: AppHandle) -> Result<(), String> {
    app.get_webview_window("pet").ok_or("Missing pet")?.start_dragging().map_err(|e| e.to_string())
}
#[tauri::command]
fn dance_visible(app: AppHandle, visible: bool) -> Result<(), String> {
    let window = app.get_webview_window("main").ok_or("Missing overlay")?;
    window.set_ignore_cursor_events(true).map_err(|e| e.to_string())?;
    if visible { window.show() } else { window.hide() }.map_err(|e| e.to_string())
}
#[tauri::command]
fn open_settings(app: AppHandle) {
    if let Some(window) = app.get_webview_window("settings") { let _ = window.show(); let _ = window.set_focus(); }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![pet_mask, watch_input, drag_pet, dance_visible, open_settings])
        .setup(|app| {
            if let Some(main) = app.get_webview_window("main") { main.set_ignore_cursor_events(true)?; }
            eprintln!("[startup] creating pet webview");
            let pet = WebviewWindowBuilder::new(app, "pet", WebviewUrl::App("index.html?pet".into()))
                .title("우사기").inner_size(360.0, 360.0).decorations(false).transparent(true)
                .shadow(false).always_on_top(true).skip_taskbar(true).focused(false)
                .resizable(false).visible(false)
                .build()?;
            eprintln!("[startup] pet webview created");
            pet.set_ignore_cursor_events(true)?;
            if let Some(monitor) = app.primary_monitor()? {
                let p = monitor.position(); let s = monitor.size();
                let actual = pet.outer_size()?;
                pet.set_position(PhysicalPosition::new(p.x + s.width as i32 - actual.width as i32, p.y + s.height as i32 - actual.height as i32 - 48))?;
            }
            pet.show()?;
            eprintln!("[startup] creating settings webview");
            let settings = WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("index.html?settings".into()))
                .title("작은 친구 설정 · 0.4.2").inner_size(460.0, 720.0).min_inner_size(400.0, 560.0)
                .visible(true).skip_taskbar(false).build()?;
            eprintln!("[startup] settings webview created");
            let handle = app.handle().clone();
            settings.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    if let Some(window) = handle.get_webview_window("settings") { let _ = window.hide(); }
                }
            });
            let open = MenuItem::with_id(app, "settings", "설정 열기", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "종료", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open, &quit])?;
            if let Some(icon) = app.default_window_icon().cloned() {
                TrayIconBuilder::new().icon(icon).tooltip("작은 친구").menu(&menu).show_menu_on_left_click(true)
                    .on_menu_event(|app, event| match event.id().as_ref() { "settings" => open_settings(app.clone()), "quit" => app.exit(0), _ => {} }).build(app)?;
            }
            #[cfg(windows)]
            global_input::start(app.handle().clone());
            Ok(())
        }).run(tauri::generate_context!()).expect("failed to run desktop companion");
}

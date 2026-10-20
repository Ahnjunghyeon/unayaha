use std::{sync::{Mutex, atomic::{AtomicBool, Ordering}}, thread, time::Duration};
use tauri::{AppHandle, Emitter, Manager};
use windows::Win32::{Foundation::POINT, UI::{Input::KeyboardAndMouse::{GetAsyncKeyState, GetLastInputInfo, LASTINPUTINFO}, WindowsAndMessaging::GetCursorPos}};

pub static WATCHING: AtomicBool = AtomicBool::new(false);
pub static MASK: Mutex<Vec<u8>> = Mutex::new(Vec::new());

// Never install low-level input hooks: polling cannot block the Windows input chain.
pub fn start(app: AppHandle) {
    thread::spawn(move || {
        let mut last_input = 0;
        let mut ignoring = true;
        let mut shortcut_down = false;
        loop {
            thread::sleep(Duration::from_millis(40));
            unsafe {
                let mut input = LASTINPUTINFO { cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32, dwTime: 0 };
                if GetLastInputInfo(&mut input).as_bool() {
                    if input.dwTime != last_input && WATCHING.load(Ordering::Relaxed) {
                        let _ = app.emit_to("pet", "global-activity", ());
                    }
                    last_input = input.dwTime;
                }
                let shortcut = GetAsyncKeyState(0x11) < 0 && GetAsyncKeyState(0x10) < 0;
                if shortcut && GetAsyncKeyState(0x51) < 0 { app.exit(0); }
                let wake = shortcut && GetAsyncKeyState(0x55) < 0;
                if wake && !shortcut_down { let _ = app.emit_to("pet", "debug-wake", ()); }
                shortcut_down = wake;
                // Keep native dragging intact while a button is held.
                if GetAsyncKeyState(0x01) < 0 { continue; }
                let Some(pet) = app.get_webview_window("pet") else { continue; };
                let mut point = POINT::default();
                if GetCursorPos(&mut point).is_err() { continue; }
                let (Ok(pos), Ok(size)) = (pet.outer_position(), pet.outer_size()) else { continue; };
                let x = point.x - pos.x;
                let y = point.y - pos.y;
                let hit = if x >= 0 && y >= 0 && x < size.width as i32 && y < size.height as i32 && size.width > 0 && size.height > 0 {
                    let mx = (x as u64 * 96 / size.width as u64) as usize;
                    let my = (y as u64 * 96 / size.height as u64) as usize;
                    MASK.lock().map(|mask| mask.get(my * 96 + mx).copied().unwrap_or(0) > 0).unwrap_or(false)
                } else { false };
                if ignoring == hit {
                    if pet.set_ignore_cursor_events(!hit).is_ok() { ignoring = !hit; }
                }
            }
        }
    });
}

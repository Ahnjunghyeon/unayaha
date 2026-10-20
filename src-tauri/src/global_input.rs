use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        OnceLock,
    },
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use tauri::{AppHandle, Emitter};
use windows::core::PCWSTR;
use windows::{
    Win32::{
        Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL, VK_SHIFT},
            WindowsAndMessaging::{
                CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW,
                TranslateMessage, HHOOK, KBDLLHOOKSTRUCT, MSG, MSLLHOOKSTRUCT,
                WH_KEYBOARD_LL, WH_MOUSE_LL, WM_KEYDOWN, WM_MOUSEMOVE, WM_MOUSEWHEEL,
                WM_LBUTTONDOWN, WM_MBUTTONDOWN, WM_RBUTTONDOWN, WM_SYSKEYDOWN,
            },
        },
    },
};

static APP: OnceLock<AppHandle> = OnceLock::new();
static LAST_EMIT_MS: AtomicU64 = AtomicU64::new(0);

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn emit_activity() {
    let now = now_ms();
    let previous = LAST_EMIT_MS.load(Ordering::Relaxed);
    if now.saturating_sub(previous) < 90
        || LAST_EMIT_MS
            .compare_exchange(previous, now, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
    {
        return;
    }
    if let Some(app) = APP.get() {
        let _ = app.emit("global-activity", ());
    }
}

unsafe extern "system" fn keyboard_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 && (wparam.0 as u32 == WM_KEYDOWN || wparam.0 as u32 == WM_SYSKEYDOWN) {
        let key = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
        let control_down = GetAsyncKeyState(VK_CONTROL.0 as i32) < 0;
        let shift_down = GetAsyncKeyState(VK_SHIFT.0 as i32) < 0;

        // Ctrl+Shift+U forces a wake-up for testing. Ctrl+Shift+Q exits the app.
        // Key values are checked in memory only; they are never emitted or stored.
        if control_down && shift_down && key.vkCode == 0x55 {
            if let Some(app) = APP.get() {
                let _ = app.emit("debug-wake", ());
            }
            return CallNextHookEx(None, code, wparam, lparam);
        }
        if control_down && shift_down && key.vkCode == 0x51 {
            if let Some(app) = APP.get() {
                app.exit(0);
            }
            return CallNextHookEx(None, code, wparam, lparam);
        }
        emit_activity();
    }
    CallNextHookEx(None, code, wparam, lparam)
}

unsafe extern "system" fn mouse_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let message = wparam.0 as u32;
        if matches!(
            message,
            WM_MOUSEMOVE | WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_MOUSEWHEEL
        ) {
            // Read the hook struct to validate the event payload, then discard it.
            let _event = &*(lparam.0 as *const MSLLHOOKSTRUCT);
            emit_activity();
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

pub fn start(app: AppHandle) {
    let _ = APP.set(app);
    thread::Builder::new()
        .name("usagi-global-input-hooks".into())
        .spawn(|| unsafe {
            let module = match GetModuleHandleW(PCWSTR::null()) {
                Ok(module) => HINSTANCE::from(module),
                Err(error) => {
                    eprintln!("Could not get the app module handle: {error}");
                    return;
                }
            };
            let keyboard: HHOOK = match SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), Some(module), 0) {
                Ok(hook) => hook,
                Err(error) => {
                    eprintln!("Could not install keyboard activity hook: {error}");
                    return;
                }
            };
            let mouse: HHOOK = match SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook), Some(module), 0) {
                Ok(hook) => hook,
                Err(error) => {
                    eprintln!("Could not install mouse activity hook: {error}");
                    let _ = windows::Win32::UI::WindowsAndMessaging::UnhookWindowsHookEx(keyboard);
                    return;
                }
            };

            let mut message = MSG::default();
            while GetMessageW(&mut message, None, 0, 0).as_bool() {
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }

            let _ = windows::Win32::UI::WindowsAndMessaging::UnhookWindowsHookEx(keyboard);
            let _ = windows::Win32::UI::WindowsAndMessaging::UnhookWindowsHookEx(mouse);
        })
        .expect("failed to start Windows input listener");
}

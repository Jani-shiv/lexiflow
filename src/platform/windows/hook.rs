use super::ffi::*;
use crate::replacement::InjectionGuard;
use crate::scheduler::DebounceScheduler;
use crate::security::SecurityFilter;
use crate::suggestion::SuggestionManager;
use crate::text_context::TextContextBuffer;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

static RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Clone)]
pub struct KeyboardHookState {
    pub buffer: Arc<Mutex<TextContextBuffer>>,
    pub scheduler: Arc<DebounceScheduler>,
    pub suggestions: Arc<SuggestionManager>,
    pub security: Arc<SecurityFilter>,
    pub injection_guard: Arc<InjectionGuard>,
}

static HOOK_STATE: Mutex<Option<KeyboardHookState>> = Mutex::new(None);
static HOOK_HANDLE: Mutex<HHOOK> = Mutex::new(0);

pub fn start_keyboard_hook(state: KeyboardHookState) -> Result<(), String> {
    *HOOK_STATE.lock().unwrap() = Some(state);
    unsafe {
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(low_level_keyboard_proc), 0, 0);
        if hook == 0 {
            return Err("Failed to install low-level keyboard hook".to_string());
        }
        *HOOK_HANDLE.lock().unwrap() = hook;
        RUNNING.store(true, Ordering::SeqCst);

        // Windows message pump
        let mut msg: MSG = std::mem::zeroed();
        while RUNNING.load(Ordering::SeqCst) && GetMessageW(&mut msg, 0, 0, 0) > 0 {
            DispatchMessageW(&msg);
        }

        let mut handle = HOOK_HANDLE.lock().unwrap();
        if *handle != 0 {
            UnhookWindowsHookEx(*handle);
            *handle = 0;
        }
    }
    Ok(())
}

pub fn stop_keyboard_hook() {
    RUNNING.store(false, Ordering::SeqCst);
}

unsafe extern "system" fn low_level_keyboard_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let hook_h = *HOOK_HANDLE.lock().unwrap();
    if n_code == HC_ACTION && (w_param == WM_KEYDOWN as WPARAM || w_param == WM_SYSKEYDOWN as WPARAM) {
        let kbd = *(l_param as *const KBDLLHOOKSTRUCT);

        let state_opt = HOOK_STATE.lock().unwrap().clone();
        if let Some(state) = state_opt {
            // Ignore injected events from our own engine
            if state.injection_guard.is_injected() {
                return CallNextHookEx(hook_h, n_code, w_param, l_param);
            }

            // Check security filter on active window
            if let Some(win_info) = super::uia::get_active_window_info() {
                if !state.security.is_target_allowed(
                    Some(&win_info.process_name),
                    Some(&win_info.title),
                    win_info.is_password,
                ) {
                    state.buffer.lock().unwrap().clear();
                    state.suggestions.dismiss_all();
                    return CallNextHookEx(hook_h, n_code, w_param, l_param);
                }

                // Handle hotkeys: Tab to accept, Escape to dismiss
                if kbd.vk_code == VK_TAB as u32 {
                    if let Some(active) = state.suggestions.get_current_suggestion() {
                        let text_snapshot = state.buffer.lock().unwrap().get_text().to_string();
                        let injector = crate::replacement::TextInjector::new((*state.injection_guard).clone());
                        let is_current = state.scheduler.is_request_current(active.request_id);

                        if injector.apply_replacement(
                            &active,
                            &win_info.process_name,
                            &text_snapshot,
                            is_current,
                        ) == crate::replacement::ReplacementResult::Success {
                            state.suggestions.accept_current();
                            state.buffer.lock().unwrap().set_text(&active.replacement_text, active.replacement_text.len());
                            return 1; // Consume key event
                        }
                    }
                } else if kbd.vk_code == VK_ESCAPE as u32 {
                    if state.suggestions.get_current_suggestion().is_some() {
                        state.suggestions.reject_current();
                        return 1; // Consume key event
                    }
                }

                // Process buffer updates
                let mut buf = state.buffer.lock().unwrap();
                buf.set_app(&win_info.process_name);

                match kbd.vk_code as u16 {
                    VK_BACK => buf.backspace(),
                    VK_DELETE => buf.delete_forward(),
                    VK_LEFT => buf.move_cursor_left(),
                    VK_RIGHT => buf.move_cursor_right(),
                    VK_RETURN => {
                        buf.insert_char('\n');
                    }
                    VK_SPACE => {
                        buf.insert_char(' ');
                    }
                    _ => {
                        // Printable ascii / virtual key mapping
                        if let Some(c) = vk_to_char(kbd.vk_code) {
                            buf.insert_char(c);
                        }
                    }
                }

                let text = buf.get_text().to_string();
                let cursor = buf.cursor_pos();
                drop(buf);

                // Submit to async debounce scheduler
                state.scheduler.submit_input(&text, cursor, &win_info.process_name);
            }
        }
    }

    CallNextHookEx(hook_h, n_code, w_param, l_param)
}

fn vk_to_char(vk: u32) -> Option<char> {
    match vk as u16 {
        0x30..=0x39 => Some((vk as u8) as char),
        0x41..=0x5A => Some(((vk as u8) + 32) as char), // lowercase default
        VK_OEM_PERIOD => Some('.'),
        0xBC => Some(','),
        0xBD => Some('-'),
        0xBF => Some('/'),
        0xBA => Some(';'),
        0xDE => Some('\''),
        _ => None,
    }
}

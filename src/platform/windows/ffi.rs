use std::ffi::c_void;

pub type HWND = isize;
pub type HANDLE = isize;
pub type HHOOK = isize;
pub type HGLOBAL = *mut c_void;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type HINSTANCE = isize;
pub type HOOKPROC = unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT;

pub const WH_KEYBOARD_LL: i32 = 13;
pub const HC_ACTION: i32 = 0;
pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_SYSKEYDOWN: u32 = 0x0104;

pub const VK_BACK: u16 = 0x08;
pub const VK_TAB: u16 = 0x09;
pub const VK_RETURN: u16 = 0x0D;
pub const VK_ESCAPE: u16 = 0x1B;
pub const VK_SPACE: u16 = 0x20;
pub const VK_LEFT: u16 = 0x25;
pub const VK_RIGHT: u16 = 0x27;
pub const VK_DELETE: u16 = 0x2E;
pub const VK_OEM_PERIOD: u16 = 0xBE;

pub const CF_UNICODETEXT: u32 = 13;
pub const GMEM_MOVEABLE: u32 = 0x0002;
pub const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;

pub const INPUT_KEYBOARD: u32 = 1;
pub const KEYEVENTF_KEYUP: u32 = 0x0002;
pub const KEYEVENTF_UNICODE: u32 = 0x0004;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub w_param: WPARAM,
    pub l_param: LPARAM,
    pub time: u32,
    pub pt: POINT,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KBDLLHOOKSTRUCT {
    pub vk_code: u32,
    pub scan_code: u32,
    pub flags: u32,
    pub time: u32,
    pub dw_extra_info: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEYBDINPUT {
    pub w_vk: u16,
    pub w_scan: u16,
    pub dw_flags: u32,
    pub time: u32,
    pub dw_extra_info: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union INPUT_UNION {
    pub ki: KEYBDINPUT,
    pub padding: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT {
    pub r#type: u32,
    pub u: INPUT_UNION,
}

#[link(name = "user32")]
extern "system" {
    pub fn GetForegroundWindow() -> HWND;
    pub fn GetWindowTextW(hwnd: HWND, lp_string: *mut u16, n_max_count: i32) -> i32;
    pub fn GetWindowThreadProcessId(hwnd: HWND, lpdw_process_id: *mut u32) -> u32;
    pub fn SetWindowsHookExW(id_hook: i32, lpfn: Option<HOOKPROC>, hmod: HINSTANCE, dw_thread_id: u32) -> HHOOK;
    pub fn UnhookWindowsHookEx(hhk: HHOOK) -> i32;
    pub fn CallNextHookEx(hhk: HHOOK, n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT;
    pub fn GetMessageW(lp_msg: *mut MSG, hwnd: HWND, w_msg_filter_min: u32, w_msg_filter_max: u32) -> i32;
    pub fn DispatchMessageW(lp_msg: *const MSG) -> LRESULT;
    pub fn OpenClipboard(hwnd: HWND) -> i32;
    pub fn CloseClipboard() -> i32;
    pub fn EmptyClipboard() -> i32;
    pub fn GetClipboardData(u_format: u32) -> HANDLE;
    pub fn SetClipboardData(u_format: u32, h_mem: HANDLE) -> HANDLE;
    pub fn SendInput(c_inputs: u32, p_inputs: *const INPUT, cb_size: i32) -> u32;
}

#[link(name = "kernel32")]
extern "system" {
    pub fn OpenProcess(dw_desired_access: u32, b_inherit_handle: i32, dw_process_id: u32) -> HANDLE;
    pub fn QueryFullProcessImageNameW(h_process: HANDLE, dw_flags: u32, lp_exe_name: *mut u16, lpdw_size: *mut u32) -> i32;
    pub fn CloseHandle(h_object: HANDLE) -> i32;
    pub fn GlobalAlloc(u_flags: u32, dw_bytes: usize) -> HGLOBAL;
    pub fn GlobalLock(h_mem: HGLOBAL) -> *mut c_void;
    pub fn GlobalUnlock(h_mem: HGLOBAL) -> i32;
}

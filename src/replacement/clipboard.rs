#[derive(Debug, Default)]
pub struct ClipboardBackup {
    saved_text: Option<String>,
}

impl ClipboardBackup {
    pub fn new() -> Self {
        Self { saved_text: None }
    }

    pub fn backup(&mut self) {
        #[cfg(target_os = "windows")]
        {
            self.saved_text = get_windows_clipboard_text();
        }
    }

    pub fn restore(&self) {
        #[cfg(target_os = "windows")]
        {
            if let Some(text) = &self.saved_text {
                set_windows_clipboard_text(text);
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn get_windows_clipboard_text() -> Option<String> {
    use crate::platform::windows::ffi::*;

    unsafe {
        if OpenClipboard(0) == 0 {
            return None;
        }

        let handle = GetClipboardData(CF_UNICODETEXT);
        if handle == 0 {
            CloseClipboard();
            return None;
        }

        let ptr = GlobalLock(handle as HGLOBAL) as *const u16;
        if ptr.is_null() {
            CloseClipboard();
            return None;
        }

        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }

        let slice = std::slice::from_raw_parts(ptr, len);
        let text = String::from_utf16_lossy(slice);

        GlobalUnlock(handle as HGLOBAL);
        CloseClipboard();

        Some(text)
    }
}

#[cfg(target_os = "windows")]
fn set_windows_clipboard_text(text: &str) -> bool {
    use crate::platform::windows::ffi::*;

    let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let size_bytes = wide.len() * std::mem::size_of::<u16>();

    unsafe {
        if OpenClipboard(0) == 0 {
            return false;
        }
        EmptyClipboard();

        let h_mem = GlobalAlloc(GMEM_MOVEABLE, size_bytes);
        if h_mem.is_null() {
            CloseClipboard();
            return false;
        }

        let ptr = GlobalLock(h_mem) as *mut u16;
        if ptr.is_null() {
            CloseClipboard();
            return false;
        }

        std::ptr::copy_nonoverlapping(wide.as_ptr(), ptr, wide.len());
        GlobalUnlock(h_mem);

        let res = SetClipboardData(CF_UNICODETEXT, h_mem as HANDLE);
        CloseClipboard();

        res != 0
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_clipboard_text() -> Option<String> {
    None
}

#[cfg(not(target_os = "windows"))]
pub fn set_clipboard_text(_text: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_backup_lifecycle() {
        let mut backup = ClipboardBackup::new();
        backup.backup();
        backup.restore();
    }
}

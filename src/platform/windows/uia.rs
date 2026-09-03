use super::ffi::*;

pub struct ActiveWindowInfo {
    pub hwnd: isize,
    pub title: String,
    pub process_id: u32,
    pub process_name: String,
    pub is_password: bool,
}

pub fn get_active_window_info() -> Option<ActiveWindowInfo> {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd == 0 {
            return None;
        }

        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, &mut pid);

        // Get window title
        let mut title_buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, title_buf.as_mut_ptr(), 512);
        let title = if len > 0 {
            String::from_utf16_lossy(&title_buf[..len as usize])
        } else {
            String::new()
        };

        // Determine process name from PID
        let proc_name = get_process_name_by_pid(pid).unwrap_or_else(|| "unknown.exe".to_string());

        // Basic check for password cues
        let lower_title = title.to_lowercase();
        let is_password = lower_title.contains("password")
            || lower_title.contains("pin")
            || lower_title.contains("credential")
            || lower_title.contains("bitwarden")
            || lower_title.contains("1password")
            || lower_title.contains("keepass");

        Some(ActiveWindowInfo {
            hwnd,
            title,
            process_id: pid,
            process_name: proc_name,
            is_password,
        })
    }
}

fn get_process_name_by_pid(pid: u32) -> Option<String> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if handle == 0 {
            return None;
        }

        let mut path_buf = [0u16; 1024];
        let mut size = 1024u32;
        let success = QueryFullProcessImageNameW(handle, 0, path_buf.as_mut_ptr(), &mut size);
        CloseHandle(handle);

        if success != 0 && size > 0 {
            let full_path = String::from_utf16_lossy(&path_buf[..size as usize]);
            let filename = std::path::Path::new(&full_path)
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or(&full_path)
                .to_string();
            Some(filename)
        } else {
            None
        }
    }
}

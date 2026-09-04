use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct SecurityFilter {
    excluded_processes: HashSet<String>,
    sensitive_window_titles: Vec<String>,
}

impl SecurityFilter {
    pub fn new(excluded_apps: &[String]) -> Self {
        let mut processes = HashSet::new();
        for app in excluded_apps {
            processes.insert(app.to_lowercase());
        }

        // Built-in standard secure application defaults
        let built_in = [
            "1password.exe",
            "1password",
            "bitwarden.exe",
            "bitwarden",
            "keepass.exe",
            "keepassxc.exe",
            "keepassxc",
            "lastpass.exe",
            "lastpass",
            "enpass.exe",
            "dashlane.exe",
            "nordpass.exe",
            "roboform.exe",
            "authy.exe",
            "credentialui.exe",
            "consent.exe",
            "logonui.exe",
            "securityhealthhost.exe",
            "smartscreen.exe",
            "pinentry",
            "pinentry-qt",
            "pinentry-gtk-2",
            "gnome-keyring-prompt",
        ];

        for app in built_in {
            processes.insert(app.to_lowercase());
        }

        let sensitive_window_titles = vec![
            "sign in".to_string(),
            "log in".to_string(),
            "login".to_string(),
            "enter password".to_string(),
            "master password".to_string(),
            "windows security".to_string(),
            "user account control".to_string(),
            "credential".to_string(),
            "authenticator".to_string(),
            "two-factor".to_string(),
            "2fa".to_string(),
            "passphrase".to_string(),
            "unlock database".to_string(),
            "pin required".to_string(),
            "sudo".to_string(),
        ];

        Self {
            excluded_processes: processes,
            sensitive_window_titles,
        }
    }

    /// Checks if a process name is in the secure exclusion blocklist
    pub fn is_process_excluded(&self, process_name: &str) -> bool {
        let normalized = process_name.to_lowercase().replace('\\', "/");
        let file_name = std::path::Path::new(&normalized)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or(&normalized);

        self.excluded_processes.contains(file_name)
            || self.excluded_processes.contains(&normalized)
    }

    /// Checks if a window title indicates an authentication / sensitive dialog
    pub fn is_window_title_sensitive(&self, title: &str) -> bool {
        let lower = title.to_lowercase();
        for keyword in &self.sensitive_window_titles {
            if lower.contains(keyword) {
                return true;
            }
        }
        false
    }

    /// Primary check: Determine whether text acquisition is permitted for this input target
    pub fn is_target_allowed(
        &self,
        process_name: Option<&str>,
        window_title: Option<&str>,
        is_password_field: bool,
    ) -> bool {
        // 1. Never capture when UIA / OS marks field as password
        if is_password_field {
            return false;
        }

        // 2. Never capture in excluded processes
        if let Some(proc) = process_name {
            if self.is_process_excluded(proc) {
                return false;
            }
        }

        // 3. Never capture in secure / auth windows
        if let Some(title) = window_title {
            if self.is_window_title_sensitive(title) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_field_rejection() {
        let filter = SecurityFilter::new(&[]);
        assert!(!filter.is_target_allowed(Some("notepad.exe"), Some("Untitled"), true));
    }

    #[test]
    fn test_password_manager_exclusion() {
        let filter = SecurityFilter::new(&["custom_vault.exe".to_string()]);
        assert!(!filter.is_target_allowed(Some("1password.exe"), Some("Vault"), false));
        assert!(!filter.is_target_allowed(Some("C:\\Program Files\\Bitwarden\\bitwarden.exe"), Some("Bitwarden"), false));
        assert!(!filter.is_target_allowed(Some("custom_vault.exe"), Some("App"), false));
        assert!(filter.is_target_allowed(Some("notepad.exe"), Some("Notes"), false));
    }

    #[test]
    fn test_sensitive_title_detection() {
        let filter = SecurityFilter::new(&[]);
        assert!(!filter.is_target_allowed(Some("chrome.exe"), Some("Sign In - Google Accounts"), false));
        assert!(!filter.is_target_allowed(Some("any.exe"), Some("Enter Master Password"), false));
        assert!(filter.is_target_allowed(Some("code.exe"), Some("main.rs - Visual Studio Code"), false));
    }
}

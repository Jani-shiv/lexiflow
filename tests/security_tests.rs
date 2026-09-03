use lexiflow::replacement::ClipboardBackup;
use lexiflow::security::SecurityFilter;

#[test]
fn test_password_field_block() {
    let filter = SecurityFilter::new(&[]);
    assert!(!filter.is_target_allowed(Some("notepad.exe"), Some("Enter password"), true));
    assert!(!filter.is_target_allowed(Some("chrome.exe"), Some("Sign In - Google Accounts"), true));
}

#[test]
fn test_password_manager_process_block() {
    let filter = SecurityFilter::new(&[]);
    assert!(!filter.is_target_allowed(Some("1password.exe"), Some("1Password Vault"), false));
    assert!(!filter.is_target_allowed(Some("bitwarden.exe"), Some("Bitwarden"), false));
    assert!(!filter.is_target_allowed(Some("keepass.exe"), Some("KeePass Database"), false));
    assert!(!filter.is_target_allowed(Some("lastpass.exe"), Some("LastPass"), false));
}

#[test]
fn test_sensitive_window_title_block() {
    let filter = SecurityFilter::new(&[]);
    assert!(!filter.is_target_allowed(Some("explorer.exe"), Some("Windows Security - Enter PIN"), false));
    assert!(!filter.is_target_allowed(Some("browser.exe"), Some("Master Password Prompt"), false));
}

#[test]
fn test_allowed_standard_applications() {
    let filter = SecurityFilter::new(&[]);
    assert!(filter.is_target_allowed(Some("notepad.exe"), Some("Untitled - Notepad"), false));
    assert!(filter.is_target_allowed(Some("code.exe"), Some("index.rs - Visual Studio Code"), false));
    assert!(filter.is_target_allowed(Some("slack.exe"), Some("General Channel"), false));
    assert!(filter.is_target_allowed(Some("winword.exe"), Some("Document1 - Word"), false));
}

#[test]
fn test_clipboard_safety_guard() {
    let mut clipboard = ClipboardBackup::new();
    clipboard.backup();
    clipboard.restore();
}

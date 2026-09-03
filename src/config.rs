use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    #[serde(default = "default_language")]
    pub language: String,

    #[serde(default = "default_debounce_ms")]
    pub debounce_ms: u64,

    #[serde(default = "default_confidence_threshold")]
    pub confidence_threshold: f32,

    #[serde(default = "default_max_context_chars")]
    pub max_context_chars: usize,

    #[serde(default = "default_accept_hotkey")]
    pub accept_hotkey: String,

    #[serde(default = "default_reject_hotkey")]
    pub reject_hotkey: String,

    #[serde(default = "default_excluded_applications")]
    pub excluded_applications: Vec<String>,

    #[serde(default = "default_auto_startup")]
    pub auto_startup: bool,

    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_enabled() -> bool {
    true
}

fn default_language() -> String {
    "en".to_string()
}

fn default_debounce_ms() -> u64 {
    250
}

fn default_confidence_threshold() -> f32 {
    0.85
}

fn default_max_context_chars() -> usize {
    500
}

fn default_accept_hotkey() -> String {
    "Tab".to_string()
}

fn default_reject_hotkey() -> String {
    "Escape".to_string()
}

fn default_excluded_applications() -> Vec<String> {
    vec![
        "1password.exe".to_string(),
        "bitwarden.exe".to_string(),
        "keepass.exe".to_string(),
        "keepassxc.exe".to_string(),
        "lastpass.exe".to_string(),
        "credentialui.exe".to_string(),
        "consent.exe".to_string(),
        "logonui.exe".to_string(),
    ]
}

fn default_auto_startup() -> bool {
    false
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            language: default_language(),
            debounce_ms: default_debounce_ms(),
            confidence_threshold: default_confidence_threshold(),
            max_context_chars: default_max_context_chars(),
            accept_hotkey: default_accept_hotkey(),
            reject_hotkey: default_reject_hotkey(),
            excluded_applications: default_excluded_applications(),
            auto_startup: default_auto_startup(),
            log_level: default_log_level(),
        }
    }
}

impl AppConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).map_err(|e| format!("Failed to read config: {}", e))?;
        toml::from_str(&content).map_err(|e| format!("Failed to parse config TOML: {}", e))
    }

    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        Self::load_from_file(&path).unwrap_or_default()
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let content = toml::to_string_pretty(self).map_err(|e| format!("Failed to serialize config: {}", e))?;
        if let Some(parent) = path.as_ref().parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(path, content).map_err(|e| format!("Failed to write config: {}", e))
    }

    pub fn default_config_path() -> PathBuf {
        let mut path = dirs_config_path();
        path.push("personal_grammar_enhancer");
        path.push("config.toml");
        path
    }
}

fn dirs_config_path() -> PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        PathBuf::from(appdata)
    } else if let Ok(home) = std::env::var("HOME") {
        let mut p = PathBuf::from(home);
        p.push(".config");
        p
    } else {
        PathBuf::from(".")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = AppConfig::default();
        assert!(cfg.enabled);
        assert_eq!(cfg.debounce_ms, 250);
        assert_eq!(cfg.confidence_threshold, 0.85);
        assert!(cfg.excluded_applications.contains(&"1password.exe".to_string()));
    }

    #[test]
    fn test_config_serialization() {
        let cfg = AppConfig::default();
        let toml_str = toml::to_string(&cfg).expect("serialize");
        let parsed: AppConfig = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(parsed.debounce_ms, cfg.debounce_ms);
    }
}

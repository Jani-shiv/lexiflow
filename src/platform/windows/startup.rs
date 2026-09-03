use std::path::Path;

pub struct WindowsStartup;

impl WindowsStartup {
    /// Configures or removes Windows auto-start shortcut in Current User Startup folder
    pub fn configure_autostart(enable: bool) -> Result<(), String> {
        let appdata = std::env::var("APPDATA").map_err(|_| "APPDATA environment variable missing".to_string())?;
        let startup_dir = Path::new(&appdata)
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join("Startup");

        if !startup_dir.exists() {
            let _ = std::fs::create_dir_all(&startup_dir);
        }

        let shortcut_path = startup_dir.join("PersonalGrammarEnhancer.cmd");

        if enable {
            let exe_path = std::env::current_exe().map_err(|e| format!("Cannot locate current exe: {}", e))?;
            let cmd_content = format!("@echo off\r\nstart \"\" \"{}\" --daemon\r\n", exe_path.display());
            std::fs::write(&shortcut_path, cmd_content).map_err(|e| format!("Failed to create startup script: {}", e))?;
        } else if shortcut_path.exists() {
            let _ = std::fs::remove_file(&shortcut_path);
        }

        Ok(())
    }

    pub fn is_autostart_enabled() -> bool {
        if let Ok(appdata) = std::env::var("APPDATA") {
            let shortcut = Path::new(&appdata)
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs")
                .join("Startup")
                .join("PersonalGrammarEnhancer.cmd");
            shortcut.exists()
        } else {
            false
        }
    }
}

pub struct LinuxPlatform;

impl LinuxPlatform {
    pub fn is_supported() -> bool {
        cfg!(target_os = "linux")
    }

    pub fn is_wayland() -> bool {
        std::env::var("WAYLAND_DISPLAY").is_ok()
    }
}

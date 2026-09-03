pub struct MacOSPlatform;

impl MacOSPlatform {
    pub fn is_supported() -> bool {
        cfg!(target_os = "macos")
    }
}

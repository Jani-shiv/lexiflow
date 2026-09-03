pub mod linux;
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

pub use linux::LinuxPlatform;
pub use macos::MacOSPlatform;
#[cfg(target_os = "windows")]
pub use windows::*;

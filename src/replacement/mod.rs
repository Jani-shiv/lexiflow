pub mod clipboard;
pub mod guard;
pub mod injector;

pub use clipboard::ClipboardBackup;
pub use guard::{InjectionGuard, InjectionToken};
pub use injector::{ReplacementResult, TextInjector};

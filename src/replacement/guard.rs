use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct InjectionGuard {
    is_injecting: Arc<AtomicBool>,
}

impl InjectionGuard {
    pub fn new() -> Self {
        Self {
            is_injecting: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_injected(&self) -> bool {
        self.is_injecting.load(Ordering::SeqCst)
    }

    /// Acquires injection token. Releases automatically on drop.
    pub fn start_injection(&self) -> InjectionToken {
        self.is_injecting.store(true, Ordering::SeqCst);
        InjectionToken {
            flag: Arc::clone(&self.is_injecting),
        }
    }
}

pub struct InjectionToken {
    flag: Arc<AtomicBool>,
}

impl Drop for InjectionToken {
    fn drop(&mut self) {
        self.flag.store(false, Ordering::SeqCst);
    }
}

impl Default for InjectionGuard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_injection_guard_scope() {
        let guard = InjectionGuard::new();
        assert!(!guard.is_injected());
        {
            let _token = guard.start_injection();
            assert!(guard.is_injected());
        }
        assert!(!guard.is_injected());
    }
}

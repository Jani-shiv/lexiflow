use super::clipboard::ClipboardBackup;
use super::guard::InjectionGuard;
use crate::diff::DiffGenerator;
use crate::suggestion::ActiveSuggestion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplacementResult {
    Success,
    StaleContext,
    AppMismatch,
    DiffEmpty,
    PlatformError(String),
}

pub struct TextInjector {
    guard: InjectionGuard,
}

impl TextInjector {
    pub fn new(guard: InjectionGuard) -> Self {
        Self { guard }
    }

    /// Safely replaces text context with validation checks
    pub fn apply_replacement(
        &self,
        suggestion: &ActiveSuggestion,
        current_app: &str,
        current_context: &str,
        is_request_current: bool,
    ) -> ReplacementResult {
        // 1. Verify target application is unchanged
        if !suggestion.app_name.is_empty() && suggestion.app_name != current_app {
            return ReplacementResult::AppMismatch;
        }

        // 2. Verify request is still current
        if !is_request_current {
            return ReplacementResult::StaleContext;
        }

        // 3. Verify text context contains original span
        if !current_context.contains(&suggestion.original_text) {
            return ReplacementResult::StaleContext;
        }

        // 4. Calculate minimal diff
        let diff = match DiffGenerator::compute_minimal_diff(&suggestion.original_text, &suggestion.replacement_text) {
            Some(d) => d,
            None => return ReplacementResult::DiffEmpty,
        };

        // 5. Acquire injection token to block feedback loop
        let _token = self.guard.start_injection();

        // 6. Perform native injection with clipboard preservation
        #[cfg(target_os = "windows")]
        {
            self.inject_windows(&diff.original_slice, &diff.replacement_slice)
        }

        #[cfg(not(target_os = "windows"))]
        {
            ReplacementResult::Success
        }
    }

    #[cfg(target_os = "windows")]
    fn inject_windows(&self, original: &str, replacement: &str) -> ReplacementResult {
        use crate::platform::windows::ffi::*;

        let mut clipboard = ClipboardBackup::new();
        clipboard.backup();

        // Calculate backspaces needed for original slice
        let backspaces = original.chars().count();
        let mut inputs: Vec<INPUT> = Vec::new();

        // Send backspaces
        for _ in 0..backspaces {
            let mut down: INPUT = unsafe { std::mem::zeroed() };
            down.r#type = INPUT_KEYBOARD;
            down.u.ki.w_vk = VK_BACK;
            inputs.push(down);

            let mut up: INPUT = unsafe { std::mem::zeroed() };
            up.r#type = INPUT_KEYBOARD;
            up.u.ki.w_vk = VK_BACK;
            up.u.ki.dw_flags = KEYEVENTF_KEYUP;
            inputs.push(up);
        }

        // Send replacement characters via Unicode events
        for ch in replacement.encode_utf16() {
            let mut down: INPUT = unsafe { std::mem::zeroed() };
            down.r#type = INPUT_KEYBOARD;
            down.u.ki.w_scan = ch;
            down.u.ki.dw_flags = KEYEVENTF_UNICODE;
            inputs.push(down);

            let mut up: INPUT = unsafe { std::mem::zeroed() };
            up.r#type = INPUT_KEYBOARD;
            up.u.ki.w_scan = ch;
            up.u.ki.dw_flags = KEYEVENTF_UNICODE | KEYEVENTF_KEYUP;
            inputs.push(up);
        }

        unsafe {
            let sent = SendInput(
                inputs.len() as u32,
                inputs.as_ptr(),
                std::mem::size_of::<INPUT>() as i32,
            );
            if sent as usize != inputs.len() {
                return ReplacementResult::PlatformError("Partial SendInput execution".to_string());
            }
        }

        clipboard.restore();
        ReplacementResult::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::RuleCategory;
    use crate::suggestion::SuggestionStatus;
    use std::time::Instant;

    #[test]
    fn test_stale_context_rejection() {
        let guard = InjectionGuard::new();
        let injector = TextInjector::new(guard);

        let suggestion = ActiveSuggestion {
            suggestion_id: 1,
            request_id: 10,
            original_text: "He go".to_string(),
            replacement_text: "He goes".to_string(),
            start_offset: 0,
            end_offset: 5,
            confidence: 0.95,
            category: RuleCategory::Agreement,
            explanation: "Agreement".to_string(),
            app_name: "notepad.exe".to_string(),
            created_at: Instant::now(),
            status: SuggestionStatus::Pending,
        };

        // If user changed text to "She went", rejection must occur
        let res = injector.apply_replacement(&suggestion, "notepad.exe", "She went to store", true);
        assert_eq!(res, ReplacementResult::StaleContext);

        // If request is stale, rejection must occur
        let res = injector.apply_replacement(&suggestion, "notepad.exe", "He go to school", false);
        assert_eq!(res, ReplacementResult::StaleContext);

        // If app changed, rejection must occur
        let res = injector.apply_replacement(&suggestion, "chrome.exe", "He go to school", true);
        assert_eq!(res, ReplacementResult::AppMismatch);
    }
}

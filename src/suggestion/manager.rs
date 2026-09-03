use super::types::{ActiveSuggestion, SuggestionStatus};
use crate::confidence::FilteredSuggestion;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct SuggestionManager {
    next_id: AtomicU64,
    current_suggestion: Mutex<Option<ActiveSuggestion>>,
    ttl: Duration,
}

impl SuggestionManager {
    pub fn new(ttl: Duration) -> Self {
        Self {
            next_id: AtomicU64::new(1),
            current_suggestion: Mutex::new(None),
            ttl,
        }
    }

    /// Posts a newly generated suggestion, superseding any prior suggestion
    pub fn post_suggestion(
        &self,
        request_id: u64,
        filtered: FilteredSuggestion,
        app_name: &str,
    ) -> ActiveSuggestion {
        let sid = self.next_id.fetch_add(1, Ordering::SeqCst);
        let item = ActiveSuggestion {
            suggestion_id: sid,
            request_id,
            original_text: filtered.original_span,
            replacement_text: filtered.replacement,
            start_offset: filtered.start_offset,
            end_offset: filtered.end_offset,
            confidence: filtered.confidence,
            category: filtered.category,
            explanation: filtered.explanation,
            app_name: app_name.to_string(),
            created_at: Instant::now(),
            status: SuggestionStatus::Pending,
        };

        let mut lock = self.current_suggestion.lock().unwrap();
        *lock = Some(item.clone());
        item
    }

    /// Gets the current active suggestion if it has not expired
    pub fn get_current_suggestion(&self) -> Option<ActiveSuggestion> {
        let mut lock = self.current_suggestion.lock().unwrap();
        if let Some(s) = lock.as_mut() {
            if s.created_at.elapsed() > self.ttl {
                s.status = SuggestionStatus::Expired;
                return None;
            }
            if s.is_active() {
                return Some(s.clone());
            }
        }
        None
    }

    /// User accepts the active suggestion
    pub fn accept_current(&self) -> Option<ActiveSuggestion> {
        let mut lock = self.current_suggestion.lock().unwrap();
        if let Some(s) = lock.as_mut() {
            if s.is_active() && s.created_at.elapsed() <= self.ttl {
                s.status = SuggestionStatus::Accepted;
                return Some(s.clone());
            }
        }
        None
    }

    /// User rejects the active suggestion
    pub fn reject_current(&self) -> Option<ActiveSuggestion> {
        let mut lock = self.current_suggestion.lock().unwrap();
        if let Some(s) = lock.as_mut() {
            if s.is_active() {
                s.status = SuggestionStatus::Rejected;
                return Some(s.clone());
            }
        }
        None
    }

    /// Dismisses active suggestion (e.g. when text changes)
    pub fn dismiss_all(&self) {
        let mut lock = self.current_suggestion.lock().unwrap();
        if let Some(s) = lock.as_mut() {
            s.status = SuggestionStatus::Dismissed;
        }
    }
}

impl Default for SuggestionManager {
    fn default() -> Self {
        Self::new(Duration::from_secs(10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::RuleCategory;

    #[test]
    fn test_suggestion_lifecycle() {
        let mgr = SuggestionManager::new(Duration::from_secs(5));
        let filtered = FilteredSuggestion {
            original_span: "go".to_string(),
            replacement: "goes".to_string(),
            start_offset: 3,
            end_offset: 5,
            confidence: 0.96,
            category: RuleCategory::Agreement,
            explanation: "Agreement".to_string(),
        };

        let item = mgr.post_suggestion(100, filtered, "notepad.exe");
        assert!(item.is_active());

        let active = mgr.get_current_suggestion().unwrap();
        assert_eq!(active.replacement_text, "goes");

        let accepted = mgr.accept_current().unwrap();
        assert_eq!(accepted.status, SuggestionStatus::Accepted);

        // Cannot accept again
        assert!(mgr.get_current_suggestion().is_none());
    }
}

use crate::grammar::RuleCategory;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuggestionStatus {
    Pending,
    Accepted,
    Rejected,
    Dismissed,
    Expired,
}

#[derive(Debug, Clone)]
pub struct ActiveSuggestion {
    pub suggestion_id: u64,
    pub request_id: u64,
    pub original_text: String,
    pub replacement_text: String,
    pub start_offset: usize,
    pub end_offset: usize,
    pub confidence: f32,
    pub category: RuleCategory,
    pub explanation: String,
    pub app_name: String,
    pub created_at: Instant,
    pub status: SuggestionStatus,
}

impl ActiveSuggestion {
    pub fn is_active(&self) -> bool {
        self.status == SuggestionStatus::Pending
    }
}

use crate::grammar::{GrammarSuggestionCandidate, RuleCategory};

#[derive(Debug, Clone)]
pub struct FilteredSuggestion {
    pub original_span: String,
    pub replacement: String,
    pub start_offset: usize,
    pub end_offset: usize,
    pub confidence: f32,
    pub category: RuleCategory,
    pub explanation: String,
}

pub struct ConfidenceFilter {
    threshold: f32,
}

impl ConfidenceFilter {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }

    pub fn filter(&self, candidates: Vec<GrammarSuggestionCandidate>) -> Vec<FilteredSuggestion> {
        candidates
            .into_iter()
            .filter(|c| c.confidence >= self.threshold)
            .map(|c| FilteredSuggestion {
                original_span: c.original_text,
                replacement: c.corrected_text,
                start_offset: c.start_offset,
                end_offset: c.end_offset,
                confidence: c.confidence,
                category: c.category,
                explanation: c.explanation,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_threshold_filtering() {
        let filter = ConfidenceFilter::new(0.90);
        let candidates = vec![
            GrammarSuggestionCandidate {
                original_text: "teh".to_string(),
                corrected_text: "the".to_string(),
                start_offset: 0,
                end_offset: 3,
                confidence: 0.95,
                category: RuleCategory::Spelling,
                explanation: "Fix typo".to_string(),
            },
            GrammarSuggestionCandidate {
                original_text: "maybe".to_string(),
                corrected_text: "perhaps".to_string(),
                start_offset: 4,
                end_offset: 9,
                confidence: 0.70,
                category: RuleCategory::Homophone,
                explanation: "Style".to_string(),
            },
        ];

        let filtered = filter.filter(candidates);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].replacement, "the");
    }
}

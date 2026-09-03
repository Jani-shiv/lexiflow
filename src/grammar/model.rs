use super::dictionary::SpellDictionary;
use super::rules::{RuleCategory, RuleEngine};
use super::statistical::StatisticalModel;

#[derive(Debug, Clone, PartialEq)]
pub struct GrammarSuggestionCandidate {
    pub original_text: String,
    pub corrected_text: String,
    pub start_offset: usize,
    pub end_offset: usize,
    pub confidence: f32,
    pub category: RuleCategory,
    pub explanation: String,
}

pub struct GrammarEngine {
    rules: RuleEngine,
    dictionary: SpellDictionary,
    statistical: StatisticalModel,
}

impl GrammarEngine {
    pub fn new() -> Self {
        Self {
            rules: RuleEngine::new(),
            dictionary: SpellDictionary::new(),
            statistical: StatisticalModel::new(),
        }
    }

    /// Primary inference entry point: processes input sentence and returns the best suggestions
    pub fn infer(&self, text: &str) -> Vec<GrammarSuggestionCandidate> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Vec::new();
        }

        let mut candidates = Vec::new();

        // 1. Run deterministic grammar rules
        let rule_matches = self.rules.evaluate(text);
        for m in rule_matches {
            let mut conf = m.confidence;
            // Use statistical model to validate fluency boost
            if self.statistical.is_more_fluent(&m.replacement, text) {
                conf = (conf + 0.02).min(0.99);
            }

            candidates.push(GrammarSuggestionCandidate {
                original_text: text[m.start..m.end].to_string(),
                corrected_text: m.replacement,
                start_offset: m.start,
                end_offset: m.end,
                confidence: conf,
                category: m.category,
                explanation: m.explanation.to_string(),
            });
        }

        // 2. Run spell checking on individual word tokens
        let mut byte_idx = 0;
        for word in text.split_whitespace() {
            // Find word offset in text
            if let Some(pos) = text[byte_idx..].find(word) {
                let actual_start = byte_idx + pos;
                let actual_end = actual_start + word.len();
                byte_idx = actual_end;

                // Strip leading/trailing punctuation for check
                let cleaned = word.trim_matches(|c: char| !c.is_alphabetic());
                if !cleaned.is_empty() {
                    if let Some(fixed_word) = self.dictionary.check_word(cleaned) {
                        // Reconstruct word with punctuation
                        let full_fix = word.replace(cleaned, &fixed_word);
                        if full_fix != word {
                            // Check if this span is already covered by a higher priority rule
                            let already_covered = candidates.iter().any(|c| {
                                !(actual_end <= c.start_offset || actual_start >= c.end_offset)
                            });

                            if !already_covered {
                                candidates.push(GrammarSuggestionCandidate {
                                    original_text: word.to_string(),
                                    corrected_text: full_fix,
                                    start_offset: actual_start,
                                    end_offset: actual_end,
                                    confidence: 0.95,
                                    category: RuleCategory::Spelling,
                                    explanation: format!("Corrected spelling of '{}' to '{}'", word, fixed_word),
                                });
                            }
                        }
                    }
                }
            }
        }

        // 3. Score & filter candidates using statistical language model
        candidates.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        candidates
    }

    /// Produces a full corrected sentence by applying all non-conflicting suggestions
    pub fn correct_sentence(&self, text: &str) -> (String, Vec<GrammarSuggestionCandidate>) {
        let suggestions = self.infer(text);
        if suggestions.is_empty() {
            return (text.to_string(), Vec::new());
        }

        // Sort by start offset descending so replacements don't invalidate previous offsets
        let mut sorted = suggestions.clone();
        sorted.sort_by(|a, b| b.start_offset.cmp(&a.start_offset));

        let mut corrected = text.to_string();
        let mut applied = Vec::new();
        let mut last_start = usize::MAX;

        for s in sorted {
            if s.end_offset <= last_start {
                if s.start_offset < corrected.len() && s.end_offset <= corrected.len() {
                    corrected.replace_range(s.start_offset..s.end_offset, &s.corrected_text);
                    last_start = s.start_offset;
                    applied.push(s);
                }
            }
        }

        applied.reverse();
        (corrected, applied)
    }
}

impl Default for GrammarEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_grammar_inference_he_go() {
        let engine = GrammarEngine::new();
        let (corrected, suggestions) = engine.correct_sentence("He go to school.");
        assert_eq!(corrected, "He goes to school.");
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_full_grammar_inference_i_am_go_office() {
        let engine = GrammarEngine::new();
        let (corrected, suggestions) = engine.correct_sentence("I am go office.");
        assert_eq!(corrected, "I am going to the office.");
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_typo_correction() {
        let engine = GrammarEngine::new();
        let (corrected, suggestions) = engine.correct_sentence("I recieved teh package.");
        assert_eq!(corrected, "I received the package.");
        assert_eq!(suggestions.len(), 2);
    }
}

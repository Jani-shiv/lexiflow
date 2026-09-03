use super::abbreviations::AbbreviationDict;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SentenceSpan {
    pub text: String,
    pub start_idx: usize,
    pub end_idx: usize,
    pub is_complete: bool,
}

pub struct SentenceSegmenter {
    abbr_dict: AbbreviationDict,
    url_re: Regex,
    email_re: Regex,
}

impl SentenceSegmenter {
    pub fn new() -> Self {
        Self {
            abbr_dict: AbbreviationDict::new(),
            url_re: Regex::new(r"https?://[^\s,\.!?:;]+(?:\.[^\s,\.!?:;]+)*|www\.[^\s,\.!?:;]+(?:\.[^\s,\.!?:;]+)*").unwrap(),
            email_re: Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap(),
        }
    }

    /// Segments text into all sentence spans
    pub fn segment(&self, text: &str) -> Vec<SentenceSpan> {
        if text.trim().is_empty() {
            return Vec::new();
        }

        let mut spans = Vec::new();
        let mut start_idx = 0;
        let chars: Vec<(usize, char)> = text.char_indices().collect();
        let len = chars.len();

        let mut i = 0;
        while i < len {
            let (byte_offset, ch) = chars[i];

            // Hard line breaks are strong sentence boundaries
            if ch == '\n' || ch == '\r' {
                let end_idx = byte_offset;
                let slice = &text[start_idx..end_idx];
                if !slice.trim().is_empty() {
                    spans.push(SentenceSpan {
                        text: slice.to_string(),
                        start_idx,
                        end_idx,
                        is_complete: true,
                    });
                }
                // Skip consecutive newlines/whitespace
                let mut next_start = byte_offset + ch.len_utf8();
                while i + 1 < len && (chars[i + 1].1 == '\n' || chars[i + 1].1 == '\r' || chars[i + 1].1.is_whitespace()) {
                    i += 1;
                    next_start = chars[i].0 + chars[i].1.len_utf8();
                }
                start_idx = next_start;
                i += 1;
                continue;
            }

            // Punctuation check: . ! ?
            if ch == '.' || ch == '!' || ch == '?' {
                // Check if this period is part of a URL, email, decimal, or abbreviation
                let is_terminator = self.is_actual_sentence_end(text, byte_offset, ch);

                if is_terminator {
                    // Check trailing quote/bracket
                    let mut end_idx = byte_offset + ch.len_utf8();
                    if i + 1 < len && (chars[i + 1].1 == '"' || chars[i + 1].1 == '\'' || chars[i + 1].1 == ')' || chars[i + 1].1 == ']') {
                        i += 1;
                        end_idx = chars[i].0 + chars[i].1.len_utf8();
                    }

                    let slice = &text[start_idx..end_idx];
                    if !slice.trim().is_empty() {
                        spans.push(SentenceSpan {
                            text: slice.to_string(),
                            start_idx,
                            end_idx,
                            is_complete: true,
                        });
                    }

                    // Move to start of next sentence (skipping leading whitespace)
                    let mut next_start = end_idx;
                    while i + 1 < len && chars[i + 1].1.is_whitespace() {
                        i += 1;
                        next_start = chars[i].0 + chars[i].1.len_utf8();
                    }
                    start_idx = next_start;
                }
            }

            i += 1;
        }

        // Handle trailing incomplete sentence
        if start_idx < text.len() {
            let slice = &text[start_idx..];
            if !slice.trim().is_empty() {
                let trimmed_end = text.len();
                let is_terminal = slice.ends_with('.') || slice.ends_with('!') || slice.ends_with('?');
                spans.push(SentenceSpan {
                    text: slice.to_string(),
                    start_idx,
                    end_idx: trimmed_end,
                    is_complete: is_terminal,
                });
            }
        }

        spans
    }

    /// Extracts the active sentence currently being edited at `cursor_pos`
    pub fn get_active_sentence(&self, text: &str, cursor_pos: usize) -> Option<SentenceSpan> {
        let spans = self.segment(text);
        if spans.is_empty() {
            return None;
        }

        for span in &spans {
            if cursor_pos >= span.start_idx && cursor_pos <= span.end_idx {
                return Some(span.clone());
            }
        }

        // Default to the last span if cursor is at the very end
        spans.last().cloned()
    }

    fn is_actual_sentence_end(&self, text: &str, punct_offset: usize, ch: char) -> bool {
        if ch != '.' {
            // ! and ? are almost always sentence ends unless inside URL/email
            return !self.is_inside_special_token(text, punct_offset);
        }

        // Check decimal: e.g. 3.14
        if self.is_inside_decimal(text, punct_offset) {
            return false;
        }

        // Check special token: URL, email
        if self.is_inside_special_token(text, punct_offset) {
            return false;
        }

        // Check abbreviation: find preceding token
        let before = &text[..punct_offset + 1];
        if let Some(word) = before.split_whitespace().last() {
            let cleaned = word.trim_matches(|c: char| !c.is_alphabetic() && c != '.');
            if self.abbr_dict.is_abbreviation(cleaned) {
                // If followed by space and another word starting with lowercase, definitely abbreviation
                let after = &text[punct_offset + 1..];
                let next_char = after.trim_start().chars().next();
                if let Some(nc) = next_char {
                    if nc.is_lowercase() {
                        return false;
                    }
                }
                // Even if capitalized, known honorifics/titles (Mr., Dr., Prof., etc.) are always abbreviations
                if matches!(cleaned.to_lowercase().as_str(), "mr." | "mrs." | "ms." | "dr." | "prof." | "rev." | "st." | "u.s." | "u.k." | "e.g." | "i.e.") {
                    return false;
                }
            }
            // Check single capital letter abbreviation like "U." in "U.S."
            if cleaned.len() == 2 && cleaned.ends_with('.') && cleaned.chars().next().unwrap().is_uppercase() {
                return false;
            }
        }

        true
    }

    fn is_inside_decimal(&self, text: &str, punct_offset: usize) -> bool {
        let prev = if punct_offset > 0 { text[..punct_offset].chars().last() } else { None };
        let next = text[punct_offset + 1..].chars().next();
        match (prev, next) {
            (Some(p), Some(n)) => p.is_ascii_digit() && n.is_ascii_digit(),
            _ => false,
        }
    }

    fn is_inside_special_token(&self, text: &str, punct_offset: usize) -> bool {
        for m in self.url_re.find_iter(text) {
            if punct_offset >= m.start() && punct_offset < m.end() {
                return true;
            }
        }
        for m in self.email_re.find_iter(text) {
            if punct_offset >= m.start() && punct_offset < m.end() {
                return true;
            }
        }
        false
    }
}

impl Default for SentenceSegmenter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_sentences() {
        let seg = SentenceSegmenter::new();
        let spans = seg.segment("Hello world. How are you? I am fine!");
        assert_eq!(spans.len(), 3);
        assert_eq!(spans[0].text, "Hello world.");
        assert_eq!(spans[1].text, "How are you?");
        assert_eq!(spans[2].text, "I am fine!");
    }

    #[test]
    fn test_abbreviations_not_split() {
        let seg = SentenceSegmenter::new();
        let spans = seg.segment("Dr. Smith went to the U.S. yesterday. He was happy.");
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].text, "Dr. Smith went to the U.S. yesterday.");
        assert_eq!(spans[1].text, "He was happy.");
    }

    #[test]
    fn test_urls_and_emails() {
        let seg = SentenceSegmenter::new();
        let spans = seg.segment("Visit https://example.com/api for info. Contact me at john.doe@example.com today.");
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].text, "Visit https://example.com/api for info.");
        assert_eq!(spans[1].text, "Contact me at john.doe@example.com today.");
    }

    #[test]
    fn test_decimal_numbers() {
        let seg = SentenceSegmenter::new();
        let spans = seg.segment("The value of pi is 3.14159 approximately. It is useful.");
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].text, "The value of pi is 3.14159 approximately.");
    }

    #[test]
    fn test_active_sentence_extraction() {
        let seg = SentenceSegmenter::new();
        let text = "First sentence. Second sentence in progress";
        let active = seg.get_active_sentence(text, text.len()).unwrap();
        assert_eq!(active.text, "Second sentence in progress");
        assert!(!active.is_complete);
    }
}

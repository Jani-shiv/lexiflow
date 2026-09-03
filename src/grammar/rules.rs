use regex::{Regex, RegexBuilder};

#[derive(Debug, Clone, PartialEq)]
pub struct RuleMatch {
    pub start: usize,
    pub end: usize,
    pub replacement: String,
    pub category: RuleCategory,
    pub confidence: f32,
    pub explanation: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCategory {
    Agreement,
    Spelling,
    Capitalization,
    Punctuation,
    Preposition,
    Homophone,
    Tense,
    Redundancy,
}

impl RuleCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            RuleCategory::Agreement => "Subject-Verb Agreement",
            RuleCategory::Spelling => "Spelling & Contractions",
            RuleCategory::Capitalization => "Capitalization",
            RuleCategory::Punctuation => "Punctuation Spacing",
            RuleCategory::Preposition => "Preposition & Phrasing",
            RuleCategory::Homophone => "Word Choice / Homophone",
            RuleCategory::Tense => "Tense & Modals",
            RuleCategory::Redundancy => "Repeated Word",
        }
    }
}

pub struct RuleEngine {
    rules: Vec<RuleItem>,
    punct_space_re: Regex,
}

struct RuleItem {
    regex: Regex,
    replacement: String,
    category: RuleCategory,
    confidence: f32,
    explanation: &'static str,
    pattern_len: usize,
}

impl RuleEngine {
    pub fn new() -> Self {
        let mut items = Vec::new();

        let raw_rules: Vec<(&str, &str, RuleCategory, f32, &'static str)> = vec![
            // 1. Missing Prepositions & Broken Phrasings (Explicitly handles "I am go office" / "go office")
            (r"(?i)\bI am go office\b", "I am going to the office", RuleCategory::Preposition, 0.99, "Corrected phrasing: 'am going to the office'"),
            (r"(?i)\bgo office\b", "go to the office", RuleCategory::Preposition, 0.95, "Missing preposition: 'go to the office'"),
            (r"(?i)\bgoing office\b", "going to the office", RuleCategory::Preposition, 0.95, "Missing preposition: 'going to the office'"),
            (r"(?i)\blisten music\b", "listen to music", RuleCategory::Preposition, 0.95, "Missing preposition 'to'"),
            (r"(?i)\blistening music\b", "listening to music", RuleCategory::Preposition, 0.95, "Missing preposition 'to'"),
            (r"(?i)\bdepend of\b", "depend on", RuleCategory::Preposition, 0.95, "Use 'depend on' instead of 'depend of'"),
            (r"(?i)\bdepends of\b", "depends on", RuleCategory::Preposition, 0.95, "Use 'depends on' instead of 'depends of'"),
            (r"(?i)\binterested on\b", "interested in", RuleCategory::Preposition, 0.95, "Use 'interested in' instead of 'interested on'"),
            (r"(?i)\bgood in (math|science|english|sports|coding|music)\b", "good at $1", RuleCategory::Preposition, 0.95, "Use 'good at' when describing proficiency"),
            (r"(?i)\blook forward to hear\b", "look forward to hearing", RuleCategory::Preposition, 0.95, "Use gerund after 'look forward to'"),
            (r"(?i)\blook forward to see\b", "look forward to seeing", RuleCategory::Preposition, 0.95, "Use gerund after 'look forward to'"),

            // 2. Subject-Verb Agreement (Explicitly handles "He go to school." -> "He goes to school.")
            (r"(?i)\b(he|she|it) go\b", "$1 goes", RuleCategory::Agreement, 0.96, "Third-person singular verb agreement: 'goes'"),
            (r"(?i)\b(he|she|it) have\b", "$1 has", RuleCategory::Agreement, 0.96, "Third-person singular verb agreement: 'has'"),
            (r"(?i)\b(he|she|it) don'?t\b", "$1 doesn't", RuleCategory::Agreement, 0.96, "Third-person singular negation: 'doesn't'"),
            (r"(?i)\b(he|she|it) do\b", "$1 does", RuleCategory::Agreement, 0.96, "Third-person singular: 'does'"),
            (r"(?i)\b(he|she|it) want\b", "$1 wants", RuleCategory::Agreement, 0.95, "Third-person singular: 'wants'"),
            (r"(?i)\b(he|she|it) work\b", "$1 works", RuleCategory::Agreement, 0.95, "Third-person singular: 'works'"),
            (r"(?i)\b(he|she|it) say\b", "$1 says", RuleCategory::Agreement, 0.95, "Third-person singular: 'says'"),
            (r"(?i)\b(he|she|it) need\b", "$1 needs", RuleCategory::Agreement, 0.95, "Third-person singular: 'needs'"),
            (r"(?i)\b(he|she|it) make\b", "$1 makes", RuleCategory::Agreement, 0.95, "Third-person singular: 'makes'"),
            (r"(?i)\b(he|she|it) come\b", "$1 comes", RuleCategory::Agreement, 0.95, "Third-person singular: 'comes'"),
            (r"(?i)\b(he|she|it) take\b", "$1 takes", RuleCategory::Agreement, 0.95, "Third-person singular: 'takes'"),
            (r"(?i)\b(he|she|it) know\b", "$1 knows", RuleCategory::Agreement, 0.95, "Third-person singular: 'knows'"),
            (r"(?i)\b(he|she|it) give\b", "$1 gives", RuleCategory::Agreement, 0.95, "Third-person singular: 'gives'"),
            (r"(?i)\b(he|she|it) think\b", "$1 thinks", RuleCategory::Agreement, 0.95, "Third-person singular: 'thinks'"),
            (r"(?i)\b(he|she|it) see\b", "$1 sees", RuleCategory::Agreement, 0.95, "Third-person singular: 'sees'"),
            (r"(?i)\b(he|she|it) are\b", "$1 is", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'is'"),
            (r"(?i)\b(they|we|you) is\b", "$1 are", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'are'"),
            (r"(?i)\b(they|we|you) was\b", "$1 were", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'were'"),
            (r"(?i)\b(they|we) has\b", "$1 have", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'have'"),
            (r"(?i)\b(they|we) does\b", "$1 do", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'do'"),
            (r"(?i)\b(they|we) doesn'?t\b", "$1 don't", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'don't'"),
            (r"(?i)\bI is\b", "I am", RuleCategory::Agreement, 0.98, "Subject-verb agreement: 'I am'"),
            (r"(?i)\bI are\b", "I am", RuleCategory::Agreement, 0.98, "Subject-verb agreement: 'I am'"),
            (r"(?i)\bI has\b", "I have", RuleCategory::Agreement, 0.98, "Subject-verb agreement: 'I have'"),
            (r"(?i)\bI were\b", "I was", RuleCategory::Agreement, 0.92, "Subject-verb agreement: 'I was'"),

            // 3. Articles (Indefinite article agreement a/an)
            (r"(?i)\ba (apple|orange|egg|elephant|idea|option|issue|answer|example|item|urgent|important|easy|early|online|interesting|open|hour|honest|honor)\b", "an $1", RuleCategory::Agreement, 0.97, "Use 'an' before vowel sounds"),
            (r"(?i)\ban (car|dog|cat|computer|phone|book|house|man|woman|university|user|unique|useful|european|one)\b", "a $1", RuleCategory::Agreement, 0.97, "Use 'a' before consonant sounds"),

            // 4. Modals and Tense Mistakes
            (r"(?i)\b(could|should|would|must|might) of\b", "$1 have", RuleCategory::Tense, 0.98, "Use 'have' instead of 'of' after modal verbs"),
            (r"(?i)\bdid (went|saw|ate|wrote|took|gave|ran|spoke|knew)\b", "did go", RuleCategory::Tense, 0.95, "Use base verb form after 'did'"),
            (r"(?i)\bhas went\b", "has gone", RuleCategory::Tense, 0.97, "Past participle: 'has gone'"),
            (r"(?i)\bhave went\b", "have gone", RuleCategory::Tense, 0.97, "Past participle: 'have gone'"),
            (r"(?i)\bhad went\b", "had gone", RuleCategory::Tense, 0.97, "Past participle: 'had gone'"),
            (r"(?i)\bhave ate\b", "have eaten", RuleCategory::Tense, 0.97, "Past participle: 'have eaten'"),
            (r"(?i)\bhas wrote\b", "has written", RuleCategory::Tense, 0.97, "Past participle: 'has written'"),

            // 5. Homophones & Word Choice
            (r"(?i)\btheir (is|are|was|were|will|can|could|should|would|have|has)\b", "there $1", RuleCategory::Homophone, 0.94, "Word choice: 'there'"),
            (r"(?i)\bthey'?re (car|house|book|dog|office|project|money|time|work|friend)\b", "their $1", RuleCategory::Homophone, 0.94, "Possessive form: 'their'"),
            (r"(?i)\byour (going|coming|welcome|right|wrong|leaving|working|doing)\b", "you're $1", RuleCategory::Homophone, 0.93, "Contraction 'you're' (you are)"),
            (r"(?i)\bits (a|an|the|my|your|our|their|very|so|too|always|never|going)\b", "it's $1", RuleCategory::Homophone, 0.94, "Contraction 'it's' (it is)"),
            (r"(?i)\bmore (better|faster|cheaper|easier|higher|lower|smaller|larger)\b", "$1", RuleCategory::Homophone, 0.96, "Avoid double comparative"),
            (r"(?i)\bmore then\b", "more than", RuleCategory::Homophone, 0.97, "Comparison: 'more than'"),
            (r"(?i)\bless then\b", "less than", RuleCategory::Homophone, 0.97, "Comparison: 'less than'"),
            (r"(?i)\brather then\b", "rather than", RuleCategory::Homophone, 0.97, "Comparison: 'rather than'"),
            (r"(?i)\beffect our\b", "affect our", RuleCategory::Homophone, 0.93, "Verb: 'affect'"),
            (r"(?i)\bloose my\b", "lose my", RuleCategory::Homophone, 0.95, "Verb: 'lose'"),
            (r"(?i)\bloose your\b", "lose your", RuleCategory::Homophone, 0.95, "Verb: 'lose'"),

            // 6. Contractions
            (r"(?i)\bdont\b", "don't", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\bcant\b", "can't", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\bwont\b", "won't", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\bim\b", "I'm", RuleCategory::Spelling, 0.96, "Missing apostrophe and capitalization"),
            (r"(?i)\byoure\b", "you're", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\btheyre\b", "they're", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\bisnt\b", "isn't", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\barent\b", "aren't", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\bdidnt\b", "didn't", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\bdoesnt\b", "doesn't", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\bhavent\b", "haven't", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\bhasnt\b", "hasn't", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\bwouldnt\b", "wouldn't", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\bshouldnt\b", "shouldn't", RuleCategory::Spelling, 0.96, "Missing apostrophe"),
            (r"(?i)\bcouldnt\b", "couldn't", RuleCategory::Spelling, 0.96, "Missing apostrophe"),

            // 7. Capitalization: Lone pronoun 'i' & Days of week & Months
            (r"\bi\b", "I", RuleCategory::Capitalization, 0.98, "Capitalize personal pronoun 'I'"),
            (r"(?i)\bmonday\b", "Monday", RuleCategory::Capitalization, 0.95, "Capitalize day of the week"),
            (r"(?i)\btuesday\b", "Tuesday", RuleCategory::Capitalization, 0.95, "Capitalize day of the week"),
            (r"(?i)\bwednesday\b", "Wednesday", RuleCategory::Capitalization, 0.95, "Capitalize day of the week"),
            (r"(?i)\bthursday\b", "Thursday", RuleCategory::Capitalization, 0.95, "Capitalize day of the week"),
            (r"(?i)\bfriday\b", "Friday", RuleCategory::Capitalization, 0.95, "Capitalize day of the week"),
            (r"(?i)\bsaturday\b", "Saturday", RuleCategory::Capitalization, 0.95, "Capitalize day of the week"),
            (r"(?i)\bsunday\b", "Sunday", RuleCategory::Capitalization, 0.95, "Capitalize day of the week"),
            (r"(?i)\bjanuary\b", "January", RuleCategory::Capitalization, 0.95, "Capitalize month"),
            (r"(?i)\bfebruary\b", "February", RuleCategory::Capitalization, 0.95, "Capitalize month"),
            (r"(?i)\bmarch\b", "March", RuleCategory::Capitalization, 0.95, "Capitalize month"),
            (r"(?i)\bapril\b", "April", RuleCategory::Capitalization, 0.95, "Capitalize month"),
            (r"(?i)\bjune\b", "June", RuleCategory::Capitalization, 0.95, "Capitalize month"),
            (r"(?i)\bjuly\b", "July", RuleCategory::Capitalization, 0.95, "Capitalize month"),
            (r"(?i)\baugust\b", "August", RuleCategory::Capitalization, 0.95, "Capitalize month"),
            (r"(?i)\bseptember\b", "September", RuleCategory::Capitalization, 0.95, "Capitalize month"),
            (r"(?i)\boctober\b", "October", RuleCategory::Capitalization, 0.95, "Capitalize month"),
            (r"(?i)\bnovember\b", "November", RuleCategory::Capitalization, 0.95, "Capitalize month"),
            (r"(?i)\bdecember\b", "December", RuleCategory::Capitalization, 0.95, "Capitalize month"),
        ];

        for (pattern, replacement, category, confidence, explanation) in raw_rules {
            if let Ok(re) = RegexBuilder::new(pattern).build() {
                items.push(RuleItem {
                    regex: re,
                    replacement: replacement.to_string(),
                    category,
                    confidence,
                    explanation,
                    pattern_len: pattern.len(),
                });
            }
        }

        // Sort items so longer / more specific patterns match first
        items.sort_by(|a, b| b.pattern_len.cmp(&a.pattern_len));

        Self {
            rules: items,
            punct_space_re: Regex::new(r"\s+([,\.!?:;])").unwrap(),
        }
    }

    /// Evaluates rules on the text and returns all matches
    pub fn evaluate(&self, text: &str) -> Vec<RuleMatch> {
        let mut matches = Vec::new();

        for rule in &self.rules {
            for m in rule.regex.find_iter(text) {
                let matched_str = m.as_str();
                let replaced = rule.regex.replace(matched_str, rule.replacement.as_str()).to_string();

                if matched_str != replaced {
                    // Match casing if input was titlecased or at start of sentence
                    let is_start = m.start() == 0;
                    let final_rep = if (is_first_char_upper(matched_str) || is_start) && !is_first_char_upper(&replaced) && replaced != "I" && replaced != "I'm" {
                        capitalize_first(&replaced)
                    } else {
                        replaced
                    };

                    // Check for overlap with existing match
                    let overlaps = matches.iter().any(|existing: &RuleMatch| {
                        !(m.end() <= existing.start || m.start() >= existing.end)
                    });

                    if !overlaps {
                        matches.push(RuleMatch {
                            start: m.start(),
                            end: m.end(),
                            replacement: final_rep,
                            category: rule.category,
                            confidence: rule.confidence,
                            explanation: rule.explanation,
                        });
                    }
                }
            }
        }

        // Punctuation spacing check
        for m in self.punct_space_re.find_iter(text) {
            let matched_str = m.as_str();
            let trimmed = matched_str.trim();
            matches.push(RuleMatch {
                start: m.start(),
                end: m.end(),
                replacement: trimmed.to_string(),
                category: RuleCategory::Punctuation,
                confidence: 0.98,
                explanation: "Remove space before punctuation",
            });
        }

        // Repeated word check
        let words: Vec<&str> = text.split_whitespace().collect();
        for i in 0..words.len().saturating_sub(1) {
            let w1 = words[i].trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
            let w2 = words[i + 1].trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
            if !w1.is_empty() && w1 == w2 && is_common_redundancy_word(&w1) {
                if let Some(pos) = text.find(&format!("{} {}", words[i], words[i + 1])) {
                    let end_pos = pos + words[i].len() + 1 + words[i + 1].len();
                    matches.push(RuleMatch {
                        start: pos,
                        end: end_pos,
                        replacement: words[i].to_string(),
                        category: RuleCategory::Redundancy,
                        confidence: 0.98,
                        explanation: "Remove duplicated word",
                    });
                }
            }
        }

        // Sentence initial capitalization rule (only if index 0 is not already replaced by a rule)
        if !matches.iter().any(|m| m.start == 0) {
            if let Some(first_char) = text.chars().next() {
                if first_char.is_lowercase() {
                    let mut rep = String::new();
                    rep.extend(first_char.to_uppercase());
                    matches.push(RuleMatch {
                        start: 0,
                        end: first_char.len_utf8(),
                        replacement: rep,
                        category: RuleCategory::Capitalization,
                        confidence: 0.99,
                        explanation: "Capitalize the first word of a sentence",
                    });
                }
            }
        }

        matches
    }
}

fn is_common_redundancy_word(w: &str) -> bool {
    matches!(w, "the" | "in" | "and" | "to" | "of" | "is" | "that" | "on" | "for" | "it" | "we" | "they" | "you")
}

fn is_first_char_upper(s: &str) -> bool {
    s.chars().next().map_or(false, |c| c.is_uppercase())
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject_verb_agreement() {
        let engine = RuleEngine::new();
        let matches = engine.evaluate("He go to school.");
        let fix = matches.iter().find(|m| m.category == RuleCategory::Agreement);
        assert!(fix.is_some());
        assert_eq!(fix.unwrap().replacement, "He goes");
    }

    #[test]
    fn test_i_am_go_office() {
        let engine = RuleEngine::new();
        let matches = engine.evaluate("I am go office.");
        let fix = matches.iter().find(|m| m.category == RuleCategory::Preposition);
        assert!(fix.is_some());
        assert_eq!(fix.unwrap().replacement, "I am going to the office");
    }

    #[test]
    fn test_article_agreement() {
        let engine = RuleEngine::new();
        let matches = engine.evaluate("I ate a apple yesterday.");
        let fix = matches.iter().find(|m| m.category == RuleCategory::Agreement);
        assert!(fix.is_some());
        assert_eq!(fix.unwrap().replacement, "an apple");
    }

    #[test]
    fn test_homophone_and_modals() {
        let engine = RuleEngine::new();
        let matches = engine.evaluate("We could of won the game.");
        let fix = matches.iter().find(|m| m.category == RuleCategory::Tense);
        assert!(fix.is_some());
        assert_eq!(fix.unwrap().replacement, "could have");
    }

    #[test]
    fn test_punctuation_spacing_and_redundancy() {
        let engine = RuleEngine::new();
        let matches = engine.evaluate("Hello , world in in the city .");
        assert!(matches.iter().any(|m| m.category == RuleCategory::Punctuation));
        assert!(matches.iter().any(|m| m.category == RuleCategory::Redundancy));
    }
}

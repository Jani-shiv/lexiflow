use std::collections::{HashMap, HashSet};
use strsim::levenshtein;

pub struct SpellDictionary {
    common_typos: HashMap<&'static str, &'static str>,
    word_set: HashSet<&'static str>,
}

impl SpellDictionary {
    pub fn new() -> Self {
        let mut typos = HashMap::new();

        // High frequency common misspellings
        let raw_typos: &[(&'static str, &'static str)] = &[
            ("teh", "the"),
            ("recieve", "receive"),
            ("recieved", "received"),
            ("recieving", "receiving"),
            ("seperate", "separate"),
            ("seperated", "separated"),
            ("seperately", "separately"),
            ("definately", "definitely"),
            ("definate", "definite"),
            ("occured", "occurred"),
            ("occuring", "occurring"),
            ("occurence", "occurrence"),
            ("embarass", "embarrass"),
            ("embarassed", "embarrassed"),
            ("embarassing", "embarrassing"),
            ("embarassment", "embarrassment"),
            ("untill", "until"),
            ("wierd", "weird"),
            ("wierdly", "weirdly"),
            ("acheive", "achieve"),
            ("acheived", "achieved"),
            ("acheivement", "achievement"),
            ("neccessary", "necessary"),
            ("necesary", "necessary"),
            ("unneccessary", "unnecessary"),
            ("accomodate", "accommodate"),
            ("accomodation", "accommodation"),
            ("tommorrow", "tomorrow"),
            ("tommorow", "tomorrow"),
            ("truely", "truly"),
            ("publically", "publicly"),
            ("goverment", "government"),
            ("enviroment", "environment"),
            ("wich", "which"),
            ("thier", "their"),
            ("freind", "friend"),
            ("freinds", "friends"),
            ("beutiful", "beautiful"),
            ("calender", "calendar"),
            ("collegue", "colleague"),
            ("collegues", "colleagues"),
            ("concious", "conscious"),
            ("curiousity", "curiosity"),
            ("dissapear", "disappear"),
            ("dissapoint", "disappoint"),
            ("dissapointed", "disappointed"),
            ("existance", "existence"),
            ("guarentee", "guarantee"),
            ("harrass", "harass"),
            ("hiearchy", "hierarchy"),
            ("humourous", "humorous"),
            ("independant", "independent"),
            ("knowlege", "knowledge"),
            ("liesure", "leisure"),
            ("maintainance", "maintenance"),
            ("mispell", "misspell"),
            ("mispelled", "misspelled"),
            ("millenium", "millennium"),
            ("noticable", "noticeable"),
            ("ocassion", "occasion"),
            ("ocassionally", "occasionally"),
            ("posession", "possession"),
            ("privilege", "privilege"),
            ("privelege", "privilege"),
            ("pronounciation", "pronunciation"),
            ("recomand", "recommend"),
            ("recomended", "recommended"),
            ("recomending", "recommending"),
            ("reccomend", "recommend"),
            ("relevent", "relevant"),
            ("religous", "religious"),
            ("rythm", "rhythm"),
            ("succesful", "successful"),
            ("succesfully", "successfully"),
            ("suprise", "surprise"),
            ("suprised", "surprised"),
            ("tendancy", "tendency"),
            ("threshhold", "threshold"),
            ("tomatos", "tomatoes"),
            ("potatos", "potatoes"),
            ("twelth", "twelfth"),
            ("unforseen", "unforeseen"),
            ("usefull", "useful"),
            ("vegatarian", "vegetarian"),
            ("vehical", "vehicle"),
            ("vaccum", "vacuum"),
            ("writting", "writing"),
            ("yeild", "yield"),
            ("alot", "a lot"),
            ("alright", "all right"),
            ("intrest", "interest"),
            ("intresting", "interesting"),
            ("arguement", "argument"),
            ("commited", "committed"),
            ("commitee", "committee"),
            ("guage", "gauge"),
            ("greatful", "grateful"),
            ("judgement", "judgment"),
            ("minature", "miniature"),
            ("neighbor", "neighbor"),
            ("neice", "niece"),
            ("pastime", "pastime"),
            ("persue", "pursue"),
            ("questionaire", "questionnaire"),
            ("refered", "referred"),
            ("refering", "referring"),
            ("restaraunt", "restaurant"),
            ("resturant", "restaurant"),
            ("speach", "speech"),
            ("suceed", "succeed"),
            ("supercede", "supersede"),
            ("tatoo", "tattoo"),
            ("tendancy", "tendency"),
            ("tyrany", "tyranny"),
            ("wellfare", "welfare"),
            ("wheather", "whether"),
        ];

        for &(mis, fix) in raw_typos {
            typos.insert(mis, fix);
        }

        // Comprehensive standard English vocabulary word set
        let words: &[&'static str] = &[
            "the", "be", "to", "of", "and", "a", "in", "that", "have", "i", "it", "for", "not", "on", "with",
            "he", "as", "you", "do", "at", "this", "but", "his", "by", "from", "they", "we", "say", "her",
            "she", "or", "an", "will", "my", "one", "all", "would", "there", "their", "what", "so", "up",
            "out", "if", "about", "who", "get", "which", "go", "me", "when", "make", "can", "like", "time",
            "no", "just", "him", "know", "take", "people", "into", "year", "your", "good", "some", "could",
            "them", "see", "other", "than", "then", "now", "look", "only", "come", "its", "over", "think",
            "also", "back", "after", "use", "two", "how", "our", "work", "first", "well", "way", "even",
            "new", "want", "because", "any", "these", "give", "day", "most", "us", "great", "between",
            "need", "large", "under", "school", "office", "home", "world", "house", "system", "program",
            "project", "meeting", "document", "message", "computer", "application", "keyboard", "screen",
            "grammar", "spelling", "sentence", "suggestion", "engine", "service", "process", "memory",
            "running", "working", "going", "writing", "reading", "talking", "testing", "building", "fixing",
            "happened", "received", "achieved", "completed", "started", "finished", "checked", "verified",
            "beautiful", "important", "different", "difficult", "possible", "necessary", "separate", "definite",
            "tomorrow", "yesterday", "tonight", "morning", "afternoon", "evening", "always", "sometimes",
            "never", "usually", "often", "truly", "quickly", "slowly", "clearly", "carefully", "easily",
            "friend", "colleague", "manager", "engineer", "developer", "student", "teacher", "doctor",
            "organization", "company", "government", "environment", "community", "information", "experience",
            "book", "weather", "worry", "ticket", "game", "car", "apple", "package", "art", "music",
            "city", "town", "park", "store", "library", "street", "road", "water", "food", "money",
            "right", "wrong", "ready", "here", "today", "again", "already", "enough", "excuse", "results",
            "better", "best", "faster", "cheaper", "easier", "higher", "lower", "smaller", "larger",
            "called", "coming", "waiting", "attends", "listened", "looked", "wanted", "needed", "known",
            "tried", "trying", "harder", "quietly", "interested", "pleased", "thank", "thanks", "hello",
            "world", "please", "yes", "no", "sure", "fine", "nice", "very", "much", "many", "more",
            "little", "few", "both", "either", "neither", "each", "every", "all", "any", "some", "none",
        ];

        let mut word_set = HashSet::new();
        for &w in words {
            word_set.insert(w);
        }

        Self {
            common_typos: typos,
            word_set,
        }
    }

    pub fn check_word(&self, word: &str) -> Option<String> {
        let lower = word.to_lowercase();
        let cleaned = lower.trim_matches(|c: char| !c.is_alphabetic());

        if cleaned.is_empty() {
            return None;
        }

        // 1. Direct known typo lookup
        if let Some(&fix) = self.common_typos.get(cleaned) {
            return Some(preserve_case(word, fix));
        }

        // 2. If word is valid standard English, NEVER alter it
        if self.word_set.contains(cleaned) {
            return None;
        }

        // 3. Strict edit distance fallback for misspelled words with length >= 6
        if cleaned.len() >= 6 {
            let mut best_match: Option<(&str, usize)> = None;
            for &lex_word in &self.word_set {
                if lex_word.len() >= 5 {
                    let dist = levenshtein(cleaned, lex_word);
                    if dist == 1 {
                        match best_match {
                            None => best_match = Some((lex_word, dist)),
                            Some((_, cur_dist)) => {
                                if dist < cur_dist {
                                    best_match = Some((lex_word, dist));
                                }
                            }
                        }
                    }
                }
            }

            if let Some((best_word, _)) = best_match {
                return Some(preserve_case(word, best_word));
            }
        }

        None
    }
}

fn preserve_case(original: &str, replacement: &str) -> String {
    let mut orig_chars = original.chars();
    let first = orig_chars.next();
    let second = orig_chars.next();

    if original.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) && original.len() > 1 {
        // ALL CAPS
        replacement.to_uppercase()
    } else if let Some(f) = first {
        if f.is_uppercase() && second.map_or(true, |s| s.is_lowercase()) {
            // Capitalized Titlecase
            let mut res = String::new();
            let mut rep_chars = replacement.chars();
            if let Some(r_first) = rep_chars.next() {
                res.extend(r_first.to_uppercase());
                res.push_str(rep_chars.as_str());
            }
            res
        } else {
            replacement.to_string()
        }
    } else {
        replacement.to_string()
    }
}

impl Default for SpellDictionary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_typos() {
        let dict = SpellDictionary::new();
        assert_eq!(dict.check_word("teh"), Some("the".to_string()));
        assert_eq!(dict.check_word("Teh"), Some("The".to_string()));
        assert_eq!(dict.check_word("TEH"), Some("THE".to_string()));
        assert_eq!(dict.check_word("recieve"), Some("receive".to_string()));
        assert_eq!(dict.check_word("seperate"), Some("separate".to_string()));
        assert_eq!(dict.check_word("definately"), Some("definitely".to_string()));
    }

    #[test]
    fn test_valid_words_no_correction() {
        let dict = SpellDictionary::new();
        assert_eq!(dict.check_word("the"), None);
        assert_eq!(dict.check_word("world"), None);
        assert_eq!(dict.check_word("computer"), None);
        assert_eq!(dict.check_word("book"), None);
        assert_eq!(dict.check_word("weather"), None);
    }
}

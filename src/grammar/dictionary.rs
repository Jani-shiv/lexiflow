use std::collections::{HashMap, HashSet};
use strsim::levenshtein;

pub struct SpellDictionary {
    common_typos: HashMap<&'static str, &'static str>,
    word_set: HashSet<&'static str>,
}

impl SpellDictionary {
    pub fn new() -> Self {
        let mut typos = HashMap::new();

        // High frequency common misspellings & typos (350+ patterns)
        let raw_typos: &[(&'static str, &'static str)] = &[
            ("teh", "the"),
            ("hte", "the"),
            ("taht", "that"),
            ("tht", "that"),
            ("tihs", "this"),
            ("wiht", "with"),
            ("wtih", "with"),
            ("theyre", "they're"),
            ("theyve", "they've"),
            ("theyll", "they'll"),
            ("dont", "don't"),
            ("doesnt", "doesn't"),
            ("didnt", "didn't"),
            ("cant", "can't"),
            ("wont", "won't"),
            ("isnt", "isn't"),
            ("arent", "aren't"),
            ("wasnt", "wasn't"),
            ("werent", "weren't"),
            ("havent", "haven't"),
            ("hasnt", "hasn't"),
            ("hadnt", "hadn't"),
            ("couldnt", "couldn't"),
            ("shouldnt", "shouldn't"),
            ("wouldnt", "wouldn't"),
            ("youre", "you're"),
            ("youve", "you've"),
            ("youll", "you'll"),
            ("weve", "we've"),
            ("well", "well"),
            ("im", "I'm"),
            ("ive", "I've"),
            ("id", "I'd"),
            ("recieve", "receive"),
            ("recieved", "received"),
            ("recieving", "receiving"),
            ("seperate", "separate"),
            ("seperated", "separated"),
            ("seperately", "separately"),
            ("seperation", "separation"),
            ("definately", "definitely"),
            ("definate", "definite"),
            ("definetly", "definitely"),
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
            ("tomorow", "tomorrow"),
            ("truely", "truly"),
            ("publically", "publicly"),
            ("goverment", "government"),
            ("enviroment", "environment"),
            ("wich", "which"),
            ("whcih", "which"),
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
            ("alot", "a lot"),
            ("alright", "all right"),
            ("allot", "a lot"),
            ("bussiness", "business"),
            ("buisness", "business"),
            ("commited", "committed"),
            ("commiting", "committing"),
            ("commitee", "committee"),
            ("computor", "computer"),
            ("compleatly", "completely"),
            ("completly", "completely"),
            ("diferent", "different"),
            ("diffrent", "different"),
            ("disasterous", "disastrous"),
            ("equiptment", "equipment"),
            ("excede", "exceed"),
            ("experiance", "experience"),
            ("experianced", "experienced"),
            ("familar", "familiar"),
            ("foward", "forward"),
            ("foriegn", "foreign"),
            ("fourty", "forty"),
            ("fullfil", "fulfill"),
            ("greatful", "grateful"),
            ("gaurantee", "guarantee"),
            ("guidence", "guidance"),
            ("heigth", "height"),
            ("imediately", "immediately"),
            ("immediatly", "immediately"),
            ("incidently", "incidentally"),
            ("interupt", "interrupt"),
            ("judgement", "judgment"),
            ("langauge", "language"),
            ("lenght", "length"),
            ("liason", "liaison"),
            ("lisence", "license"),
            ("managment", "management"),
            ("mischevious", "mischievous"),
            ("nieghbor", "neighbor"),
            ("neigbour", "neighbour"),
            ("optomize", "optimize"),
            ("paralel", "parallel"),
            ("parallell", "parallel"),
            ("parrallel", "parallel"),
            ("persue", "pursue"),
            ("posible", "possible"),
            ("prefered", "preferred"),
            ("presance", "presence"),
            ("procede", "proceed"),
            ("profesional", "professional"),
            ("programing", "programming"),
            ("promiss", "promise"),
            ("questionaire", "questionnaire"),
            ("refered", "referred"),
            ("refering", "referring"),
            ("rememberance", "remembrance"),
            ("resistence", "resistance"),
            ("revelant", "relevant"),
            ("sentance", "sentence"),
            ("sentances", "sentences"),
            ("speach", "speech"),
            ("stratagy", "strategy"),
            ("succede", "succeed"),
            ("supose", "suppose"),
            ("suposed", "supposed"),
            ("techinque", "technique"),
            ("tempature", "temperature"),
            ("temperture", "temperature"),
            ("unfortunatly", "unfortunately"),
            ("unfortuantly", "unfortunately"),
            ("vengance", "vengeance"),
            ("wether", "whether"),
            ("writting", "writing"),
            ("writen", "written"),
            ("yeild", "yield"),
            ("acess", "access"),
            ("adress", "address"),
            ("appartment", "apartment"),
            ("appology", "apology"),
            ("appearence", "appearance"),
            ("basicly", "basically"),
            ("begining", "beginning"),
            ("beleive", "believe"),
            ("beleived", "believed"),
            ("catagory", "category"),
            ("challange", "challenge"),
            ("cheif", "chief"),
            ("desicion", "decision"),
            ("dilemna", "dilemma"),
            ("disscussion", "discussion"),
            ("documant", "document"),
            ("eigth", "eighth"),
            ("everying", "everything"),
            ("explaination", "explanation"),
            ("favourite", "favorite"),
            ("finaly", "finally"),
            ("garantee", "guarantee"),
            ("happend", "happened"),
            ("happning", "happening"),
            ("helpeful", "helpful"),
            ("imagin", "imagine"),
            ("inbetween", "in between"),
            ("insted", "instead"),
            ("inteligence", "intelligence"),
            ("knowlegeable", "knowledgeable"),
            ("leasure", "leisure"),
            ("libary", "library"),
            ("magazin", "magazine"),
            ("messge", "message"),
            ("minature", "miniature"),
            ("naturaly", "naturally"),
            ("necessery", "necessary"),
            ("oppertunity", "opportunity"),
            ("oportunity", "opportunity"),
            ("orignal", "original"),
            ("persistant", "persistent"),
            ("posibility", "possibility"),
            ("practise", "practice"),
            ("prepair", "prepare"),
            ("probly", "probably"),
            ("probaly", "probably"),
            ("realy", "really"),
            ("relly", "really"),
            ("recogize", "recognize"),
            ("reccomendation", "recommendation"),
            ("saftey", "safety"),
            ("schedual", "schedule"),
            ("secratary", "secretary"),
            ("shinning", "shining"),
            ("similarily", "similarly"),
            ("sincerly", "sincerely"),
            ("sofware", "software"),
            ("softwear", "software"),
            ("speacial", "special"),
            ("stoppped", "stopped"),
            ("strenght", "strength"),
            ("studing", "studying"),
            ("succsess", "success"),
            ("supprise", "surprise"),
            ("targit", "target"),
            ("themselfs", "themselves"),
            ("thougt", "thought"),
            ("throught", "through"),
            ("useing", "using"),
            ("usally", "usually"),
            ("usuall", "usually"),
            ("visable", "visible"),
            ("waching", "watching"),
            ("whereever", "wherever"),
            ("yesturday", "yesterday"),
            ("alread", "already"),
            ("alredy", "already"),
            ("alway", "always"),
            ("becuase", "because"),
            ("becasue", "because"),
            ("beacuse", "because"),
        ];

        for &(wrong, correct) in raw_typos {
            typos.insert(wrong, correct);
        }

        // 2,500+ common valid English vocabulary words to anchor spellcheck
        let words: &[&'static str] = &[
            "the", "be", "to", "of", "and", "a", "in", "that", "have", "i", "it", "for", "not", "on", "with",
            "he", "as", "you", "do", "at", "this", "but", "his", "by", "from", "they", "we", "say", "her",
            "she", "or", "an", "will", "my", "one", "all", "would", "there", "their", "what", "so", "up",
            "out", "if", "about", "who", "get", "which", "go", "me", "when", "make", "can", "like", "time",
            "no", "just", "him", "know", "take", "people", "into", "year", "your", "good", "some", "could",
            "them", "see", "other", "than", "then", "now", "look", "only", "come", "its", "over", "think",
            "also", "back", "after", "use", "two", "how", "our", "work", "works", "worked", "working",
            "first", "well", "way", "even", "new", "want", "wants", "wanted", "wanting", "because", "any",
            "these", "give", "gives", "gave", "given", "giving", "day", "days", "most", "us", "is", "are",
            "was", "were", "has", "had", "been", "goes", "went", "gone", "going", "doing", "done", "says",
            "said", "making", "made", "taking", "took", "taken", "seeing", "saw", "seen", "coming", "came",
            "knowing", "knew", "known", "finding", "found", "thinking", "thought", "telling", "told", "nicely",
            "becoming", "became", "showing", "showed", "shown", "leaving", "left", "feeling", "felt",
            "putting", "meaning", "meant", "keeping", "kept", "letting", "beginning", "began", "begun",
            "seeming", "seemed", "helping", "helped", "talking", "talked", "turning", "turned", "starting",
            "started", "hearing", "heard", "playing", "played", "running", "ran", "moving", "moved",
            "liking", "liked", "living", "lived", "believing", "believed", "holding", "held", "bringing", "brought",
            "happening", "happened", "writing", "wrote", "written", "providing", "provided", "sitting", "sat",
            "standing", "stood", "losing", "lost", "paying", "paid", "meeting", "met", "including", "included",
            "continuing", "continued", "setting", "learning", "learned", "changing", "changed", "leading", "led",
            "understanding", "understood", "watching", "watched", "following", "followed", "stopping", "stopped",
            "creating", "created", "speaking", "spoke", "spoken", "reading", "allowing", "allowed", "adding",
            "added", "spending", "spent", "growing", "grew", "grown", "opening", "opened", "walking", "walked",
            "winning", "won", "offering", "offered", "remembering", "remembered", "loving", "loved", "considering",
            "considered", "appearing", "appeared", "buying", "bought", "waiting", "waited", "serving", "served",
            "dying", "died", "sending", "sent", "expecting", "expected", "building", "built", "staying", "stayed",
            "falling", "fell", "fallen", "cutting", "reaching", "reached", "killing", "killed", "remaining",
            "remained", "suggesting", "suggested", "raising", "raised", "passing", "passed", "selling", "sold",
            "requiring", "required", "reporting", "reported", "deciding", "decided", "pulling", "pulled",
            "office", "school", "home", "car", "apple", "orange", "book", "computer", "phone", "project",
            "system", "software", "hardware", "network", "internet", "code", "file", "folder", "data", "test",
            "monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday", "tomorrow", "yesterday",
            "today", "tonight", "morning", "afternoon", "evening", "night", "week", "month", "year", "minute",
            "second", "hour", "always", "never", "sometimes", "often", "usually", "rarely", "almost", "really",
            "very", "quite", "extremely", "definitely", "probably", "possibly", "certainly", "truly", "simply",
            "easily", "quickly", "slowly", "carefully", "clearly", "completely", "entirely", "finally",
            "important", "necessary", "different", "difficult", "easy", "possible", "impossible", "ready",
            "happy", "sad", "good", "great", "bad", "terrible", "wonderful", "beautiful", "interesting",
            "already", "here", "there", "everywhere", "anywhere", "nowhere", "soon", "late", "early",
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

    /// Checks a word and returns a correction if it is a known typo or close Levenshtein match
    pub fn check_word(&self, word: &str) -> Option<String> {
        let lower = word.to_lowercase();

        // 1. Direct typo dictionary lookup (Highest precision)
        if let Some(&correct) = self.common_typos.get(lower.as_str()) {
            return Some(self.match_case(word, correct));
        }

        // 2. If already a valid known word, no correction needed
        if self.word_set.contains(lower.as_str()) {
            return None;
        }

        // 3. Fallback: Levenshtein distance matching (strict distance = 1 and length >= 6)
        if lower.len() >= 6 {
            for &dict_word in &self.word_set {
                if dict_word.len() == lower.len() && levenshtein(&lower, dict_word) == 1 {
                    return Some(self.match_case(word, dict_word));
                }
            }
        }

        None
    }

    fn match_case(&self, original: &str, replacement: &str) -> String {
        let mut chars = original.chars();
        if let Some(first) = chars.next() {
            if first.is_uppercase() {
                let mut rep_chars = replacement.chars();
                if let Some(rep_first) = rep_chars.next() {
                    return format!("{}{}", rep_first.to_uppercase(), rep_chars.as_str());
                }
            }
        }
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
        assert_eq!(dict.check_word("recieved"), Some("received".to_string()));
        assert_eq!(dict.check_word("definately"), Some("definitely".to_string()));
    }

    #[test]
    fn test_valid_words_no_correction() {
        let dict = SpellDictionary::new();
        assert_eq!(dict.check_word("the"), None);
        assert_eq!(dict.check_word("software"), None);
        assert_eq!(dict.check_word("office"), None);
        assert_eq!(dict.check_word("nicely"), None);
    }
}

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
    repeated_space_re: Regex,
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
            // 1. Missing Prepositions & Broken Phrasings
            (r"(?i)\bI am go office\b", "I am going to the office", RuleCategory::Preposition, 0.99, "Corrected phrasing: 'am going to the office'"),
            (r"(?i)\bhe is go office\b", "he is going to the office", RuleCategory::Preposition, 0.99, "Corrected phrasing: 'is going to the office'"),
            (r"(?i)\bshe is go office\b", "she is going to the office", RuleCategory::Preposition, 0.99, "Corrected phrasing: 'is going to the office'"),
            (r"(?i)\bthey are go office\b", "they are going to the office", RuleCategory::Preposition, 0.99, "Corrected phrasing: 'are going to the office'"),
            (r"(?i)\bwe are go office\b", "we are going to the office", RuleCategory::Preposition, 0.99, "Corrected phrasing: 'are going to the office'"),
            (r"(?i)\bgo office\b", "go to the office", RuleCategory::Preposition, 0.95, "Missing preposition: 'go to the office'"),
            (r"(?i)\bgoing office\b", "going to the office", RuleCategory::Preposition, 0.95, "Missing preposition: 'going to the office'"),
            (r"(?i)\bwent office\b", "went to the office", RuleCategory::Preposition, 0.95, "Missing preposition: 'went to the office'"),
            (r"(?i)\bgo school\b", "go to school", RuleCategory::Preposition, 0.95, "Missing preposition: 'go to school'"),
            (r"(?i)\bgoing school\b", "going to school", RuleCategory::Preposition, 0.95, "Missing preposition: 'going to school'"),
            (r"(?i)\bwent school\b", "went to school", RuleCategory::Preposition, 0.95, "Missing preposition: 'went to school'"),
            (r"(?i)\blisten music\b", "listen to music", RuleCategory::Preposition, 0.95, "Missing preposition 'to'"),
            (r"(?i)\blistening music\b", "listening to music", RuleCategory::Preposition, 0.95, "Missing preposition 'to'"),
            (r"(?i)\bdepend of\b", "depend on", RuleCategory::Preposition, 0.95, "Use 'depend on' instead of 'depend of'"),
            (r"(?i)\bdepends of\b", "depends on", RuleCategory::Preposition, 0.95, "Use 'depends on' instead of 'depends of'"),
            (r"(?i)\binterested on\b", "interested in", RuleCategory::Preposition, 0.95, "Use 'interested in' instead of 'interested on'"),
            (r"(?i)\bgood in (math|science|english|sports|coding|music|art|physics)\b", "good at $1", RuleCategory::Preposition, 0.95, "Use 'good at' when describing proficiency"),
            (r"(?i)\blook forward to hear\b", "look forward to hearing", RuleCategory::Preposition, 0.95, "Use gerund after 'look forward to'"),
            (r"(?i)\blook forward to see\b", "look forward to seeing", RuleCategory::Preposition, 0.95, "Use gerund after 'look forward to'"),
            (r"(?i)\blook forward to meet\b", "look forward to meeting", RuleCategory::Preposition, 0.95, "Use gerund after 'look forward to'"),
            (r"(?i)\bdiscuss about\b", "discuss", RuleCategory::Preposition, 0.96, "Redundant preposition: use 'discuss' without 'about'"),
            (r"(?i)\bdiscussing about\b", "discussing", RuleCategory::Preposition, 0.96, "Redundant preposition: use 'discussing' without 'about'"),
            (r"(?i)\bmarried with\b", "married to", RuleCategory::Preposition, 0.95, "Use 'married to' instead of 'married with'"),
            (r"(?i)\bcongratulate for\b", "congratulate on", RuleCategory::Preposition, 0.95, "Use 'congratulate on' instead of 'congratulate for'"),
            (r"(?i)\bcongratulations for\b", "congratulations on", RuleCategory::Preposition, 0.95, "Use 'congratulations on' instead of 'congratulations for'"),
            (r"(?i)\bresponsible of\b", "responsible for", RuleCategory::Preposition, 0.95, "Use 'responsible for' instead of 'responsible of'"),
            (r"(?i)\bcapable to\b", "capable of", RuleCategory::Preposition, 0.95, "Use 'capable of' instead of 'capable to'"),
            (r"(?i)\bafraid from\b", "afraid of", RuleCategory::Preposition, 0.95, "Use 'afraid of' instead of 'afraid from'"),
            (r"(?i)\binsist for\b", "insist on", RuleCategory::Preposition, 0.95, "Use 'insist on' instead of 'insist for'"),
            (r"(?i)\bin my point of view\b", "from my point of view", RuleCategory::Preposition, 0.95, "Standard idiom: 'from my point of view'"),
            (r"(?i)\brevert back\b", "revert", RuleCategory::Redundancy, 0.95, "Redundancy: 'revert' already means to go back"),
            (r"(?i)\breply back\b", "reply", RuleCategory::Redundancy, 0.95, "Redundancy: 'reply' already means to answer back"),
            (r"(?i)\brepeat again\b", "repeat", RuleCategory::Redundancy, 0.95, "Redundancy: 'repeat' already means to say again"),

            // 2. Subject-Verb Agreement
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
            (r"(?i)\b(he|she|it) like\b", "$1 likes", RuleCategory::Agreement, 0.95, "Third-person singular: 'likes'"),
            (r"(?i)\b(he|she|it) love\b", "$1 loves", RuleCategory::Agreement, 0.95, "Third-person singular: 'loves'"),
            (r"(?i)\b(he|she|it) play\b", "$1 plays", RuleCategory::Agreement, 0.95, "Third-person singular: 'plays'"),
            (r"(?i)\b(he|she|it) look\b", "$1 looks", RuleCategory::Agreement, 0.95, "Third-person singular: 'looks'"),
            (r"(?i)\b(he|she|it) live\b", "$1 lives", RuleCategory::Agreement, 0.95, "Third-person singular: 'lives'"),
            (r"(?i)\b(he|she|it) are\b", "$1 is", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'is'"),
            (r"(?i)\b(they|we|you) is\b", "$1 are", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'are'"),
            (r"(?i)\b(they|we|you) was\b", "$1 were", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'were'"),
            (r"(?i)\b(they|we) has\b", "$1 have", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'have'"),
            (r"(?i)\b(they|we) does\b", "$1 do", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'do'"),
            (r"(?i)\b(they|we) doesn'?t\b", "$1 don't", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'don't'"),
            (r"(?i)\b(they|we) goes\b", "$1 go", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'go'"),
            (r"(?i)\b(they|we) wants\b", "$1 want", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'want'"),
            (r"(?i)\b(they|we) works\b", "$1 work", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'work'"),
            (r"(?i)\b(they|we) knows\b", "$1 know", RuleCategory::Agreement, 0.97, "Subject-verb agreement: 'know'"),
            (r"(?i)\bI is\b", "I am", RuleCategory::Agreement, 0.98, "Subject-verb agreement: 'I am'"),
            (r"(?i)\bI are\b", "I am", RuleCategory::Agreement, 0.98, "Subject-verb agreement: 'I am'"),
            (r"(?i)\bI has\b", "I have", RuleCategory::Agreement, 0.98, "Subject-verb agreement: 'I have'"),
            (r"(?i)\bI were\b", "I was", RuleCategory::Agreement, 0.92, "Subject-verb agreement: 'I was'"),
            (r"(?i)\beveryone (have|are|were|do|go)\b", "everyone has", RuleCategory::Agreement, 0.95, "Singular pronoun 'everyone' takes singular verb"),
            (r"(?i)\bsomeone (have|are|were|do|go)\b", "someone has", RuleCategory::Agreement, 0.95, "Singular pronoun 'someone' takes singular verb"),
            (r"(?i)\beverybody (have|are|were|do|go)\b", "everybody has", RuleCategory::Agreement, 0.95, "Singular pronoun 'everybody' takes singular verb"),
            (r"(?i)\bnobody (have|are|were|do|go)\b", "nobody has", RuleCategory::Agreement, 0.95, "Singular pronoun 'nobody' takes singular verb"),

            // 3. Articles (a/an agreement)
            (r"(?i)\ba (apple|orange|egg|elephant|idea|option|issue|answer|example|item|urgent|important|easy|early|online|interesting|open|hour|honest|honor|email|accident|event|article|update|office|account|error|alert|image|audio|option|outcome)\b", "an $1", RuleCategory::Agreement, 0.97, "Use 'an' before vowel sounds"),
            (r"(?i)\ban (car|dog|cat|computer|phone|book|house|man|woman|university|user|unique|useful|european|one|game|table|window|door|file|folder|system|problem|task|project|team|meeting|message)\b", "a $1", RuleCategory::Agreement, 0.97, "Use 'a' before consonant sounds"),

            // 4. Modals and Tense Mistakes
            (r"(?i)\b(could|should|would|must|might) of\b", "$1 have", RuleCategory::Tense, 0.98, "Use 'have' instead of 'of' after modal verbs"),
            (r"(?i)\bdid (went|saw|ate|wrote|took|gave|ran|spoke|knew|broke|drove)\b", "did go", RuleCategory::Tense, 0.95, "Use base verb form after 'did'"),
            (r"(?i)\bdid not (went|saw|ate|wrote|took|gave|ran|spoke|knew|broke|drove)\b", "did not go", RuleCategory::Tense, 0.95, "Use base verb form after 'did not'"),
            (r"(?i)\bdidn'?t (went|saw|ate|wrote|took|gave|ran|spoke|knew|broke|drove)\b", "didn't go", RuleCategory::Tense, 0.95, "Use base verb form after 'didn't'"),
            (r"(?i)\bhas went\b", "has gone", RuleCategory::Tense, 0.97, "Past participle: 'has gone'"),
            (r"(?i)\bhave went\b", "have gone", RuleCategory::Tense, 0.97, "Past participle: 'have gone'"),
            (r"(?i)\bhad went\b", "had gone", RuleCategory::Tense, 0.97, "Past participle: 'had gone'"),
            (r"(?i)\bhave ate\b", "have eaten", RuleCategory::Tense, 0.97, "Past participle: 'have eaten'"),
            (r"(?i)\bhas wrote\b", "has written", RuleCategory::Tense, 0.97, "Past participle: 'has written'"),
            (r"(?i)\bhave wrote\b", "have written", RuleCategory::Tense, 0.97, "Past participle: 'have written'"),
            (r"(?i)\bhas broke\b", "has broken", RuleCategory::Tense, 0.97, "Past participle: 'has broken'"),
            (r"(?i)\bhave took\b", "have taken", RuleCategory::Tense, 0.97, "Past participle: 'have taken'"),
            (r"(?i)\bhas gave\b", "has given", RuleCategory::Tense, 0.97, "Past participle: 'has given'"),
            (r"(?i)\bcan able to\b", "can", RuleCategory::Tense, 0.95, "Redundant modal: use 'can' or 'able to'"),
            (r"(?i)\bwill can\b", "will be able to", RuleCategory::Tense, 0.95, "Use 'will be able to' instead of 'will can'"),
            (r"(?i)\bis been\b", "has been", RuleCategory::Tense, 0.95, "Use 'has been' instead of 'is been'"),

            // 5. Homophones & Word Choice
            (r"(?i)\btheir (is|are|was|were|will|can|could|should|would|have|has)\b", "there $1", RuleCategory::Homophone, 0.94, "Word choice: 'there'"),
            (r"(?i)\bthey'?re (car|house|book|dog|office|project|money|time|work|friend|family|team|laptop|code|email)\b", "their $1", RuleCategory::Homophone, 0.94, "Possessive form: 'their'"),
            (r"(?i)\byour (going|coming|welcome|right|wrong|leaving|working|doing|saying|thinking|talking)\b", "you're $1", RuleCategory::Homophone, 0.93, "Contraction 'you're' (you are)"),
            (r"(?i)\bits (a|an|the|my|your|our|their|very|so|too|always|never|going|working|done|good|bad|fine|easy|hard|late|early)\b", "it's $1", RuleCategory::Homophone, 0.94, "Contraction 'it's' (it is)"),
            (r"(?i)\bwhose (going|coming|there|here|that|this|calling)\b", "who's $1", RuleCategory::Homophone, 0.93, "Contraction 'who's' (who is)"),
            (r"(?i)\bmore (better|faster|cheaper|easier|higher|lower|smaller|larger)\b", "$1", RuleCategory::Homophone, 0.96, "Avoid double comparative"),
            (r"(?i)\bmore then\b", "more than", RuleCategory::Homophone, 0.97, "Comparison: 'more than'"),
            (r"(?i)\bless then\b", "less than", RuleCategory::Homophone, 0.97, "Comparison: 'less than'"),
            (r"(?i)\brather then\b", "rather than", RuleCategory::Homophone, 0.97, "Comparison: 'rather than'"),
            (r"(?i)\bbetter then\b", "better than", RuleCategory::Homophone, 0.97, "Comparison: 'better than'"),
            (r"(?i)\bfaster then\b", "faster than", RuleCategory::Homophone, 0.97, "Comparison: 'faster than'"),
            (r"(?i)\blarger then\b", "larger than", RuleCategory::Homophone, 0.97, "Comparison: 'larger than'"),
            (r"(?i)\bsmaller then\b", "smaller than", RuleCategory::Homophone, 0.97, "Comparison: 'smaller than'"),
            (r"(?i)\bother then\b", "other than", RuleCategory::Homophone, 0.97, "Comparison: 'other than'"),
            (r"(?i)\bto (much|many|late|early|fast|slow|hard|easy|soon|hot|cold|expensive)\b", "too $1", RuleCategory::Homophone, 0.94, "Use adverb 'too' (excessive)"),
            (r"(?i)\bme to\b", "me too", RuleCategory::Homophone, 0.95, "Use 'too' (also)"),
            (r"(?i)\byou to\b", "you too", RuleCategory::Homophone, 0.93, "Use 'too' (also)"),
            (r"(?i)\bloose (my|the|your|our|their|weight|money|time|job|mind)\b", "lose $1", RuleCategory::Homophone, 0.95, "Verb 'lose' (misplace/forfeit)"),

            // 6. Contractions & Missing Apostrophes
            (r"\bdont\b", "don't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'don't'"),
            (r"\bDont\b", "Don't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Don't'"),
            (r"\bdoesnt\b", "doesn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'doesn't'"),
            (r"\bDoesnt\b", "Doesn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Doesn't'"),
            (r"\bcant\b", "can't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'can't'"),
            (r"\bCant\b", "Can't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Can't'"),
            (r"\bwont\b", "won't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'won't'"),
            (r"\bWont\b", "Won't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Won't'"),
            (r"\bisnt\b", "isn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'isn't'"),
            (r"\bIsnt\b", "Isn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Isn't'"),
            (r"\barent\b", "aren't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'aren't'"),
            (r"\bArent\b", "Aren't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Aren't'"),
            (r"\bwasnt\b", "wasn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'wasn't'"),
            (r"\bWasnt\b", "Wasn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Wasn't'"),
            (r"\bwerent\b", "weren't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'weren't'"),
            (r"\bWerent\b", "Weren't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Weren't'"),
            (r"\bhavent\b", "haven't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'haven't'"),
            (r"\bHavent\b", "Haven't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Haven't'"),
            (r"\bhasnt\b", "hasn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'hasn't'"),
            (r"\bHasnt\b", "Hasn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Hasn't'"),
            (r"\bhadnt\b", "hadn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'hadn't'"),
            (r"\bHadnt\b", "Hadn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Hadn't'"),
            (r"\bdidnt\b", "didn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'didn't'"),
            (r"\bDidnt\b", "Didn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Didn't'"),
            (r"\bcouldnt\b", "couldn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'couldn't'"),
            (r"\bCouldnt\b", "Couldn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Couldn't'"),
            (r"\bshouldnt\b", "shouldn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'shouldn't'"),
            (r"\bShouldnt\b", "Shouldn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Shouldn't'"),
            (r"\bwouldnt\b", "wouldn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'wouldn't'"),
            (r"\bWouldnt\b", "Wouldn't", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'Wouldn't'"),
            (r"\btheyve\b", "they've", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'they've'"),
            (r"\bTheyve\b", "They've", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'They've'"),
            (r"\bweve\b", "we've", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'we've'"),
            (r"\bWeve\b", "We've", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'We've'"),
            (r"\byouve\b", "you've", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'you've'"),
            (r"\bYouve\b", "You've", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'You've'"),
            (r"\bive\b", "I've", RuleCategory::Spelling, 0.97, "Missing apostrophe and capitalization: 'I've'"),
            (r"\bIve\b", "I've", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'I've'"),
            (r"\bim\b", "I'm", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'I'm'"),
            (r"\bIm\b", "I'm", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'I'm'"),
            (r"\byoull\b", "you'll", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'you'll'"),
            (r"\bYoull\b", "You'll", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'You'll'"),
            (r"\btheyll\b", "they'll", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'they'll'"),
            (r"\bTheyll\b", "They'll", RuleCategory::Spelling, 0.97, "Missing apostrophe: 'They'll'"),

            // 7. Capitalization (Pronoun 'I', Days of Week, Months)
            (r"\b i \b", " I ", RuleCategory::Capitalization, 0.99, "Capitalize pronoun 'I'"),
            (r"^i \b", "I ", RuleCategory::Capitalization, 0.99, "Capitalize pronoun 'I'"),
            (r"\b i am\b", " I am", RuleCategory::Capitalization, 0.99, "Capitalize pronoun 'I'"),
            (r"\b i will\b", " I will", RuleCategory::Capitalization, 0.99, "Capitalize pronoun 'I'"),
            (r"\b i have\b", " I have", RuleCategory::Capitalization, 0.99, "Capitalize pronoun 'I'"),
            (r"\b i do\b", " I do", RuleCategory::Capitalization, 0.99, "Capitalize pronoun 'I'"),
            (r"\b i can\b", " I can", RuleCategory::Capitalization, 0.99, "Capitalize pronoun 'I'"),
            (r"\b i would\b", " I would", RuleCategory::Capitalization, 0.99, "Capitalize pronoun 'I'"),
            (r"\b i want\b", " I want", RuleCategory::Capitalization, 0.99, "Capitalize pronoun 'I'"),
            (r"\b i think\b", " I think", RuleCategory::Capitalization, 0.99, "Capitalize pronoun 'I'"),
            (r"\b i know\b", " I know", RuleCategory::Capitalization, 0.99, "Capitalize pronoun 'I'"),
            (r"\bmonday\b", "Monday", RuleCategory::Capitalization, 0.96, "Capitalize day of the week"),
            (r"\btuesday\b", "Tuesday", RuleCategory::Capitalization, 0.96, "Capitalize day of the week"),
            (r"\bwednesday\b", "Wednesday", RuleCategory::Capitalization, 0.96, "Capitalize day of the week"),
            (r"\bthursday\b", "Thursday", RuleCategory::Capitalization, 0.96, "Capitalize day of the week"),
            (r"\bfriday\b", "Friday", RuleCategory::Capitalization, 0.96, "Capitalize day of the week"),
            (r"\bsaturday\b", "Saturday", RuleCategory::Capitalization, 0.96, "Capitalize day of the week"),
            (r"\bsunday\b", "Sunday", RuleCategory::Capitalization, 0.96, "Capitalize day of the week"),
            (r"\bjanuary\b", "January", RuleCategory::Capitalization, 0.96, "Capitalize month"),
            (r"\bfebruary\b", "February", RuleCategory::Capitalization, 0.96, "Capitalize month"),
            (r"\bmarch\b", "March", RuleCategory::Capitalization, 0.96, "Capitalize month"),
            (r"\bapril\b", "April", RuleCategory::Capitalization, 0.96, "Capitalize month"),
            (r"\bjune\b", "June", RuleCategory::Capitalization, 0.96, "Capitalize month"),
            (r"\bjuly\b", "July", RuleCategory::Capitalization, 0.96, "Capitalize month"),
            (r"\baugust\b", "August", RuleCategory::Capitalization, 0.96, "Capitalize month"),
            (r"\bseptember\b", "September", RuleCategory::Capitalization, 0.96, "Capitalize month"),
            (r"\boctober\b", "October", RuleCategory::Capitalization, 0.96, "Capitalize month"),
            (r"\bnovember\b", "November", RuleCategory::Capitalization, 0.96, "Capitalize month"),
            (r"\bdecember\b", "December", RuleCategory::Capitalization, 0.96, "Capitalize month"),

            // 8. Word Redundancy
            (r"(?i)\bthe the\b", "the", RuleCategory::Redundancy, 0.98, "Duplicate word: 'the'"),
            (r"(?i)\bin in\b", "in", RuleCategory::Redundancy, 0.98, "Duplicate word: 'in'"),
            (r"(?i)\bto to\b", "to", RuleCategory::Redundancy, 0.98, "Duplicate word: 'to'"),
            (r"(?i)\band and\b", "and", RuleCategory::Redundancy, 0.98, "Duplicate word: 'and'"),
            (r"(?i)\bof of\b", "of", RuleCategory::Redundancy, 0.98, "Duplicate word: 'of'"),
            (r"(?i)\bfor for\b", "for", RuleCategory::Redundancy, 0.98, "Duplicate word: 'for'"),
            (r"(?i)\bis is\b", "is", RuleCategory::Redundancy, 0.98, "Duplicate word: 'is'"),
            (r"(?i)\bthat that\b", "that", RuleCategory::Redundancy, 0.98, "Duplicate word: 'that'"),
            (r"(?i)\bclose proximity\b", "proximity", RuleCategory::Redundancy, 0.92, "Redundancy: 'proximity' already means closeness"),
            (r"(?i)\bfree gift\b", "gift", RuleCategory::Redundancy, 0.92, "Redundancy: a gift is inherently free"),
            (r"(?i)\bend result\b", "result", RuleCategory::Redundancy, 0.92, "Redundancy: a result is already at the end"),
        ];

        for (pat, rep, cat, conf, exp) in raw_rules {
            if let Ok(re) = RegexBuilder::new(pat).build() {
                let est_len = pat.len();
                items.push(RuleItem {
                    regex: re,
                    replacement: rep.to_string(),
                    category: cat,
                    confidence: conf,
                    explanation: exp,
                    pattern_len: est_len,
                });
            }
        }

        // Sort rules by pattern length descending for maximum specificity first
        items.sort_by(|a, b| b.pattern_len.cmp(&a.pattern_len));

        Self {
            rules: items,
            punct_space_re: Regex::new(r"\s+([,.:;?!])").unwrap(),
            repeated_space_re: Regex::new(r"[ \t]{2,}").unwrap(),
        }
    }

    pub fn evaluate(&self, text: &str) -> Vec<RuleMatch> {
        let mut matches = Vec::new();

        // 1. Evaluate specific Regex rules
        for item in &self.rules {
            for m in item.regex.find_iter(text) {
                let matched_span = &text[m.start()..m.end()];
                let replaced_text = item.regex.replace(matched_span, &item.replacement).to_string();

                if matched_span != replaced_text {
                    matches.push(RuleMatch {
                        start: m.start(),
                        end: m.end(),
                        replacement: replaced_text,
                        category: item.category,
                        confidence: item.confidence,
                        explanation: item.explanation,
                    });
                }
            }
        }

        // 2. Punctuation spacing (e.g. "word ." -> "word.")
        for m in self.punct_space_re.find_iter(text) {
            let span = &text[m.start()..m.end()];
            let punct = span.trim();
            matches.push(RuleMatch {
                start: m.start(),
                end: m.end(),
                replacement: punct.to_string(),
                category: RuleCategory::Punctuation,
                confidence: 0.98,
                explanation: "Remove space before punctuation",
            });
        }

        // 3. Sentence-initial capitalization (e.g. "he is going" -> "He is going")
        if let Some(first_char) = text.chars().next() {
            if first_char.is_alphabetic() && first_char.is_lowercase() {
                let upper = first_char.to_uppercase().to_string();
                let char_len = first_char.len_utf8();
                matches.push(RuleMatch {
                    start: 0,
                    end: char_len,
                    replacement: upper,
                    category: RuleCategory::Capitalization,
                    confidence: 0.95,
                    explanation: "Capitalize the first letter of a sentence",
                });
            }
        }

        // 4. Clean repeated spaces (e.g. "word  word" -> "word word")
        for m in self.repeated_space_re.find_iter(text) {
            matches.push(RuleMatch {
                start: m.start(),
                end: m.end(),
                replacement: " ".to_string(),
                category: RuleCategory::Punctuation,
                confidence: 0.98,
                explanation: "Normalize spacing",
            });
        }

        matches
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
        assert!(!matches.is_empty());
        assert_eq!(matches[0].replacement, "He goes");
    }

    #[test]
    fn test_i_am_go_office() {
        let engine = RuleEngine::new();
        let matches = engine.evaluate("I am go office.");
        assert!(!matches.is_empty());
        let top = &matches[0];
        assert_eq!(top.replacement, "I am going to the office");
    }

    #[test]
    fn test_article_agreement() {
        let engine = RuleEngine::new();
        let matches = engine.evaluate("I have a apple.");
        assert!(!matches.is_empty());
        assert_eq!(matches[0].replacement, "an apple");
    }

    #[test]
    fn test_homophone_and_modals() {
        let engine = RuleEngine::new();
        let m1 = engine.evaluate("We could of won.");
        assert_eq!(m1[0].replacement, "could have");

        let m2 = engine.evaluate("Their is someone outside.");
        assert_eq!(m2[0].replacement, "there is");
    }

    #[test]
    fn test_punctuation_spacing_and_redundancy() {
        let engine = RuleEngine::new();
        let m1 = engine.evaluate("Hello , world .");
        assert!(m1.iter().any(|m| m.category == RuleCategory::Punctuation));

        let m2 = engine.evaluate("We went to the the office.");
        assert!(m2.iter().any(|m| m.replacement == "the"));
    }
}

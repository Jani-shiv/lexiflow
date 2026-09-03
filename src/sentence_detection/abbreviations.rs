use std::collections::HashSet;

pub struct AbbreviationDict {
    abbreviations: HashSet<&'static str>,
}

impl AbbreviationDict {
    pub fn new() -> Self {
        let mut set = HashSet::new();
        // Titles & Honorifics
        set.insert("mr.");
        set.insert("mrs.");
        set.insert("ms.");
        set.insert("dr.");
        set.insert("prof.");
        set.insert("rev.");
        set.insert("gen.");
        set.insert("sen.");
        set.insert("rep.");
        set.insert("gov.");
        set.insert("capt.");
        set.insert("lt.");
        set.insert("col.");
        set.insert("maj.");
        set.insert("sgt.");
        set.insert("st.");
        set.insert("jr.");
        set.insert("sr.");

        // Latin & Academic
        set.insert("e.g.");
        set.insert("i.e.");
        set.insert("etc.");
        set.insert("vs.");
        set.insert("v.");
        set.insert("al.");
        set.insert("cf.");
        set.insert("ibid.");
        set.insert("op.");
        set.insert("cit.");

        // Geography & Organizations
        set.insert("u.s.");
        set.insert("u.k.");
        set.insert("u.s.a.");
        set.insert("e.u.");
        set.insert("un.");
        set.insert("inc.");
        set.insert("ltd.");
        set.insert("co.");
        set.insert("corp.");
        set.insert("dept.");
        set.insert("univ.");

        // Time & Measurements
        set.insert("a.m.");
        set.insert("p.m.");
        set.insert("a.");
        set.insert("p.");
        set.insert("jan.");
        set.insert("feb.");
        set.insert("mar.");
        set.insert("apr.");
        set.insert("jun.");
        set.insert("jul.");
        set.insert("aug.");
        set.insert("sep.");
        set.insert("sept.");
        set.insert("oct.");
        set.insert("nov.");
        set.insert("dec.");
        set.insert("mon.");
        set.insert("tue.");
        set.insert("wed.");
        set.insert("thu.");
        set.insert("fri.");
        set.insert("sat.");
        set.insert("sun.");
        set.insert("no.");
        set.insert("vol.");
        set.insert("pp.");
        set.insert("fig.");

        Self { abbreviations: set }
    }

    pub fn is_abbreviation(&self, token: &str) -> bool {
        let lower = token.to_lowercase();
        self.abbreviations.contains(lower.as_str())
    }
}

impl Default for AbbreviationDict {
    fn default() -> Self {
        Self::new()
    }
}

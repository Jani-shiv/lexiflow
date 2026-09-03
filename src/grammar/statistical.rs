use std::collections::HashMap;

/// Lightweight statistical language scoring model for ranking fluency and candidate corrections.
/// Designed for ultra-low memory (< 10MB) and microsecond scoring latency.
pub struct StatisticalModel {
    bigram_counts: HashMap<(String, String), u32>,
    unigram_counts: HashMap<String, u32>,
}

impl StatisticalModel {
    pub fn new() -> Self {
        let mut unigrams = HashMap::new();
        let mut bigrams = HashMap::new();

        // Standard English transition frequencies for common collocations and grammar patterns
        let common_pairs: &[(&str, &str, u32)] = &[
            ("i", "am", 1500),
            ("i", "have", 1400),
            ("i", "was", 1200),
            ("i", "will", 1100),
            ("i", "do", 1000),
            ("i", "would", 900),
            ("i", "can", 950),
            ("i", "want", 900),
            ("i", "think", 850),
            ("i", "know", 900),
            ("he", "is", 1300),
            ("he", "goes", 1200),
            ("he", "has", 1200),
            ("he", "was", 1100),
            ("he", "works", 950),
            ("he", "wants", 900),
            ("she", "is", 1300),
            ("she", "goes", 1200),
            ("she", "has", 1200),
            ("she", "was", 1100),
            ("she", "works", 950),
            ("it", "is", 1800),
            ("it", "has", 1200),
            ("it", "works", 1100),
            ("they", "are", 1600),
            ("they", "were", 1300),
            ("they", "have", 1400),
            ("they", "go", 1200),
            ("we", "are", 1500),
            ("we", "were", 1200),
            ("we", "have", 1400),
            ("we", "go", 1200),
            ("you", "are", 1700),
            ("you", "were", 1300),
            ("you", "have", 1400),
            ("go", "to", 1600),
            ("goes", "to", 1500),
            ("going", "to", 1700),
            ("went", "to", 1500),
            ("to", "the", 2500),
            ("to", "school", 1200),
            ("in", "the", 2400),
            ("on", "the", 2200),
            ("at", "the", 2100),
            ("from", "the", 1800),
            ("by", "the", 1500),
            ("with", "the", 1600),
            ("the", "office", 800),
            ("the", "school", 750),
            ("the", "project", 700),
            ("the", "team", 650),
            ("an", "apple", 500),
            ("an", "hour", 600),
            ("an", "honest", 400),
            ("an", "idea", 550),
            ("a", "car", 700),
            ("a", "book", 650),
            ("a", "university", 450),
            ("could", "have", 1200),
            ("should", "have", 1100),
            ("would", "have", 1300),
            ("must", "have", 900),
            ("might", "have", 800),
            ("don't", "know", 1100),
            ("doesn't", "know", 900),
            ("thank", "you", 1800),
            ("good", "morning", 900),
            ("good", "at", 1100),
            ("as", "well", 1100),
            ("more", "than", 1400),
            ("better", "than", 1200),
            ("faster", "than", 900),
            ("look", "forward", 1100),
            ("forward", "to", 1200),
            ("depend", "on", 1000),
            ("depends", "on", 1000),
            ("interested", "in", 1100),
            ("listen", "to", 1200),
            ("listening", "to", 1100),
            ("married", "to", 900),
            ("responsible", "for", 1000),
        ];

        for &(w1, w2, count) in common_pairs {
            let key = (w1.to_string(), w2.to_string());
            *bigrams.entry(key).or_insert(0) += count;

            *unigrams.entry(w1.to_string()).or_insert(0) += count;
            *unigrams.entry(w2.to_string()).or_insert(0) += count;
        }

        Self {
            bigram_counts: bigrams,
            unigram_counts: unigrams,
        }
    }

    /// Evaluates probability of a string candidate using bigram probability scoring
    pub fn score_sentence(&self, text: &str) -> f64 {
        let words: Vec<String> = text
            .split_whitespace()
            .map(|w| w.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|w| !w.is_empty())
            .collect();

        if words.len() < 2 {
            return 0.5;
        }

        let mut log_prob = 0.0;
        let k = 0.1; // Laplace smoothing parameter
        let vocab_size = (self.unigram_counts.len() + 100) as f64;

        for i in 0..words.len() - 1 {
            let w1 = &words[i];
            let w2 = &words[i + 1];

            let bigram_count = *self.bigram_counts.get(&(w1.clone(), w2.clone())).unwrap_or(&0) as f64;
            let unigram_count = *self.unigram_counts.get(w1).unwrap_or(&0) as f64;

            // Smoothed conditional probability P(w2 | w1)
            let prob = (bigram_count + k) / (unigram_count + k * vocab_size);
            log_prob += prob.ln();
        }

        // Return geometric mean probability
        (log_prob / (words.len() - 1) as f64).exp()
    }

    /// Returns true if candidate text is statistically more fluent than original text
    pub fn is_more_fluent(&self, candidate: &str, original: &str) -> bool {
        let score_cand = self.score_sentence(candidate);
        let score_orig = self.score_sentence(original);
        score_cand >= score_orig
    }
}

impl Default for StatisticalModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistical_fluency_ranking() {
        let model = StatisticalModel::new();
        let s_fluent = model.score_sentence("He goes to the office.");
        let s_ungrammatical = model.score_sentence("He go the office.");

        assert!(s_fluent > s_ungrammatical);
        assert!(model.is_more_fluent("He goes to the office.", "He go the office."));
    }
}

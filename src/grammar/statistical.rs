use std::collections::HashMap;

/// Lightweight statistical language scoring model for ranking fluency and candidate corrections.
/// Designed for ultra-low memory (< 10MB) and microsecond scoring latency.
pub struct StatisticalModel {
    bigram_counts: HashMap<(String, String), u32>,
    unigram_counts: HashMap<String, u32>,
    total_unigrams: u64,
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
            ("he", "is", 1300),
            ("he", "goes", 1200),
            ("he", "has", 1200),
            ("he", "was", 1100),
            ("he", "works", 950),
            ("she", "is", 1300),
            ("she", "goes", 1200),
            ("she", "has", 1200),
            ("she", "was", 1100),
            ("it", "is", 1800),
            ("it", "has", 1200),
            ("it", "works", 1100),
            ("they", "are", 1600),
            ("they", "were", 1300),
            ("they", "have", 1400),
            ("we", "are", 1500),
            ("we", "were", 1200),
            ("we", "have", 1400),
            ("you", "are", 1700),
            ("you", "were", 1300),
            ("you", "have", 1400),
            ("go", "to", 1600),
            ("goes", "to", 1500),
            ("going", "to", 1700),
            ("went", "to", 1500),
            ("to", "the", 2500),
            ("in", "the", 2400),
            ("on", "the", 2200),
            ("at", "the", 2100),
            ("the", "office", 800),
            ("the", "school", 750),
            ("the", "project", 700),
            ("an", "apple", 500),
            ("an", "hour", 600),
            ("a", "car", 700),
            ("a", "book", 650),
            ("could", "have", 1200),
            ("should", "have", 1100),
            ("would", "have", 1300),
            ("don't", "know", 1100),
            ("doesn't", "know", 900),
            ("thank", "you", 1800),
            ("good", "morning", 900),
            ("as", "well", 1100),
        ];

        let mut total_uni = 0u64;

        for &(w1, w2, count) in common_pairs {
            *unigrams.entry(w1.to_string()).or_insert(0) += count;
            *unigrams.entry(w2.to_string()).or_insert(0) += count;
            bigrams.insert((w1.to_string(), w2.to_string()), count);
            total_uni += count as u64 * 2;
        }

        Self {
            bigram_counts: bigrams,
            unigram_counts: unigrams,
            total_unigrams: total_uni.max(1),
        }
    }

    /// Evaluates log-probability of a token sequence with Laplace smoothing
    pub fn score_sentence(&self, text: &str) -> f64 {
        let tokens: Vec<String> = text
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'').to_lowercase())
            .filter(|w| !w.is_empty())
            .collect();

        if tokens.is_empty() {
            return 0.0;
        }

        let vocab_size = (self.unigram_counts.len() + 1000) as f64;
        let mut log_prob = 0.0;

        for i in 0..tokens.len() {
            let w = &tokens[i];
            if i == 0 {
                let count = *self.unigram_counts.get(w).unwrap_or(&0) as f64;
                let p = (count + 1.0) / (self.total_unigrams as f64 + vocab_size);
                log_prob += p.ln();
            } else {
                let prev = &tokens[i - 1];
                let bi_count = *self.bigram_counts.get(&(prev.clone(), w.clone())).unwrap_or(&0) as f64;
                let prev_count = *self.unigram_counts.get(prev).unwrap_or(&0) as f64;
                let p = (bi_count + 1.0) / (prev_count + vocab_size);
                log_prob += p.ln();
            }
        }

        log_prob / (tokens.len() as f64)
    }

    /// Returns true if `candidate` is statistically more fluent than `original`
    pub fn is_more_fluent(&self, original: &str, candidate: &str) -> bool {
        self.score_sentence(candidate) > self.score_sentence(original)
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
        let bad = "He go to the office";
        let good = "He goes to the office";
        assert!(model.is_more_fluent(bad, good));
    }
}

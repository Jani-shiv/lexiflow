#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextDiff {
    pub start: usize,
    pub end: usize,
    pub original_slice: String,
    pub replacement_slice: String,
}

pub struct DiffGenerator;

impl DiffGenerator {
    /// Computes the minimal diff edit range between `original` and `corrected`
    pub fn compute_minimal_diff(original: &str, corrected: &str) -> Option<TextDiff> {
        if original == corrected {
            return None;
        }

        let orig_bytes = original.as_bytes();
        let corr_bytes = corrected.as_bytes();

        // Find common prefix
        let mut prefix_len = 0;
        while prefix_len < orig_bytes.len()
            && prefix_len < corr_bytes.len()
            && orig_bytes[prefix_len] == corr_bytes[prefix_len]
        {
            prefix_len += 1;
        }

        // Adjust prefix to valid UTF-8 char boundary
        while prefix_len > 0 && !original.is_char_boundary(prefix_len) {
            prefix_len -= 1;
        }

        // Find common suffix
        let mut orig_suffix = orig_bytes.len();
        let mut corr_suffix = corr_bytes.len();

        while orig_suffix > prefix_len
            && corr_suffix > prefix_len
            && orig_bytes[orig_suffix - 1] == corr_bytes[corr_suffix - 1]
        {
            orig_suffix -= 1;
            corr_suffix -= 1;
        }

        // Adjust suffix to valid UTF-8 char boundary
        while orig_suffix < orig_bytes.len() && !original.is_char_boundary(orig_suffix) {
            orig_suffix += 1;
        }
        while corr_suffix < corr_bytes.len() && !corrected.is_char_boundary(corr_suffix) {
            corr_suffix += 1;
        }

        let orig_slice = original[prefix_len..orig_suffix].to_string();
        let corr_slice = corrected[prefix_len..corr_suffix].to_string();

        Some(TextDiff {
            start: prefix_len,
            end: orig_suffix,
            original_slice: orig_slice,
            replacement_slice: corr_slice,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_diff_single_word() {
        let orig = "He go to school.";
        let corr = "He goes to school.";
        let diff = DiffGenerator::compute_minimal_diff(orig, corr).unwrap();
        assert_eq!(diff.start, 5); // after "He go"
        assert_eq!(diff.end, 5);
        assert_eq!(diff.original_slice, "");
        assert_eq!(diff.replacement_slice, "es");
    }

    #[test]
    fn test_minimal_diff_identical() {
        let text = "This is perfectly fine.";
        let diff = DiffGenerator::compute_minimal_diff(text, text);
        assert!(diff.is_none());
    }

    #[test]
    fn test_minimal_diff_multibyte_utf8() {
        let orig = "Café au lay";
        let corr = "Café au lait";
        let diff = DiffGenerator::compute_minimal_diff(orig, corr).unwrap();
        assert_eq!(diff.original_slice, "y");
        assert_eq!(diff.replacement_slice, "it");
    }
}

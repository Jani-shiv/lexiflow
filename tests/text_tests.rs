use lexiflow::sentence_detection::SentenceSegmenter;

#[test]
fn test_simple_and_long_sentences() {
    let segmenter = SentenceSegmenter::new();
    let text = "This is a simple sentence. Here is another one! Is this a third sentence?";
    let spans = segmenter.segment(text);
    assert_eq!(spans.len(), 3);
}

#[test]
fn test_complex_punctuation_and_quotes() {
    let segmenter = SentenceSegmenter::new();
    let text = "She said, \"This is incredible!\" Then he smiled.";
    let spans = segmenter.segment(text);
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].text, "She said, \"This is incredible!\"");
    assert_eq!(spans[1].text, "Then he smiled.");
}

#[test]
fn test_abbreviations_preservation() {
    let segmenter = SentenceSegmenter::new();
    let text = "Prof. Xavier met Dr. Watson at 8 a.m. in the U.K. on Mon. morning.";
    let spans = segmenter.segment(text);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_urls_emails_and_decimals() {
    let segmenter = SentenceSegmenter::new();
    let text = "Check https://github.com/rust-lang/rust. Send queries to admin@example.com or call 3.14159 units.";
    let spans = segmenter.segment(text);
    assert_eq!(spans.len(), 2);
}

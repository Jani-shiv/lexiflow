use lexiflow::grammar::GrammarEngine;

#[test]
fn test_zero_network_inference() {
    let engine = GrammarEngine::new();
    let text = "He go to school yesterday and see a apple.";
    let (corrected, matches) = engine.correct_sentence(text);
    assert!(!matches.is_empty());
    assert_eq!(corrected, "He goes to school yesterday and see an apple.");
}

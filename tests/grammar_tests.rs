use lexiflow::grammar::GrammarEngine;

#[test]
fn test_subject_verb_agreement_all() {
    let engine = GrammarEngine::new();

    let cases = [
        ("He go to school.", "He goes to school."),
        ("She have a book.", "She has a book."),
        ("They is coming.", "They are coming."),
        ("It work nicely.", "It works nicely."),
        ("He don't know.", "He doesn't know."),
        ("We was waiting.", "We were waiting."),
        ("I has the ticket.", "I have the ticket."),
    ];

    for (input, expected) in cases {
        let (corrected, _) = engine.correct_sentence(input);
        assert_eq!(corrected, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_article_agreement() {
    let engine = GrammarEngine::new();

    let cases = [
        ("I ate a apple.", "I ate an apple."),
        ("It is a urgent request.", "It is an urgent request."),
        ("He bought an car.", "He bought a car."),
        ("She attends an university.", "She attends a university."),
    ];

    for (input, expected) in cases {
        let (corrected, _) = engine.correct_sentence(input);
        assert_eq!(corrected, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_preposition_and_phrasing() {
    let engine = GrammarEngine::new();

    let cases = [
        ("I am go office.", "I am going to the office."),
        ("Please listen music.", "Please listen to music."),
        ("It depends of the weather.", "It depends on the weather."),
        ("She is interested on art.", "She is interested in art."),
        ("I look forward to hear from you.", "I look forward to hearing from you."),
    ];

    for (input, expected) in cases {
        let (corrected, _) = engine.correct_sentence(input);
        assert_eq!(corrected, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_spelling_and_typos() {
    let engine = GrammarEngine::new();

    let cases = [
        ("I recieved teh package.", "I received the package."),
        ("It is definately neccessary.", "It is definitely necessary."),
        ("We need to seperate them.", "We need to separate them."),
        ("See you tommorrow morning.", "See you tomorrow morning."),
    ];

    for (input, expected) in cases {
        let (corrected, _) = engine.correct_sentence(input);
        assert_eq!(corrected, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_capitalization_and_contractions() {
    let engine = GrammarEngine::new();

    let cases = [
        ("i will come on monday.", "I will come on Monday."),
        ("dont worry about it.", "Don't worry about it."),
        ("im ready to go.", "I'm ready to go."),
        ("theyre here already.", "They're here already."),
    ];

    for (input, expected) in cases {
        let (corrected, _) = engine.correct_sentence(input);
        assert_eq!(corrected, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_tense_and_homophones() {
    let engine = GrammarEngine::new();

    let cases = [
        ("We should of called.", "We should have called."),
        ("Their is no excuse.", "There is no excuse."),
        ("More better results.", "Better results."),
        ("More then enough.", "More than enough."),
    ];

    for (input, expected) in cases {
        let (corrected, _) = engine.correct_sentence(input);
        assert_eq!(corrected, expected, "Failed for input: {}", input);
    }
}

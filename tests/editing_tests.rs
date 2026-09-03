use lexiflow::text_context::TextContextBuffer;
use std::time::Duration;

#[test]
fn test_rapid_editing_backspace_and_insert() {
    let mut buf = TextContextBuffer::new(1000, Duration::from_secs(60));
    buf.set_app("notepad.exe");

    for c in "The quikc brown fox".chars() {
        buf.insert_char(c);
    }
    assert_eq!(buf.get_text(), "The quikc brown fox");

    // Backspace 10 times to remove " brown fox"
    for _ in 0..10 {
        buf.backspace();
    }
    assert_eq!(buf.get_text(), "The quikc");

    // Fix typo: backspace 2 chars ("kc")
    buf.backspace();
    buf.backspace();
    assert_eq!(buf.get_text(), "The qui");

    // Insert "ck brown fox jumps"
    buf.insert_str("ck brown fox jumps");
    assert_eq!(buf.get_text(), "The quick brown fox jumps");
}

#[test]
fn test_paste_and_replacement() {
    let mut buf = TextContextBuffer::new(1000, Duration::from_secs(60));
    buf.set_app("code.exe");

    buf.set_text("Pasted entire sentence in editor.", 33);
    assert_eq!(buf.get_text(), "Pasted entire sentence in editor.");
    assert_eq!(buf.cursor_pos(), 33);
}

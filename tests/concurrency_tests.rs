use lexiflow::replacement::{InjectionGuard, ReplacementResult, TextInjector};
use lexiflow::scheduler::DebounceScheduler;
use lexiflow::suggestion::{ActiveSuggestion, SuggestionStatus};
use std::sync::Arc;
use std::time::Instant;

#[test]
fn test_race_condition_protection_while_typing() {
    let scheduler = DebounceScheduler::new(50);
    let guard = InjectionGuard::new();
    let injector = TextInjector::new(guard);

    // 1. User types first part: "I am go office"
    let req1_id = scheduler.submit_input("I am go office", 14, "notepad.exe");

    // 2. AI starts processing req1
    let suggestion_req1 = ActiveSuggestion {
        suggestion_id: 1,
        request_id: req1_id,
        original_text: "I am go office".to_string(),
        replacement_text: "I am going to the office".to_string(),
        start_offset: 0,
        end_offset: 14,
        confidence: 0.98,
        category: lexiflow::grammar::RuleCategory::Preposition,
        explanation: "Preposition fix".to_string(),
        app_name: "notepad.exe".to_string(),
        created_at: Instant::now(),
        status: SuggestionStatus::Pending,
    };

    // 3. User immediately continues typing before AI finishes: "...because I need to finish"
    let current_user_text = "I am go office because I need to finish the project before tomorrow.";
    let req2_id = scheduler.submit_input(current_user_text, current_user_text.len(), "notepad.exe");

    // 4. Verify request 1 is now stale
    assert!(!scheduler.is_request_current(req1_id));
    assert!(scheduler.is_request_current(req2_id));

    // 5. Attempt to apply the old result from req1 with current version status
    let is_current = scheduler.is_request_current(suggestion_req1.request_id);
    let result = injector.apply_replacement(
        &suggestion_req1,
        "notepad.exe",
        current_user_text,
        is_current,
    );

    // 6. Old result MUST NOT be applied because request is stale
    assert_eq!(result, ReplacementResult::StaleContext);
}

#[test]
fn test_concurrent_rapid_keystrokes() {
    let scheduler = Arc::new(DebounceScheduler::new(20));
    let mut handles = Vec::new();

    for i in 0..10 {
        let sc = Arc::clone(&scheduler);
        let h = std::thread::spawn(move || {
            for j in 0..50 {
                let text = format!("Typing sentence from thread {} step {}", i, j);
                sc.submit_input(&text, text.len(), "test.exe");
            }
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    assert!(scheduler.latest_request_id() >= 500);
}

use fancy_regex::Regex;

use crate::{DisplayLevel, Draft, Message::*, Note, Rule, TestDraft};

#[test]
fn max_word_len_works() {
    let outcome = Draft {
        rules: vec![Rule {
            pattern: Regex::new("a").unwrap(),
            intent: false,
            note: Some(Note("Should not contain ⟨a⟩".to_string())),
        }],
        messages: vec![
            Info(Note("This is a really really long note".to_string())),
            // Passing test
            Test(TestDraft {
                word: "hello".to_string(),
                intent: true,
            }),
            // Failing test
            Test(TestDraft {
                word: "abc".to_string(),
                intent: true,
            }),
        ],
        mode: Default::default(),
        test_count: 2,
    }
    .run();

    assert_eq!(outcome.max_word_len(DisplayLevel::ShowAll), 5); // hello
    assert_eq!(outcome.max_word_len(DisplayLevel::IgnorePasses), 3); // abc
    assert_eq!(outcome.max_word_len(DisplayLevel::OnlyFails), 3); // abc
    assert_eq!(outcome.max_word_len(DisplayLevel::HideAll), 0); // [none]
}

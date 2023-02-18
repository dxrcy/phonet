use fancy_regex::Regex;

use phonet::{self, Draft, Message, Note, Rule, TestDraft};

#[test]
fn tokipona_should_pass() {
    let file = include_str!("../examples/tokipona.phonet");

    assert_eq!(
        Draft::from(file).expect("Failed to parse").run().fail_count,
        0
    );
}

#[test]
fn example_draft_works() {
    let file = include_str!("../examples/example.phonet");

    let draft = Draft::from(file).expect("Failed to parse");

    assert_eq!(draft.mode, phonet::Mode::Romanized);
    assert_eq!(draft.test_count, 15);

    let mut rules = draft.rules.iter();

    assert_eq!(
        rules.next().unwrap(),
        &Rule {
            pattern: Regex::new(r"^[ptkmnswjlaeiou]+$").unwrap(),
            intent: true,
            note: Some(Note("Invalid letters".to_string())),
        },
    );

    assert_eq!(
        rules.next().unwrap(),
        &Rule {
            pattern: Regex::new(r"^([ptkmnswjl][aeiou])+$").unwrap(),
            intent: true,
            note: Some(Note("Syllable structure".to_string())),
        }
    );

    assert_eq!(
        rules.next().unwrap(),
        &Rule {
            pattern: Regex::new(r"(.)\1").unwrap(),
            intent: false,
            note: Some(Note("No repeated letters".to_string())),
        }
    );

    assert_eq!(
        rules.next().unwrap(),
        &Rule {
            pattern: Regex::new(r"(?<x>.)\k<x>").unwrap(),
            intent: false,
            note: Some(Note("No repeated letters".to_string())),
        }
    );

    assert_eq!(rules.next(), None);

    let mut messages = draft.messages.iter();

    assert_eq!(
        messages.next().unwrap(),
        &Message::Info(Note("Invalid letters".to_string()))
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "taso".to_string(),
            intent: true
        })
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "tyxo".to_string(),
            intent: false
        })
    );

    assert_eq!(
        messages.next().unwrap(),
        &Message::Info(Note("Examples of failing tests".to_string()))
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "tyxo".to_string(),
            intent: true
        })
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "taso".to_string(),
            intent: false
        })
    );

    assert_eq!(
        messages.next().unwrap(),
        &Message::Info(Note("Syllable structure".to_string()))
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "taso".to_string(),
            intent: true
        })
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "kili".to_string(),
            intent: true
        })
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "ano".to_string(),
            intent: false
        })
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "taaso".to_string(),
            intent: false
        })
    );

    assert_eq!(
        messages.next().unwrap(),
        &Message::Info(Note("Some more tests".to_string()))
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "silo".to_string(),
            intent: true
        })
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "tila".to_string(),
            intent: true
        })
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "aka".to_string(),
            intent: false
        })
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "axe".to_string(),
            intent: false
        })
    );

    assert_eq!(
        messages.next().unwrap(),
        &Message::Info(Note("No repeated letters".to_string()))
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "taso".to_string(),
            intent: true
        })
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "taaso".to_string(),
            intent: false
        })
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "ttaso".to_string(),
            intent: false
        })
    );

    assert_eq!(
        messages.next().unwrap(),
        &Message::Info(Note("2 tests *should* have failed!".to_string()))
    );

    assert_eq!(messages.next(), None);
}

#[test]
fn example_outcome_works() {
    let file = include_str!("../examples/example.phonet");

    let outcome = Draft::from(file).expect("Failed to parse").run();

    println!("{:?}", outcome);
    panic!()
}

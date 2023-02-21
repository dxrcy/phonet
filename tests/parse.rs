use fancy_regex::Regex;

use phonet::{
    self,
    draft::{Message, Mode, Note, Rule, TestDraft},
    Draft,
};

#[test]
fn example_draft_works() {
    let file = include_str!("../examples/example.phonet");

    let draft = Draft::from(file).expect("Failed to parse");

    assert_eq!(draft.mode, Mode::Romanized);
    assert_eq!(draft.test_count, 17);

    for rule in &draft.rules {
        assert!(
            !rule.pattern.to_string().contains(' '),
            "Rule should not contain space"
        );
    }

    let mut rules = draft.rules.iter();

    assert_eq!(
        rules.next().unwrap(),
        &Rule {
            pattern: Regex::new(r"^(?:(?:[ptkmnswjl])|(?:[aeiou]))+$").unwrap(),
            intent: true,
            note: Some(Note("Invalid letters".to_string())),
        },
    );

    assert_eq!(
        rules.next().unwrap(),
        &Rule {
            pattern: Regex::new(r"^(?:[aeiou])?((?:[ptkmnswjl])(?:[aeiou]))+$").unwrap(),
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
            intent: true
        })
    );
    assert_eq!(
        messages.next().unwrap(),
        &Message::Test(TestDraft {
            word: "atoso".to_string(),
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
            word: "an".to_string(),
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
            word: "akka".to_string(),
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

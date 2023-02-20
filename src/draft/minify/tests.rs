use std::collections::HashMap;

use super::{Message::*, *};

macro_rules! classes {
    () => {{
        let mut hm = HashMap::new();

        hm.insert("C".to_string(), "[ptk]".to_string());
        hm.insert("V".to_string(), "[aeiou]".to_string());
        hm.insert("_".to_string(), "[⟨C⟩⟨V⟩]".to_string());

        hm
    }};
}

#[test]
fn minify_works() {
    let mode = Mode::Broad;

    let classes = classes!();

    let rules_raw = vec![
        RawRule {
            pattern: "^⟨_⟩+$".to_string(),
            intent: true,
            note: Some(Note("Should contain ⟨a⟩".to_string())),
        },
        RawRule {
            pattern: "⟨V⟩⟨V⟩".to_string(),
            intent: false,
            note: None,
        },
    ];

    // * All kinds of tests

    let messages = vec![
        //
        Test(TestDraft {
            word: "ax".to_string(),
            intent: false,
        }),
        //
        Info(Note("this is a note".to_string())),
        //
        Test(TestDraft {
            word: "abc".to_string(),
            intent: true,
        }),
        //
        Info(Note("another note".to_string())),
        //
        Test(TestDraft {
            word: "hello".to_string(),
            intent: false,
        }),
    ];

    let minified = minify(mode, &classes, &rules_raw, &messages, true).unwrap();

    assert_eq!(
        minified,
        "~//;+^[[ptk][aeiou]]+$;![aeiou][aeiou];?+abc;?!ax hello"
    );

    // * No negative tests

    let messages = vec![
        //
        Info(Note("this is a note".to_string())),
        //
        Test(TestDraft {
            word: "abc".to_string(),
            intent: true,
        }),
        //
        Info(Note("another note".to_string())),
    ];

    let minified = minify(mode, &classes, &rules_raw, &messages, true).unwrap();

    assert_eq!(minified, "~//;+^[[ptk][aeiou]]+$;![aeiou][aeiou];?+abc");

    // * No positive tests

    let messages = vec![
        //
        Test(TestDraft {
            word: "ax".to_string(),
            intent: false,
        }),
        //
        Test(TestDraft {
            word: "hello".to_string(),
            intent: false,
        }),
    ];

    let minified = minify(mode, &classes, &rules_raw, &messages, true).unwrap();

    assert_eq!(
        minified,
        "~//;+^[[ptk][aeiou]]+$;![aeiou][aeiou];?!ax hello"
    );

    // * No tests, no rules

    let messages = vec![
        Info(Note("this is a note".to_string())),
        Info(Note("another note".to_string())),
    ];

    let minified = minify(mode, &classes, &[], &messages, true).unwrap();

    assert_eq!(minified, "~//;");
}

//TODO Failed to parse

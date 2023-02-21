use super::super::Note;
use super::*;

#[test]
fn minify_works() {
    let mode = Mode::Broad;

    let classes = example_classes!();

    let raw_rules = vec![
        RawRule {
            pattern: "^⟨_⟩+$".to_string(),
            intent: true,
            note: Some(Note("Should contain ⟨a⟩".to_string())),
            line: 0,
        },
        RawRule {
            pattern: "⟨V⟩⟨V⟩".to_string(),
            intent: false,
            note: None,
            line: 0,
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

    let minified = minify(mode, &classes, &raw_rules, &messages, true).unwrap();

    assert_eq!(
        minified,
        "~//;+^[[ptk][aeiou]]+$;![aeiou][aeiou];?+abc;?!ax hello"
    );

    // * ...with tests disabled

    let minified = minify(mode, &classes, &raw_rules, &messages, false).unwrap();

    assert_eq!(minified, "~//;+^[[ptk][aeiou]]+$;![aeiou][aeiou]");

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

    let minified = minify(mode, &classes, &raw_rules, &messages, true).unwrap();

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

    let minified = minify(mode, &classes, &raw_rules, &messages, true).unwrap();

    assert_eq!(
        minified,
        "~//;+^[[ptk][aeiou]]+$;![aeiou][aeiou];?!ax hello"
    );

    // * ...with tests disabled

    let minified = minify(mode, &classes, &raw_rules, &messages, false).unwrap();

    assert_eq!(minified, "~//;+^[[ptk][aeiou]]+$;![aeiou][aeiou]");

    // * No tests, no rules

    let messages = vec![
        Info(Note("this is a note".to_string())),
        Info(Note("another note".to_string())),
    ];

    let minified = minify(mode, &classes, &[], &messages, true).unwrap();

    assert_eq!(minified, "~//;");

    // * ...with tests disabled

    let minified = minify(mode, &classes, &raw_rules, &messages, false).unwrap();

    assert_eq!(minified, "~//;+^[[ptk][aeiou]]+$;![aeiou][aeiou]");
}

//TODO Failed to parse

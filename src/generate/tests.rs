use std::collections::HashMap;

use fancy_regex::Regex;

use crate::draft::Note;

use super::*;

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
fn get_letters_works() {
    let classes = classes!();

    assert_eq!(get_any_class(&classes).unwrap(), "[[ptk][aeiou]]");

    assert_eq!(remove_regex_symbols("[[ptk][aeiou]]"), "ptkaeiou");

    assert_eq!(get_letters(&classes).unwrap(), "ptkaeiou");
}

#[test]
fn generate_works() {
    let classes = classes!();

    let rules = vec![
        Rule {
            pattern: Regex::new("^[[ptk][aeiou]]+$").unwrap(),
            intent: true,
            note: Some(Note("Should contain ⟨a⟩".to_string())),
        },
        Rule {
            pattern: Regex::new("[aeiou][aeiou]").unwrap(),
            intent: false,
            note: None,
        },
    ];

    let count = 50;
    let length = 4..6;

    let draft = Draft {
        messages: vec![],
        rules: rules.clone(),
        mode: Default::default(),
        test_count: 0,
        raw_rules: vec![],
        raw_classes: classes,
    };

    // Generate some random valid words
    let words = draft.generate(count, length.clone()).unwrap();

    assert_eq!(words.len(), count);

    for word in words {
        // Check word is valid
        assert!(matches!(validate_test(&word, &rules), Valid));
        // Check length is in range
        assert!(length.contains(&word.len()));
    }
}

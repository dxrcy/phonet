use std::collections::HashMap;

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
fn parse_rules_works() {
    let classes = classes!();

    let raw_rules = vec![
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

    // Non-capture syntax `(?:___)` is removed, somewhere between regex parse and stringify
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

    assert_eq!(parse_rules(&raw_rules, &classes).unwrap(), rules);
}

#[test]
fn parse_regex_works() {
    let classes = classes!();

    assert_eq!(parse_regex("⟨V⟩", &classes).unwrap().to_string(), "[aeiou]");
    assert_eq!(parse_regex("⟨C⟩", &classes).unwrap().to_string(), "[ptk]");
    assert_eq!(
        parse_regex("⟨_⟩", &classes).unwrap().to_string(),
        "[[ptk][aeiou]]"
    );

    assert_eq!(
        parse_regex("⟨V⟩|<C>⟨C⟩", &classes).unwrap().to_string(),
        "[aeiou]|[ptk][ptk]"
    );

    assert_eq!(
        parse_regex("^⟨_⟩+$", &classes).unwrap().to_string(),
        "^[[ptk][aeiou]]+$"
    );

    assert_eq!(
        parse_regex(r"(?<a>a)\k<a>|(?P<b><C>)", &classes)
            .unwrap()
            .to_string(),
        r"(?<a>a)\k<a>|(?P<b>[ptk])"
    );
}

//TODO Use error kinds
#[test]
fn parse_regex_returns_error() {
    let classes = classes!();

    // Unknown class
    assert!(matches!(
        parse_regex("⟨E⟩", &classes),
        Err(Error::Generic(..))
    ));

    // Invalid regex
    assert!(matches!(
        parse_regex(r"*\", &classes),
        Err(Error::Generic(..))
    ));
}

use crate::error::ParseError;

use super::*;

#[test]
fn parse_rules_works() {
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
    let classes = example_classes!();

    assert_eq!(
        parse_regex("⟨V⟩", &classes, 0).unwrap().to_string(),
        "[aeiou]"
    );
    assert_eq!(
        parse_regex("⟨C⟩", &classes, 0).unwrap().to_string(),
        "[ptk]"
    );
    assert_eq!(
        parse_regex("⟨_⟩", &classes, 0).unwrap().to_string(),
        "[[ptk][aeiou]]"
    );

    assert_eq!(
        parse_regex("⟨V⟩|<C>⟨C⟩", &classes, 0).unwrap().to_string(),
        "[aeiou]|[ptk][ptk]"
    );

    assert_eq!(
        parse_regex("^⟨_⟩+$", &classes, 0).unwrap().to_string(),
        "^[[ptk][aeiou]]+$"
    );

    assert_eq!(
        parse_regex(r"(?<a>a)\k<a>|(?P<b><C>)", &classes, 0)
            .unwrap()
            .to_string(),
        r"(?<a>a)\k<a>|(?P<b>[ptk])"
    );
}

//TODO Use error kinds
#[test]
fn parse_regex_returns_error() {
    let classes = example_classes!();

    // Unknown class
    assert!(matches!(
        parse_regex("⟨E⟩", &classes, 0),
        Err(Error::Parse(ParseError::ClassNotFound(..), _))
    ));

    // Invalid regex
    assert!(matches!(
        parse_regex(r"*\", &classes, 0),
        Err(Error::Parse(ParseError::RegexParseFail(..), _))
    ));
}

use crate::error::ParseError;

use super::*;

#[test]
fn swap_angle_brackets_works() {
    assert_eq!(replace_angle_brackets("<abc>"), "⟨abc⟩");
    assert_eq!(replace_angle_brackets("(?<=abc)"), "(?<=abc)");
    assert_eq!(replace_angle_brackets("(?<!abc)"), "(?<!abc)");
    assert_eq!(replace_angle_brackets("(?<abc>)"), "(?<abc>)");
    assert_eq!(replace_angle_brackets("(?P<abc>)"), "(?P<abc>)");
    assert_eq!(replace_angle_brackets(r"\k<abc>"), r"\k<abc>");
    assert_eq!(replace_angle_brackets(r"(?<a>.)\k<a>"), r"(?<a>.)\k<a>");
    assert_eq!(replace_angle_brackets("(?:<abc>)"), "(?:⟨abc⟩)");
    assert_eq!(replace_angle_brackets("?<abc>"), "?⟨abc⟩");
    assert_eq!(replace_angle_brackets("<abc><def>"), "⟨abc⟩⟨def⟩");
    assert_eq!(replace_angle_brackets("<abc><"), "⟨abc⟩<");
    assert_eq!(replace_angle_brackets("<abc>>"), "⟨abc⟩>");
}

#[test]
fn replace_classes_works() {
    let classes = example_classes!();

    assert_eq!(
        replace_classes("<C>", &classes, 0).unwrap(),
        "[ptk]".to_string()
    );

    assert_eq!(
        replace_classes("<C>-<V>", &classes, 0).unwrap(),
        "[ptk]-[aeiou]".to_string()
    );

    assert_eq!(
        replace_classes("<_>", &classes, 0).unwrap(),
        "[[ptk][aeiou]]".to_string()
    );

    assert_eq!(
        replace_classes("(?<=1)", &classes, 0).unwrap(),
        "(?<=1)".to_string()
    );

    assert_eq!(
        replace_classes("(?<abc><C>)", &classes, 0).unwrap(),
        "(?<abc>[ptk])".to_string()
    );

    assert_eq!(replace_classes("a>b", &classes, 0).unwrap(), "a>b");
    assert_eq!(replace_classes("a<b", &classes, 0).unwrap(), "a<b");
}

#[test]
fn replace_classes_returns_error() {
    let classes = example_classes!();

    assert!(matches!(
        replace_classes("<c>", &classes, 0),
        Err(Error::Parse(ParseError::ClassNotFound(..), _))
    ));

    assert!(matches!(
        replace_classes("<a<b>c>", &classes, 0),
        Err(Error::Parse(ParseError::InvalidClassName(..), _))
    ));
}

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
    let classes = HashMap::from([
        ("C".to_string(), "[ptk]".to_string()),
        ("Vowels".to_string(), "[aio]".to_string()),
        ("_".to_string(), "[<C><Vowels>]".to_string()),
    ]);

    assert_eq!(
        replace_classes("<C>", &classes).unwrap(),
        "[ptk]".to_string()
    );

    assert_eq!(
        replace_classes("<C>-<Vowels>", &classes).unwrap(),
        "[ptk]-[aio]".to_string()
    );

    assert_eq!(
        replace_classes("<_>", &classes).unwrap(),
        "[[ptk][aio]]".to_string()
    );

    assert_eq!(
        replace_classes("(?<=1)", &classes).unwrap(),
        "(?<=1)".to_string()
    );

    assert_eq!(
        replace_classes("(?<abc><C>)", &classes).unwrap(),
        "(?<abc>[ptk])".to_string()
    );

    assert_eq!(replace_classes("a>b", &classes).unwrap(), "a>b");
    assert_eq!(replace_classes("a<b", &classes).unwrap(), "a<b");
}

//TODO Use error kinds
#[test]
fn replace_classes_returns_error() {
    let classes = classes!();

    assert!(matches!(
        replace_classes("<c>", &classes),
        Err(Error::Generic(..))
    ));

    assert!(matches!(
        replace_classes("<a<b>c>", &classes),
        Err(Error::Generic(..))
    ));
}

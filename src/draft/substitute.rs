use std::collections::HashMap;

use fancy_regex_macro::regex;

use crate::Error;

/// Replace ascii `<` and `>` with `⟨` and `⟩` respectively, for classes
///
/// Does not replace `<` and `>` with use in look-behinds or named group definitions or references
///
/// Uses `fancy_regex` `replace_all` method, with with capture preservation
fn replace_angle_brackets(pattern: &str) -> String {
    regex!(r"(?<!\(\?)(?<!\(\?P)(?<!\\k)<([^>]*)>")
        .replace_all(pattern, r"⟨$1⟩")
        .to_string()
}

/// Substitute class names regex rule with class values (recursively)
///
/// `pattern` argument must not contain spaces
pub fn substitute_classes(
    pattern: &str,
    classes: &HashMap<String, String>,
    // line: usize,
) -> Result<String, Error> {
    // Replace `<` and `>` with `⟨` and `⟩` respectively, where classes are
    let pattern = replace_angle_brackets(pattern);

    // Return string
    let mut output = String::new();

    // Build class name
    let mut name_build: Option<String> = None;

    // Loop characters
    for ch in pattern.chars() {
        match ch {
            // Open class name
            '⟨' => {
                if name_build.is_some() {
                    // Name is already building - Another opening bracket should not be there
                    return Err(Error::Generic(0, format!("Unexpected class name open")));
                }

                // Start building name
                name_build = Some(String::new());
            }

            // Close class name
            '⟩' => {
                // Get class name
                let Some(name) = name_build else {
                    // No name is building - Closing bracket should not be there
                    return Err(Error::Generic(0, format!("Unexpected class name close")));
                };

                // Get class value
                let Some(value) = classes.get(&name) else {
                    // Class name was not found
                    return Err(Error::Generic(0, format!("Class not found")));
                };

                // Add value to output (recursively)
                output.push_str(&substitute_classes(value, classes)?);

                // Finish building and reset
                name_build = None;
            }

            // Other character
            _ => {
                match &mut name_build {
                    // Name is building - push to name
                    Some(name) => name.push(ch),
                    // Name is not building - push to regular output
                    None => output.push(ch),
                }
            }
        }
    }

    // Class name was not finished building, before end of end of pattern
    if name_build.is_some() {
        return Err(Error::Generic(
            0,
            String::from("Unexpected end of pattern for class name"),
        ));
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
    fn substitute_classes_works() {
        let classes = HashMap::from([
            ("C".to_string(), "[ptk]".to_string()),
            ("Vowels".to_string(), "[aio]".to_string()),
            ("_".to_string(), "[<C><Vowels>]".to_string()),
        ]);

        assert_eq!(
            substitute_classes("<C>", &classes).unwrap(),
            "[ptk]".to_string()
        );

        assert_eq!(
            substitute_classes("<C>-<Vowels>", &classes).unwrap(),
            "[ptk]-[aio]".to_string()
        );

        assert_eq!(
            substitute_classes("<_>", &classes).unwrap(),
            "[[ptk][aio]]".to_string()
        );

        assert_eq!(
            substitute_classes("(?<=1)", &classes).unwrap(),
            "(?<=1)".to_string()
        );

        assert_eq!(
            substitute_classes("(?<abc><C>)", &classes).unwrap(),
            "(?<abc>[ptk])".to_string()
        );

        assert_eq!(substitute_classes("a>b", &classes).unwrap(), "a>b");
        assert_eq!(substitute_classes("a<b", &classes).unwrap(), "a<b");

        //TODO After implemented error kinds
        // assert!(match substitute_classes("<c>", &classes) {
        //     Err(Error::ClassNotFound { .. }) => true,
        //     _ => false,
        // });

        // assert!(match substitute_classes("<a<b>c>", &classes) {
        //     Err(Error::ClassNotFound { .. }) => true,
        //     _ => false,
        // });
    }
}

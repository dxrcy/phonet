use std::collections::HashMap;

use fancy_regex::Regex;
use fancy_regex_macro::regex;

use super::{
    statements::split_statements, substitute::substitute_classes, Draft, Message, Mode, Note, Rule,
    TestDraft,
};
use crate::{Error, REGEX_MATCH_FAIL};

/// Alias for `HashMap` of `String` and `String`, for raw classes
type Classes = HashMap<String, String>;

/// Mirrors `Rule` struct, but with `String` instead of `Regex`
struct RawRule {
    intent: bool,
    pattern: String,
    note: Option<Note>,
}

/// Parse *Phonet* `Draft` from file
/// TODO Tests!
pub fn parse_draft(file: &str) -> Result<Draft, Error> {
    // Split file into statements
    let statements = split_statements(file);

    // Field builders
    let mut messages = Vec::new();
    let mut mode: Option<Mode> = None;

    // Field builders without regex parsed
    let mut rules_raw = Vec::new();
    let mut classes_raw = HashMap::new();

    // Most recent note
    let mut last_note: Option<Note> = None;

    // Loop statements
    for (statement, line) in statements {
        let statement = statement.trim();

        // Skip if blank
        if statement.is_empty() {
            continue;
        }

        // Get line operator first character
        let mut chars = statement.chars();
        let Some(operator) = chars.next() else {
            continue;
        };

        // Match line operator
        match operator {
            // Comment
            '#' => continue,

            // Mode
            '~' => {
                // Fail if mode is already defined
                if mode.is_some() {
                    return Err(Error::Generic(line, format!("Mode already defined")));
                }

                // Remove spaces
                while chars.as_str().starts_with(' ') {
                    chars.next();
                }

                // Select mode
                mode = Some(match (chars.next(), chars.last()) {
                    (Some('<'), Some('>')) => Mode::Romanized,
                    (Some('/'), Some('/')) => Mode::Broad,
                    (Some('['), Some(']')) => Mode::Narrow,

                    // Invalid mode specifier
                    _ => return Err(Error::Generic(line, format!("Invalid mode specifier"))),
                })
            }

            // Class
            '$' => {
                let mut split = chars.as_str().split('=');

                // Get class name
                let Some(name) = split.next() else {
                    return Err(Error::Generic(line, format!("No class name given")));
                };
                let name = name.trim();

                // Check if name is valid
                if !regex!(r"^\w+$").is_match(&name).expect(REGEX_MATCH_FAIL) {
                    return Err(Error::Generic(
                        line,
                        format!("Invalid class name '{}'", name),
                    ));
                }

                // Check that class name does not exist
                if classes_raw.get(name).is_some() {
                    return Err(Error::Generic(
                        line,
                        format!("Class already exists named '{}'", name),
                    ));
                }

                // Get class pattern
                let Some(pattern) = split.next() else {
                    return Err(Error::Generic(line, format!("No class name given")));
                };

                // Add class
                // Wrap value in NON-CAPTURING GROUP (just in case)
                // This is non-capturing, for classes to work with back-references
                // otherwise classes would be inherently capturing, and count towards group index in back-reference
                classes_raw.insert(name.trim().to_string(), pattern.trim().to_string());
            }

            // Rule
            '+' | '!' => {
                // `+` for true, `!` for false
                let intent = operator == '+';

                let pattern = chars.as_str().replace(' ', "");

                // Get most recent note, owned
                let note = last_note.clone();

                // Add rule
                rules_raw.push(RawRule {
                    intent,
                    pattern,
                    note,
                })
            }

            // Test
            '?' => {
                // Remove spaces
                while chars.as_str().starts_with(' ') {
                    chars.next();
                }

                // Get intent
                let intent = match chars.next() {
                    // Should be INVALID to pass
                    Some('+') => true,
                    // Should be VALID to pass
                    Some('!') => false,

                    // Unknown character
                    Some(ch) => {
                        return Err(Error::Generic(
                            line,
                            format!("Invalid intent identifier {}", ch),
                        ));
                    }
                    // No character
                    None => {
                        return Err(Error::Generic(line, format!("Missing intent identifier")));
                    }
                };

                // Split at space
                for word in chars.as_str().split_whitespace() {
                    let word = word.trim().to_string();

                    // Add test
                    if !word.is_empty() {
                        messages.push(Message::Test(TestDraft { intent, word }));
                    }
                }
            }

            // Note
            '*' => {
                let note = chars.as_str().trim();

                if note.is_empty() {
                    return Err(Error::Generic(line, format!("Note cannot be empty")));
                }

                // Add message
                messages.push(Message::Info(Note(note.to_string())));

                // Add note
                last_note = Some(Note(note.to_string()));
            }

            // Unknown line operator
            _ => {
                return Err(Error::Generic(
                    line,
                    format!("Unknown line operator {}", operator),
                ))
            }
        }
    }

    let draft = Draft {
        rules: parse_rules(rules_raw, &classes_raw)?,
        messages,
        mode: mode.unwrap_or_default(),
    };

    println!("{:#?}", draft);

    Ok(draft)
}

/// Parse each rule in list
/// TODO Tests!
fn parse_rules(rules: Vec<RawRule>, classes: &Classes) -> Result<Vec<Rule>, Error> {
    let mut new = Vec::new();

    for RawRule {
        pattern,
        intent,
        note,
    } in rules
    {
        new.push(Rule {
            pattern: parse_regex(&pattern, classes)?,
            intent,
            note,
        })
    }

    Ok(new)
}

/// Substitute class names and parse as regex
/// TODO Tests!
fn parse_regex(pattern: &str, classes: &Classes) -> Result<Regex, Error> {
    // Substitute class names
    let pattern = substitute_classes(pattern, classes)?;

    // Parse as regex
    Regex::new(&pattern).map_err(|_err| Error::Generic(0, format!("Failed to parse rule as regex")))
}

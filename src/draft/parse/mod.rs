#[cfg(test)]
mod tests;

use std::collections::HashMap;

use fancy_regex::Regex;
use fancy_regex_macro::regex;

use super::{
    replace::replace_classes, statements::split_statements, Classes, Draft, Message::*, Mode, Note,
    RawRule, Rule, TestDraft,
};
use crate::{error::Error, REGEX_MATCH_FAIL};

impl Draft {
    /// Parse Phonet `Draft` from file
    pub fn from(file: &str) -> Result<Draft, Error> {
        // Split file into statements
        let statements = split_statements(file);

        // Field builders
        let mut messages = Vec::new();
        let mut mode: Option<Mode> = None;

        // Field builders without regex parsed
        let mut raw_rules = Vec::new();
        let mut raw_classes: Classes = HashMap::new();

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
                        return parse_error!(line, ModeAlreadyDefined);
                    }

                    // Remove spaces
                    while chars.as_str().starts_with(' ') {
                        chars.next();
                    }

                    // Select mode
                    mode = Some(match Mode::from_options(chars.next(), chars.last()) {
                        Some(value) => value,
                        None => return parse_error!(line, InvalidModeSpecifier),
                    });
                }

                // Class
                '$' => {
                    let mut split = chars.as_str().split('=');

                    // Get class name
                    let Some(name) = split.next() else {
                        return parse_error!(line, NoClassName);
                    };
                    let name = name.trim();

                    // Check if name is valid
                    if !regex!(r"^\w+$").is_match(name).expect(REGEX_MATCH_FAIL) {
                        return parse_error!(line, InvalidClassName, name.to_string());
                    }

                    // Check that class name does not exist
                    if raw_classes.get(name).is_some() {
                        return parse_error!(line, ClassAlreadyExists, name.to_string());
                    }

                    // Get class pattern
                    let Some(pattern) = split.next() else {
                        return parse_error!(line, NoClassPattern, name.to_string());
                    };
                    let pattern = pattern.replace(' ', "");

                    // Add class
                    // Wrap value in NON-CAPTURING GROUP (just in case)
                    // This is non-capturing, for classes to work with back-references
                    // otherwise classes would be inherently capturing, and count towards group index in back-reference
                    raw_classes.insert(
                        name.trim().to_string(),
                        (format!("(?:{})", pattern.trim()), line),
                    );
                }

                // Rule
                '+' | '!' => {
                    // `+` for true, `!` for false
                    let intent = operator == '+';

                    let pattern = chars.as_str().replace(' ', "");

                    // Get most recent note, owned
                    let note = last_note.clone();

                    // Add rule
                    raw_rules.push(RawRule {
                        intent,
                        pattern,
                        note,
                        line,
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

                        // Unknown or no character
                        _ => {
                            return parse_error!(line, InvalidTestIntent);
                        }
                    };

                    // Split at space
                    for word in chars.as_str().split_whitespace() {
                        let word = word.trim().to_string();

                        // Add test
                        if !word.is_empty() {
                            messages.push(Test(TestDraft { intent, word }));
                        }
                    }
                }

                // Note
                '*' => {
                    let mut note = chars.as_str().trim();

                    if note.is_empty() {
                        return parse_error!(line, EmptyNote);
                    }

                    // Add note if not quiet reason
                    if note.starts_with(':') {
                        // Quiet reason - don't add note
                        chars.next();
                        note = chars.as_str().trim();
                    } else {
                        // Add note to messages
                        messages.push(Info(Note(note.to_string())));
                    }

                    // Add note
                    last_note = Some(Note(note.to_string()));
                }

                // Unknown line operator
                _ => {
                    return parse_error!(line, UnknownStatementOperator, operator);
                }
            }
        }

        // Get amount of tests in messages
        let test_count = messages.iter().filter(|msg| msg.is_test()).count();

        // Use default mode if none specified
        let mode = mode.unwrap_or_default();

        Ok(Self {
            rules: parse_rules(&raw_rules, &raw_classes)?,
            raw_rules,
            messages,
            mode,
            test_count,
            raw_classes,
        })
    }
}

/// Parse each rule in list
fn parse_rules(rules: &[RawRule], classes: &Classes) -> Result<Vec<Rule>, Error> {
    let mut new = Vec::new();

    for RawRule {
        pattern,
        intent,
        note,
        line,
    } in rules
    {
        new.push(Rule {
            pattern: parse_regex(pattern, classes, *line)?,
            intent: *intent,
            note: note.clone(),
        })
    }

    Ok(new)
}

/// Substitute class names and parse as regex
fn parse_regex(pattern: &str, classes: &Classes, line: usize) -> Result<Regex, Error> {
    // Substitute class names
    let pattern = replace_classes(pattern, classes, line)?;

    // Parse as regex
    match Regex::new(&pattern) {
        Ok(regex) => Ok(regex),
        Err(err) => parse_error!(line, RegexParseFail, err),
    }
}

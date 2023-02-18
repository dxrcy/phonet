#[cfg(test)]
mod tests;

use std::collections::HashMap;

use fancy_regex::Regex;

use super::{substitute::substitute_classes, Note, Rule};
use crate::Error;

// use statements::split_statements;
// use substitute::substitute_classes;

/// Alias for `HashMap` of `String` and `String`, for raw classes
pub type Classes = HashMap<String, String>;

/// Mirrors `Rule` struct, but with `String` instead of `Regex`
pub struct RawRule {
    pub intent: bool,
    pub pattern: String,
    pub note: Option<Note>,
}

/// Parse each rule in list
pub fn parse_rules(rules: Vec<RawRule>, classes: &Classes) -> Result<Vec<Rule>, Error> {
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
fn parse_regex(pattern: &str, classes: &Classes) -> Result<Regex, Error> {
    // Substitute class names
    let pattern = substitute_classes(pattern, classes)?;

    // Parse as regex
    Regex::new(&pattern).map_err(|_err| Error::Generic(0, format!("Failed to parse rule as regex")))
}

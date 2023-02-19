#[cfg(test)]
mod tests;

use fancy_regex::Regex;

use super::{replace::replace_classes, types::*, Rule};
use crate::Error;

/// Parse each rule in list
pub(super) fn parse_rules(rules: Vec<RawRule>, classes: &Classes) -> Result<Vec<Rule>, Error> {
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
    let pattern = replace_classes(pattern, classes)?;

    // Parse as regex
    Regex::new(&pattern).map_err(|_err| Error::Generic(0, format!("Failed to parse rule as regex")))
}
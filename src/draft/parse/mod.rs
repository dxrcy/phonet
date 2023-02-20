#[cfg(test)]
mod tests;

use fancy_regex::Regex;

use super::{replace::replace_classes, types::*, Rule};
use crate::{error::Error, parse_error};

/// Parse each rule in list
pub(super) fn parse_rules(rules: &[RawRule], classes: &Classes) -> Result<Vec<Rule>, Error> {
    let mut new = Vec::new();

    for RawRule {
        pattern,
        intent,
        note,
    } in rules
    {
        new.push(Rule {
            pattern: parse_regex(&pattern, classes)?,
            intent: *intent,
            note: note.clone(),
        })
    }

    Ok(new)
}

/// Substitute class names and parse as regex
fn parse_regex(pattern: &str, classes: &Classes) -> Result<Regex, Error> {
    // Substitute class names
    let pattern = replace_classes(pattern, classes)?;

    // Parse as regex
    match Regex::new(&pattern) {
        Ok(regex) => Ok(regex),
        Err(err) => parse_error!(0, RegexParseFail, err),
    }
}

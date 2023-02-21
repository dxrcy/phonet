#![allow(clippy::useless_format)] //TODO until error handling is implemented

/// Parsing of *Phonet* `Draft`
pub mod draft;
/// Error type for *Phonet*
pub mod error;
/// Running and displaying of *Phonet* `Draft`
pub mod outcome;

/// Color styles for stdout
mod color;
/// Generation of random words
mod generate;

pub use crate::{
    color::colorize,
    draft::Draft,
    outcome::{DisplayLevel, Outcome},
};

// pub use color::colorize;
// pub use draft::{Draft, Message, Mode, Note, Rule, TestDraft};
// pub use outcome::{DisplayLevel, FailKind, Outcome, PassStatus, TestOutcome};

/// Message for failed matching of static regex
const REGEX_MATCH_FAIL: &str = "Regex failed on 'match' method. This should never happen";

/// Adds '.min' to filename, before last file extension
///
/// Returns 'min.' before the entire name, if the filename has no extension
///
/// Returns an empty string, if `file` is an empty string
pub fn get_min_filename(file: &str) -> String {
    let mut split = file.split('.');

    // Get last file extension
    let ext = match split.next_back() {
        // Return blank string if filename is empty
        None | Some("") => return String::new(),

        Some(string) => string,
    };

    let rest: Vec<&str> = split.collect();
    if !rest.is_empty() {
        // Filename and extension
        rest.join(".") + ".min." + ext
    } else {
        // No extension or only extension (no filename)
        "min.".to_string() + ext
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_min_filename_works() {
        assert_eq!(get_min_filename(""), "");
        assert_eq!(get_min_filename("phonet"), "min.phonet");
        assert_eq!(get_min_filename("myfile.phonet"), "myfile.min.phonet");
        assert_eq!(get_min_filename("one.two.phonet"), "one.two.min.phonet");
    }
}

/// All parse functions for `Draft` struct
mod parse;

use fancy_regex::Regex;

use self::parse::parse_draft;
use crate::{outcome::Outcome, Error};

/// Parsed *Phonet* file
#[derive(Debug)]
pub struct Draft {
    /// List of defined rules
    pub rules: Vec<Rule>,
    /// List of messages to be displayed
    ///
    /// Each item may be a `Note` and `TestDraft`
    pub messages: Vec<Message<TestDraft>>,
    /// Transcription mode of file
    pub mode: Mode,
    /// Amount of tests in `messages` field
    pub test_count: usize,
    // minified: Minified,
}

/// Pattern rule
#[derive(Debug)]
pub struct Rule {
    /// Regex pattern
    pub pattern: Regex,
    /// Whether pattern should match or not, for a test to be valid
    pub intent: bool,
    /// Note for rule
    ///
    /// Reason given, if test fails from this rule
    pub note: Option<Note>,
}

/// Single message to be displayed
///
/// May be a `Info` (`Note`) and `Test`
#[derive(Debug, PartialEq)]
pub enum Message<T> {
    /// Plain text `Note`
    Info(Note),
    /// Test of generic type
    Test(T),
}

/// Wrapper for `String`
#[derive(Debug, Clone, PartialEq)]
pub struct Note(pub String);

/// Test that has not ran
#[derive(Debug)]
pub struct TestDraft {
    /// String to test
    pub word: String,
    /// Whether test should be valid or not, to pass
    pub intent: bool,
}

/// Transcription mode of file
#[derive(Debug)]
pub enum Mode {
    /// Use `~<>`
    Romanized,
    /// Use `~//`
    Broad,
    /// Use `~[]`
    Narrow,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Romanized
    }
}

impl Draft {
    /// Parse Phonet `Draft` from file
    pub fn from(file: &str) -> Result<Self, Error> {
        parse_draft(file)
    }

    /// Run drafted tests
    pub fn run(self) -> Outcome {
        Outcome::run(self)
    }
}

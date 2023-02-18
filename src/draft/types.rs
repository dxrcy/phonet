use std::collections::HashMap;

use fancy_regex::Regex;

/// Alias for `HashMap` of `String` and `String`, for raw classes
pub(super) type Classes = HashMap<String, String>;

/// Pattern rule
#[derive(Debug)]
pub struct Rule {
    /// Whether pattern should match or not, for a test to be valid
    pub pattern: Regex,
    /// Regex pattern
    pub intent: bool,
    /// Note for rule
    ///
    /// Reason given, if test fails from this rule
    pub note: Option<Note>,
}

// Scuffed equality check for `Rule`
impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.intent == other.intent
            && self.note == other.note
            // Regex must be stringified
            && self.pattern.to_string() == other.pattern.to_string()
    }
}

/// Mirrors `Rule` struct, but with `String` instead of `Regex`
pub(super) struct RawRule {
    /// Whether pattern should match or not, for a test to be valid
    pub intent: bool,
    /// Regex pattern, as `String`
    pub pattern: String,
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
#[derive(Debug, PartialEq)]
pub struct TestDraft {
    /// String to test
    pub word: String,
    /// Whether test should be valid or not, to pass
    pub intent: bool,
}

/// Transcription mode of file
#[derive(Debug, PartialEq)]
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

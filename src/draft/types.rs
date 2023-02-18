use fancy_regex::Regex;

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

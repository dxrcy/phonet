/// Shorthand for `Err(Error::Parse(ParseError::______))`
///
/// Only used in the `parse` module
macro_rules! parse_error {
    ( $line: expr, $kind: ident ) => {
        Err($crate::error::Error::Parse(
            $crate::error::ParseError::$kind,
            $line,
        ))
    };
    ( $line: expr, $kind: ident, $( $value: expr )* ) => {
        Err($crate::error::Error::Parse(
            $crate::error::ParseError::$kind(
                $( $value )*
            ),
            $line,
        ))
    };
}

/// Minify draft to string
mod minify;
/// Parse functions
mod parse;
/// Substitute class names recursively
mod replace;
/// Split file into statements
mod statements;

use std::collections::HashMap;

use fancy_regex::Regex;

// Holds types for `Draft` struct
// mod types;

pub(crate) use self::replace::replace_classes;
// pub use self::types::*;

// use std::collections::HashMap;

// use fancy_regex_macro::regex;

use self::minify::minify;
// parse::parse_rules, statements::split_statements
use crate::{error::Error, outcome::Outcome};

/// Alias for `HashMap` of `String` and `String`, for raw classes
pub(crate) type Classes = HashMap<String, String>;

/// Parsed *Phonet* file
///
/// Use `Draft::run` method to run tests, and convert to an `Outcome`
///
/// # Examples
///
/// ```
/// # use phonet::{Draft, draft::Mode};
/// let file = "
///   ~<>
///   $_ = [ptkaeiou]
///   * Some note
///     + ^ <_>+ $
///       ?+ kato
///       ?! x10
/// ";
///
/// let draft = Draft::from(file).unwrap();
///
/// assert_eq!(draft.rules.len(), 1);
/// assert_eq!(draft.messages.len(), 3);
/// assert_eq!(draft.mode, Mode::Romanized);
/// assert_eq!(draft.test_count, 2);
/// ```
///
/// # Errors
///
/// If the file contains invalid syntax.
///
/// ```
/// # use phonet::Draft;
/// use phonet::error::{Error, ParseError::*};
///
/// let draft = Draft::from("@");
///
/// assert!(matches!(draft, Err(
///     Error::Parse(UnknownStatementOperator('@'), _)
/// )));
/// ```
#[derive(Debug, PartialEq)]
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

    pub(crate) raw_rules: Vec<RawRule>,
    pub(crate) raw_classes: Classes,
}

/// Pattern rule for `Draft`
#[derive(Debug, Clone)]
pub struct Rule {
    /// Whether pattern should match or not, for a test to be valid
    pub pattern: Regex,
    /// Regex pattern
    pub intent: bool,
    /// Note for rule (optional)
    ///
    /// Reason given, if test fails from this rule
    pub note: Option<Note>,
}

/// Mirrors `Rule` struct, but with `String` instead of `Regex`
#[derive(Debug, PartialEq)]
pub(crate) struct RawRule {
    /// Regex pattern, as `String`
    pub pattern: String,
    /// Whether pattern should match or not, for a test to be valid
    pub intent: bool,
    /// Note for rule
    ///
    /// Reason given, if test fails from this rule
    pub note: Option<Note>,
}

/// Single message to be displayed in `Draft` and `Outcome`
///
/// May be a `Info` (`Note`) and `Test`
///
/// `Test` type should hold `TestDraft` or `TestOutcome`, for `Draft` and `Outcome` structs respectively
#[derive(Debug, PartialEq)]
pub enum Message<T> {
    /// Plain text `Note`
    Info(Note),
    /// Test of generic type
    Test(T),
}

/// Wrapper for `String`
///
///TODO Remove this - use string
#[derive(Debug, Clone, PartialEq)]
pub struct Note(pub String);

/// Test that has not yet ran, for `Draft`
#[derive(Debug, PartialEq, Clone)]
pub struct TestDraft {
    /// String to test
    pub word: String,
    /// Whether test should be valid or not to pass
    pub intent: bool,
}

/// Transcription mode of file
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    /// Use `~<>`
    Romanized,
    /// Use `~//`
    Broad,
    /// Use `~[]`
    Narrow,
}

impl Draft {
    /// Run drafted tests
    pub fn run(&self) -> Outcome {
        Outcome::run(self)
    }

    /// Returns a minified version of the original file of the `Draft`
    ///
    /// If `with_tests` is true, the minified string will include tests
    pub fn minify(&self, with_tests: bool) -> Result<String, Error> {
        minify(
            self.mode,
            &self.raw_classes,
            &self.raw_rules,
            &self.messages,
            with_tests,
        )
    }
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

impl<T> Message<T> {
    /// Returns `true` if self is `Info`
    pub fn is_note(&self) -> bool {
        matches!(self, Self::Info(_))
    }

    /// Returns `true` if self is `Test`
    pub fn is_test(&self) -> bool {
        matches!(self, Self::Test(_))
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Romanized
    }
}

impl Mode {
    /// Get `Mode` from characters
    ///
    /// Returns `None` if characters are not valid
    pub fn from(first: char, last: char) -> Option<Self> {
        use Mode::*;

        Some(match (first, last) {
            ('<', '>') => Romanized,
            ('/', '/') => Broad,
            ('[', ']') => Narrow,

            _ => return None,
        })
    }

    /// Get `Mode` from optional characters
    ///
    /// Returns `None` if characters are not valid, or any characters are `None`
    pub fn from_options(first: Option<char>, last: Option<char>) -> Option<Self> {
        match (first, last) {
            (Some(a), Some(b)) => Self::from(a, b),
            _ => None,
        }
    }

    /// Convert `Mode` to basic characters
    pub fn as_str(self) -> &'static str {
        use Mode::*;

        match self {
            Romanized => "<>",
            Broad => "//",
            Narrow => "[]",
        }
    }
}

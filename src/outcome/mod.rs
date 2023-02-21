/// Display function for `Outcome` struct
mod display;
/// Run function for `Outcome` struct
mod run;

use clap::{builder::PossibleValue, ValueEnum};

pub(crate) use self::run::{validate_test, Validity};

use crate::draft::{Message, Note};

/// Outcome of tests ran from `Draft`
///
///TODO Examples
#[derive(Debug)]
pub struct Outcome {
    /// Messages to display
    pub messages: Vec<Message<TestOutcome>>,
    /// Amount of failed tests ran
    pub fail_count: usize,
}

/// Outcome of `TestDraft` that was ran
#[derive(Debug, PartialEq)]
pub struct TestOutcome {
    /// String that was tested
    pub word: String,
    /// Whether test should have been valid or not to pass
    pub intent: bool,
    /// Whether the test has passed or not
    pub status: PassStatus,
}

/// Status of test that was ran
#[derive(Debug, PartialEq)]
pub enum PassStatus {
    /// Test passed
    ///
    /// Intent and validity matched
    Pass,
    /// Test failed
    ///
    /// Intent and validity did not match
    Fail(FailKind),
}

/// The manner in which a test failed
#[derive(Debug, PartialEq)]
pub enum FailKind {
    /// The test was supposed to not match the rules, however it did
    ShouldBeInvalid,
    /// The test was invalid, but should have matched
    ///
    /// No reason was given for this fail
    NoReasonGiven,
    /// The test was invalid, but should have matched
    ///
    /// A custom reason was given to the rule of which this test failed against
    CustomReason(Note),
}

/// The kinds of messages to display to the output, when `Outcome::display` is called
#[derive(Debug, Clone, Copy)]
pub enum DisplayLevel {
    /// Show everything: passed or failed tests, and notes
    ShowAll,
    /// Show failed tests and notes, but not passes
    IgnorePasses,
    /// Show only failed tests, not passed tests or notes
    OnlyFails,
    /// Show nothing: not passed or failed tests, or notes
    HideAll,
}

impl PassStatus {
    /// Returns `true` if self is `Pass`
    pub fn is_pass(&self) -> bool {
        matches!(self, Self::Pass)
    }

    /// Returns `true` if self is `Fail`
    pub fn is_fail(&self) -> bool {
        matches!(self, Self::Fail(_))
    }
}

impl Default for DisplayLevel {
    fn default() -> Self {
        Self::ShowAll
    }
}

// Custom implementation, for argument aliases
impl ValueEnum for DisplayLevel {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        // `help` values must mirror comments
        Some(match self {
            Self::ShowAll => PossibleValue::new("show-all")
                .aliases(["s", "show", "sa", "showall"])
                .help("Show everything: passed or failed tests, and notes"),

            Self::IgnorePasses => PossibleValue::new("ignore-passes")
                .aliases(["i", "ignore-passes", "ignore", "ip"])
                .help("Show failed tests and notes, but not passes"),

            Self::OnlyFails => PossibleValue::new("only-fails")
                .aliases(["o", "f", "only", "fails", "of", "onlyfails"])
                .help("Show only failed tests, not passed tests or notes"),

            Self::HideAll => PossibleValue::new("hide-all")
                .aliases(["h", "hide", "ha", "hideall"])
                .help("Show nothing: not passed or failed tests, or notes"),
        })
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::ShowAll,
            Self::IgnorePasses,
            Self::OnlyFails,
            Self::HideAll,
        ]
    }
}

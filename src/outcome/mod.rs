/// Display function for `Outcome` struct
mod display;
/// Run function for `Outcome` struct
mod run;

pub(crate) use run::{validate_test, Validity};

use crate::draft::{Message, Note};

#[derive(Debug)]
pub struct Outcome {
    pub messages: Vec<Message<TestOutcome>>,
    pub fail_count: usize,
}

#[derive(Debug, PartialEq)]
pub struct TestOutcome {
    pub word: String,
    pub intent: bool,
    pub status: PassStatus,
}

#[derive(Debug, PartialEq)]
pub enum PassStatus {
    Pass,
    Fail(FailKind),
}

#[derive(Debug, PartialEq)]
pub enum FailKind {
    ShouldBeInvalid,
    NoReasonGiven,
    CustomReason(Note),
}

#[derive(Debug, Clone, Copy)]
pub enum DisplayLevel {
    ShowAll,
    IgnorePasses,
    OnlyFails,
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

use clap::{builder::PossibleValue, ValueEnum};

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

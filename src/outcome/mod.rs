/// Color styles for stdout
mod colorize;
/// Display function for `Outcome` struct
mod display;
/// Run function for `Outcome` struct
mod run;

use crate::draft::{Message, Note};

#[derive(Debug)]
pub struct Outcome {
    /// TODO Rename to `messages` ?
    pub list: Vec<Message<TestOutcome>>,
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

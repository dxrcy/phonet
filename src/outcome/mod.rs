/// Display function for `Outcome` struct
mod display;
/// Run function for `Outcome` struct
mod run;

use crate::draft::{Message, Note};

#[derive(Debug)]
pub struct Outcome {
    pub list: Vec<Message<TestOutcome>>,
    pub fail_count: usize,
}

#[derive(Debug, PartialEq)]
pub struct TestOutcome {
    pub intent: bool,
    pub word: String,
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

pub enum DisplayLevel {
    ShowAll,
    IgnorePasses,
    OnlyFails,
    HideAll,
}

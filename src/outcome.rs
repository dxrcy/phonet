use crate::draft::{Draft, Message, Note};

pub struct Outcome<'a> {
    reasons: Vec<Note>,
    list: Vec<Message<TestOutcome<'a>>>,
    fail_count: usize,
}

struct TestOutcome<'a> {
    intent: bool,
    word: String,
    status: PassStatus<'a>,
}

enum PassStatus<'a> {
    Pass,
    Fail(FailKind<'a>),
}

enum FailKind<'a> {
    ShouldBeInvalid,
    NoReasonGiven,
    Custom(&'a Note),
}

pub enum DisplayLevel {
    ShowAll,
    IgnorePasses,
    OnlyFails,
    HideAll,
}

impl<'a> Outcome<'a> {
    pub fn run(draft: &Draft) -> Self {
        todo!()
    }

    pub fn display(&self, display_level: DisplayLevel, no_color: bool) {
        todo!()
    }
}

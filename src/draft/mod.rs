mod parse;
mod statements;
mod substitute;

use fancy_regex::Regex;

use crate::{outcome::Outcome, Error};

use self::parse::parse_draft;

#[derive(Debug)]
pub struct Draft {
    pub rules: Vec<Rule>,
    pub messages: Vec<Message<TestDraft>>,
    pub mode: Mode,
    // minified: Minified,
}

#[derive(Debug)]
pub struct Rule {
    pub intent: bool,
    pub pattern: Regex,
    pub note: Option<Note>,
}

#[derive(Debug)]
pub enum Message<T> {
    Info(Note),
    Test(T),
}

#[derive(Debug, Clone)]
pub struct Note(String);

#[derive(Debug)]
pub struct TestDraft {
    pub intent: bool,
    pub word: String,
}

#[derive(Debug)]
pub enum Mode {
    Romanized,
    Broad,
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

    /// Get amount of tests in `messages` field
    pub fn test_count(&self) -> usize {
        self.messages
            .iter()
            .filter(|msg| matches!(msg, Message::Test(_)))
            .count()
    }

    /// Run tests
    pub fn run(&self) -> Outcome {
        Outcome::run(self)
    }
}

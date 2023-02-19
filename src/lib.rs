#![allow(clippy::useless_format)] //TODO until error handling is implemented

/// Parsing of *Phonet* `Draft`
mod draft;
/// Error type for *Phonet*
mod error;
/// Running and displaying of *Phonet* `Draft`
mod outcome;

pub use draft::{Draft, Message, Mode, Note, Rule, TestDraft};
pub use error::Error;
pub use outcome::{DisplayLevel, FailKind, Outcome, PassStatus, TestOutcome};

/// Message for failed matching of static regex
const REGEX_MATCH_FAIL: &str = "Regex failed on 'match' method. This should never happen";

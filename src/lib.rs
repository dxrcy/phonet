mod draft;
mod outcome;

use thiserror::Error;

pub use draft::{Draft, Mode};
pub use outcome::{DisplayLevel, Outcome};

const REGEX_MATCH_FAIL: &str = "Regex failed on 'match' method. This should never happen";

#[derive(Error, Debug)]
pub enum Error {
    #[error("Generic error '{1}' on line {0}")]
    Generic(usize, String),
}
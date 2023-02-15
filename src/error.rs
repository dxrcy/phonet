use thiserror::Error;

/// Error type for *Phonet*
#[derive(Error, Debug)]
pub enum Error {
    /// Generic error type
    /// TODO Remove this
    #[error("Generic error '{1}' on line {0}")]
    Generic(usize, String),
}

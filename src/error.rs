use thiserror::Error;

/// Error type for *Phonet*
#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while parsing: {0}, at line {1}")]
    Parse(ParseError, usize),

    #[error("Missing 'any' class. Use `$_ = ___` to define it")]
    MissingAnyClass,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Mode already defined")]
    ModeAlreadyDefined,

    #[error("Invalid mode specifier, at line")]
    InvalidModeSpecifier,

    #[error("No class name given, at line")]
    NoClassName,

    #[error("Invalid class name '{0}'")]
    InvalidClassName(String),

    #[error("Class already exists named '{0}'")]
    ClassAlreadyExists(String),

    #[error("No pattern was given for class named '{0}'")]
    NoClassPattern(String),

    #[error("Missing or invalid test intent identifier")]
    InvalidTestIntent,

    #[error("Note cannot be empty")]
    EmptyNote,

    #[error("Unknown statement operator '{0}'")]
    UnknownStatementOperator(char),

    #[error("Unexpected class name opening bracket in regex pattern")]
    UnexpectedClassNameOpen,

    #[error("Unexpected class name closing bracket in regex pattern")]
    UnexpectedClassNameClose,

    #[error("Class not found named '{0}'")]
    ClassNotFound(String),

    #[error("Unexpected end of regex pattern for class name")]
    UnexpectedPatternEnd,

    #[error("Failed to parse rule pattern as regex - {0}")]
    RegexParseFail(fancy_regex::Error),
}

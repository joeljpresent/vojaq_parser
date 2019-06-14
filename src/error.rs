use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ParsingError {
    BadEscapedSequence(char)
}

impl Error for ParsingError {}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::BadEscapedSequence(c) => {
                write!(f, r"Bad escaped sequence: \{}", c)
            }
        }
    }
}

pub type ParsingResult<T> = Result<T, ParsingError>;
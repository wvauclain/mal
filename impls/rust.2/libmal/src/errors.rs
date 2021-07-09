#[derive(Clone, Copy, Debug)]
pub enum ParseError {
    /// EOF is reached before a terminating double-quote is found
    UnbalancedString,
    /// EOF is reached before reaching the end of a list
    UnbalancedParens,
    /// EOF is reached when a Form is expected
    MissingForm,
}

impl From<&ParseError> for ParseError {
    fn from(err: &ParseError) -> ParseError {
        *err
    }
}

use std::fmt::{Display, Error, Formatter};
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                ParseError::UnbalancedString => "unbalanced string",
                ParseError::UnbalancedParens => "unbalanced parens",
                ParseError::MissingForm => "reached EOF while searching for form",
            }
        )
    }
}

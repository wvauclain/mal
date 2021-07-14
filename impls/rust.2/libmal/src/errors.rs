#[derive(Clone, Copy, Debug)]
pub enum ParseError {
    /// EOF is reached before a terminating double-quote is found
    UnbalancedString,
    /// EOF is reached before reaching the end of a list
    UnbalancedParens,
    UnbalancedSquareBrackets,
    UnbalancedCurlyBrackets,
    UnexpectedCloseParen,
    UnexpectedCloseSquareBracket,
    UnexpectedCloseCurlyBracket,
    UnexpectedCompound,
    KeyWithoutValue,
    /// EOF is reached when a Form is expected
    MissingForm,
}

impl From<&ParseError> for ParseError {
    fn from(err: &ParseError) -> ParseError {
        *err
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                ParseError::UnbalancedString => "unbalanced string",
                ParseError::UnbalancedParens => "unbalanced parens",
                ParseError::UnbalancedSquareBrackets => "unbalanced square brackets",
                ParseError::UnbalancedCurlyBrackets => "unbalanced curly brackets",
                ParseError::UnexpectedCloseParen => "unexpected close paren",
                ParseError::UnexpectedCloseSquareBracket => "unexpected close square bracket",
                ParseError::UnexpectedCloseCurlyBracket => "unexpected close curly bracket",
                ParseError::UnexpectedCompound => "compound value used as a key in a map",
                ParseError::KeyWithoutValue => "found key without corresponding value",
                ParseError::MissingForm => "reached EOF while searching for form",
            }
        )
    }
}

impl std::error::Error for ParseError {}

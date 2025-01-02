//! Errors that may be encountered while parsing

use std::error;
use std::fmt;

use lalrpop_util;

use super::lexer;
use super::token;

/// The kind of error encountered while parsing.
///
/// This is non-exhaustive and expected to grow as the parser matures.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorKind {
    InvalidInteger,
    Other,
}

/// An error encountered while parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    source: lalrpop_util::ParseError<usize, token::Token, lexer::LexError>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.source {
            lalrpop_util::ParseError::User { error } => match error {
                lexer::LexError::InvalidInteger(parse_error) => {
                    write!(f, "Failed to parse integer: {}", parse_error)
                }
                lexer::LexError::Other => {
                    write!(f, "Unexpected error parsing: {:?}", self.source)
                }
            },
            lalrpop_util::ParseError::InvalidToken { location } => {
                write!(f, "Invalid token encountered at character {}", location)
            }
            lalrpop_util::ParseError::UnrecognizedEof {
                location,
                expected: _,
            } => {
                write!(f, "Unexpected EOF encountered at character {}", location)
            }
            lalrpop_util::ParseError::UnrecognizedToken {
                token: (start, token, end),
                expected: _,
            } => {
                write!(
                    f,
                    "Unexpected token ({}) encountered from characters {} to {}",
                    token, start, end
                )
            }
            lalrpop_util::ParseError::ExtraToken {
                token: (start, token, end),
            } => {
                write!(
                    f,
                    "Extra token ({}) encountered from characters {} to {}",
                    token, start, end
                )
            }
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.source)
    }
}

impl From<lalrpop_util::ParseError<usize, token::Token, lexer::LexError>> for ParseError {
    fn from(value: lalrpop_util::ParseError<usize, token::Token, lexer::LexError>) -> Self {
        ParseError { source: value }
    }
}

impl ParseError {
    pub fn kind(&self) -> ParseErrorKind {
        match &self.source {
            lalrpop_util::ParseError::User { error } => match error {
                lexer::LexError::InvalidInteger(_) => ParseErrorKind::InvalidInteger,
                lexer::LexError::Other => ParseErrorKind::Other,
            },
            _ => ParseErrorKind::Other,
        }
    }
}

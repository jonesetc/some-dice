use std::error;
use std::fmt;
use std::num;

use logos::{Logos, SpannedIter};

use super::token;

pub(super) type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub(super) struct Lexer<'input> {
    token_stream: SpannedIter<'input, token::Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            token_stream: token::Token::lexer(input).spanned(),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Spanned<token::Token, usize, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
            .next()
            .map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(super) enum LexError {
    InvalidInteger(num::ParseIntError),
    #[default]
    Other,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for LexError {}

impl From<num::ParseIntError> for LexError {
    fn from(err: num::ParseIntError) -> Self {
        LexError::InvalidInteger(err)
    }
}

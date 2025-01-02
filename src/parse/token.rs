use std::fmt;

use logos::Logos;

use super::lexer;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(error = lexer::LexError)]
#[logos(skip r"(?&whitespace)")]
#[logos(skip r"(?&comment)")]
#[logos(subpattern number = r"[0-9]+")]
#[logos(subpattern string = r#""[^"]*""#)]
#[logos(subpattern identifier = "[A-Z_]+")]
#[logos(subpattern whitespace = r"\s+")]
#[logos(subpattern comment = r"\\[^\\]*\\")]
pub(super) enum Token {
    // Keywords
    #[token("output")]
    OutputKeyword,
    #[token("if")]
    IfKeyword,
    #[token("else")]
    ElseKeyword,
    #[token("loop")]
    LoopKeyword,
    #[token("over")]
    OverKeyword,
    #[token("function")]
    FunctionKeyword,
    #[token("result")]
    ResultKeyword,
    #[token("set")]
    SetKeyword,
    #[token("to")]
    ToKeyword,

    // Literals
    #[regex(r"(?&number)", |lex| lex.slice().parse())]
    IntegerLiteral(i32),
    #[regex(r"(?&string)", |lex| {
        let with_quotes = lex.slice();
        with_quotes[1..with_quotes.len()-1].to_string()
    })]
    StringLiteral(String),

    // Identifiers
    #[regex(r"(?&identifier)", |lex| lex.slice().to_string())]
    VariableName(String),

    // Types
    #[token("n")]
    Number,
    #[token("d")]
    Dice,
    #[token("s")]
    Sequence,

    // Separator symbols
    #[token(",")]
    Comma,
    #[token("..")]
    Range,
    #[token(":")]
    Colon,

    // Wrapping Symbols
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("\"")]
    DoubleQuote,

    // Operation Symbols
    #[token("!")]
    Bang,
    #[token("#")]
    Hash,
    #[token("^")]
    Caret,
    #[token("*")]
    Asterisk,
    #[token("/")]
    ForwardSlash,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("&")]
    Ampersand,
    #[token("|")]
    Pipe,
    #[token("=")]
    Equal,
    #[token("!=")]
    BangEqual,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("@")]
    Access,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Token::OutputKeyword => write!(f, "output"),
            Token::IfKeyword => write!(f, "if"),
            Token::ElseKeyword => write!(f, "else"),
            Token::LoopKeyword => write!(f, "loop"),
            Token::OverKeyword => write!(f, "over"),
            Token::FunctionKeyword => write!(f, "function"),
            Token::ResultKeyword => write!(f, "result"),
            Token::SetKeyword => write!(f, "set"),
            Token::ToKeyword => write!(f, "to"),
            Token::IntegerLiteral(int) => write!(f, "{}", int),
            Token::StringLiteral(string) => write!(f, "{}", string),
            Token::VariableName(var) => write!(f, "{}", var),
            Token::Number => write!(f, "n"),
            Token::Dice => write!(f, "d"),
            Token::Sequence => write!(f, "s"),
            Token::Comma => write!(f, ","),
            Token::Range => write!(f, ".."),
            Token::Colon => write!(f, ":"),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::OpenBracket => write!(f, "["),
            Token::CloseBracket => write!(f, "]"),
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
            Token::DoubleQuote => write!(f, "\""),
            Token::Bang => write!(f, "!"),
            Token::Hash => write!(f, "#"),
            Token::Caret => write!(f, "^"),
            Token::Asterisk => write!(f, "*"),
            Token::ForwardSlash => write!(f, "/"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Ampersand => write!(f, "&"),
            Token::Pipe => write!(f, "|"),
            Token::Equal => write!(f, "="),
            Token::BangEqual => write!(f, "!="),
            Token::Less => write!(f, "<"),
            Token::Greater => write!(f, ">"),
            Token::LessEqual => write!(f, "<="),
            Token::GreaterEqual => write!(f, ">="),
            Token::Access => write!(f, "@"),
        }
    }
}

use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Ident(String),

    // Literals
    String(String),
    Int(String),
    Float(String),
    Bool(String),

    // Keywords
    And,
    Else,
    For,
    Fun,
    If,
    Import,
    Let,
    Match,
    Mut,
    Not,
    Or,
    Return,
    While,

    // Operators
    Eq,
    EqEq,
    Neq,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Plus,
    Minus,
    Star,
    Slash,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Semi,
    Comma,
    Dot,
    Amp,
    Pipe,
    Bang,
    Backslash,

    // Miscellaneous
    Eof,
    Invalid(LexerError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LexerError {
    InvalidEscapeSequence(char),
    Expected(char),
    UnknownChar(char),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LexerError::*;
        match self {
            InvalidEscapeSequence(c) => write!(
                f,
                "Invalid character in escape sequence: '{}' (U+{:x})",
                c, *c as u32
            ),
            Expected(c) => write!(f, "Expected {} (U+{:x}) while lexing", *c, *c as u32),
            UnknownChar(c) => write!(f, "Unknown character '{}' (U+{:x}) in file", *c, *c as u32),
        }
    }
}

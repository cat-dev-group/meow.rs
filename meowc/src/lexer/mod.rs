//! Lexing is the simplest compilation phase. Its goal is to convert a stream
//! of characters into a Vec<Token> that can then be fed into the parser.

mod token;

use crate::errors::location::{EndPosition, Locatable, Location, Position};
use std::{collections::HashMap, path::Path, str::Chars};
use token::{LexerError, Token};

pub struct Lexer<'a> {
    current: char,
    next: char,
    filename: &'a Path,
    file_contents: &'a str,
    token_start_position: Position,
    current_position: Position,
    chars: Chars<'a>,
    keywords: HashMap<&'static str, Token>,
}

impl<'a> Locatable<'a> for Lexer<'a> {
    fn locate(&self) -> Location<'a> {
        let end = EndPosition::new(self.current_position.index);
        Location::new(self.filename, self.token_start_position, end)
    }
}

type IterElem<'a> = Option<(Token, Location<'a>)>;

impl<'a> Lexer<'a> {
    pub fn get_keywords() -> HashMap<&'static str, Token> {
        let mut keywords: HashMap<&str, Token> = HashMap::new();

        keywords.insert("true", Token::Bool(true));
        keywords.insert("false", Token::Bool(false));
        keywords.insert("const", Token::Const);
        keywords.insert("elif", Token::Elif);
        keywords.insert("else", Token::Else);
        keywords.insert("fun", Token::Fun);
        keywords.insert("if", Token::If);
        keywords.insert("import", Token::Import);
        keywords.insert("let", Token::Let);
        keywords.insert("match", Token::Match);
        keywords.insert("return", Token::Return);

        keywords
    }

    pub fn new(filename: &'a Path, file_contents: &'a str) -> Lexer<'a> {
        let mut chars = file_contents.chars();
        let current = chars.next().unwrap_or('\0');
        let next = chars.next().unwrap_or('\0');
        Lexer {
            current,
            next,
            filename,
            file_contents,
            current_position: Position::begin(),
            token_start_position: Position::begin(),
            chars,
            keywords: Lexer::get_keywords(),
        }
    }

    fn at_end_of_input(&self) -> bool {
        self.current == '\0'
    }

    fn advance(&mut self) -> char {
        let ret = self.current;
        self.current = self.next;
        self.next = self.chars.next().unwrap_or('\0');
        self.current_position.advance(ret == '\n');
        ret
    }

    fn advance_with(&mut self, token: Token) -> IterElem<'a> {
        self.advance();
        Some((token, self.locate()))
    }

    fn advance2_with(&mut self, token: Token) -> IterElem<'a> {
        self.advance();
        self.advance_with(token)
    }

    fn get_slice_containing_current_token(&self) -> &'a str {
        &self.file_contents[self.token_start_position.index..self.current_position.index]
    }

    fn expect(&mut self, expected: char, token: Token) -> IterElem<'a> {
        if self.current == expected {
            self.advance_with(token)
        } else {
            self.advance_with(Token::Invalid(LexerError::Expected(expected)))
        }
    }

    fn advance_while<F>(&mut self, mut f: F) -> &'a str
    where
        F: FnMut(char, char) -> bool,
    {
        while f(self.current, self.next) && !self.at_end_of_input() {
            self.advance();
        }
        self.get_slice_containing_current_token()
    }

    fn lex_integer(&mut self) -> String {
        let start = self.current_position.index;

        while !self.at_end_of_input() && (self.current.is_digit(10) || self.current == '_') {
            self.advance();
        }

        let end = self.current_position.index;
        self.file_contents[start..end].replace('_', "")
    }

    fn lex_number(&mut self) -> IterElem<'a> {
        let integer_string = self.lex_integer();

        if self.current == '.' && self.next.is_digit(10) {
            self.advance();
            let float_string = integer_string + "." + &self.lex_integer();

            let float = float_string.parse().unwrap();
            Some((Token::Float(float), self.locate()))
        } else {
            let integer = integer_string.parse().unwrap();
            let location = self.locate();
            Some((Token::Int(integer), location))
        }
    }

    fn lex_string(&mut self) -> IterElem<'a> {
        self.advance();
        let mut contents = String::new();
        while self.current != '"' {
            let current_char = if self.current == '\\' {
                self.advance();
                match self.current {
                    '\\' | '\'' => self.current,
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '0' => '\0',
                    _ => {
                        let error = LexerError::InvalidEscapeSequence(self.current);
                        return self.advance2_with(Token::Invalid(error));
                    }
                }
            } else {
                self.current
            };
            contents.push(current_char);
            self.advance();
        }
        self.expect('"', Token::String(contents))
    }

    fn lex_singleline_comment(&mut self) -> IterElem<'a> {
        self.advance_while(|current, _| current != '\n');
        self.next()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Location<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.current, self.next) {
            ('\0', _) => {
                if self.current_position.index > self.file_contents.len() {
                    None
                } else {
                    self.advance_with(Token::EndOfInput)
                }
            }

            // Literals
            ('"', _) => self.lex_string(),
            (c, _) if c.is_digit(10) => self.lex_number(),

            // Comments
            ('/', '/') => self.lex_singleline_comment(),

            // Double char tokens
            ('=', '=') => self.advance2_with(Token::EqEq),
            ('!', '=') => self.advance2_with(Token::Neq),
            ('>', '=') => self.advance2_with(Token::GtEq),
            ('<', '=') => self.advance2_with(Token::LtEq),

            // Single char tokens
            ('=', _) => self.advance_with(Token::Eq),
            ('>', _) => self.advance_with(Token::Gt),
            ('<', _) => self.advance_with(Token::Lt),
            ('+', _) => self.advance_with(Token::Plus),
            ('-', _) => self.advance_with(Token::Minus),
            ('*', _) => self.advance_with(Token::Star),
            ('/', _) => self.advance_with(Token::Slash),
            ('(', _) => self.advance_with(Token::OpenParen),
            (')', _) => self.advance_with(Token::CloseParen),
            ('[', _) => self.advance_with(Token::OpenBracket),
            (']', _) => self.advance_with(Token::CloseBracket),
            ('{', _) => self.advance_with(Token::OpenBrace),
            ('}', _) => self.advance_with(Token::CloseBrace),
            (';', _) => self.advance_with(Token::Semicolon),
            (',', _) => self.advance_with(Token::Comma),
            ('.', _) => self.advance_with(Token::Dot),
            ('!', _) => self.advance_with(Token::Exclamation),
            ('?', _) => self.advance_with(Token::Question),
            ('&', _) => self.advance_with(Token::Ampersand),
            ('|', _) => self.advance_with(Token::Pipe),
            ('\\', _) => self.advance_with(Token::Backslash),

            (
                '\n' | '\r' | '\t' | ' ' | '\u{000B}' | '\u{000C}' | '\u{0085}' | '\u{200E}'
                | '\u{200F}' | '\u{2028}' | '\u{2029}',
                _,
            ) => self.advance_with(Token::Whitespace),
            (c, _) => self.advance_with(Token::Invalid(LexerError::UnknownChar(c))),
        }
    }
}

use core::fmt::{Display, Formatter, Result};
use std::ops::Deref;

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String(String),
    Number(f64),

    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
    Error,
}

pub struct Token {
    r#type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            r#type,
            lexeme,
            line,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.r#type, self.lexeme, self.line)
    }
}

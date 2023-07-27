use core::fmt;

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
    String,
    Number,

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
}

pub struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: Option<Box<dyn std::any::Any>>,
    line: usize,
}

impl Token {
    pub fn new(
        r#type: TokenType,
        lexeme: String,
        literal: Option<Box<dyn std::any::Any>>,
        line: usize,
    ) -> Token {
        Token {
            r#type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {:?} {}",
            self.r#type, self.lexeme, self.literal, self.line
        )
    }
}

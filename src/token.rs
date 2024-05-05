use core::fmt::{Display, Formatter, Result};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
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

    Identifier(String),
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

impl FromStr for TokenType {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<TokenType, ()> {
        match s {
            "or" => Ok(TokenType::Or),
            "and" => Ok(TokenType::And),
            "fn" => Ok(TokenType::Fn),
            "class" => Ok(TokenType::Class),
            "else" => Ok(TokenType::Else),
            "false" => Ok(TokenType::False),
            "true" => Ok(TokenType::True),
            "for" => Ok(TokenType::For),
            "if" => Ok(TokenType::If),
            "nil" => Ok(TokenType::Nil),
            "print" => Ok(TokenType::Print),
            "return" => Ok(TokenType::Return),
            "super" => Ok(TokenType::Super),
            "this" => Ok(TokenType::This),
            "var" => Ok(TokenType::Var),
            "while" => Ok(TokenType::While),
            _ => Ok(TokenType::Identifier(s.to_string())),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

pub struct Tokens(Vec<Token>);

impl Tokens {
    pub fn new() -> Tokens {
        Tokens(Vec::new())
    }

    pub fn push(&mut self, token: Token) {
        self.0.push(token)
    }

    pub fn clone(&self) -> Tokens {
        Tokens(self.0.clone())
    }
}

impl Display for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.iter().fold(Ok(()), |result, token| {
            result.and_then(|_| writeln!(f, "{}", token))
        })
    }
}

impl std::ops::Index<usize> for Tokens {
    type Output = Token;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }

    pub fn get_lexeme(&self) -> String {
        return self.lexeme.clone();
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.line)
    }
}

pub struct TokenBuilder {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl TokenBuilder {
    pub fn new() -> TokenBuilder {
        TokenBuilder {
            token_type: TokenType::Error,
            lexeme: "".to_string(),
            line: 0,
        }
    }

    pub fn build(self) -> Token {
        Token {
            token_type: self.token_type,
            lexeme: self.lexeme,
            line: self.line,
        }
    }

    pub fn token_type(mut self, token_type: TokenType) -> TokenBuilder {
        self.token_type = token_type;
        self
    }

    pub fn lexeme(mut self, lexeme: String) -> TokenBuilder {
        self.lexeme = lexeme;
        self
    }

    pub fn line(mut self, line: usize) -> TokenBuilder {
        self.line = line;
        self
    }
}

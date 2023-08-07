use core::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
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

pub fn keyword_map(keyword: String) -> TokenType {
    match keyword.as_str() {
        "or" => TokenType::Or,
        "and" => TokenType::And,
        "fn" => TokenType::Fn,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "true" => TokenType::True,
        "for" => TokenType::For,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier(keyword),
    }
}

pub struct Tokens(Vec<Token>);

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
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

impl Display for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.iter().fold(Ok(()), |result, token| {
            result.and_then(|_| writeln!(f, "{}", token))
        })
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.line)
    }
}

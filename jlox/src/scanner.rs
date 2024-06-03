use std::{fmt, str::FromStr};

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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for TokenType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.line)
    }
}

pub struct Scanner<'a> {
    code: &'a String,
    tokens: Vec<Token>,
    line: usize,
    start: usize,
    current: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(code: &'a String) -> Scanner {
        Scanner {
            code,
            tokens: vec![],
            line: 1,
            start: 0,
            current: 0,
        }
    }
}

impl Scanner<'_> {
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while self.is_not_finished() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), self.line));
        return self.tokens.clone();
    }

    fn is_not_finished(&self) -> bool {
        !self.is_finished()
    }

    fn is_finished(&self) -> bool {
        self.current >= self.code.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '\n' => self.line += 1,
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Star),
            ';' => self.add_token(TokenType::Semicolon),
            '/' if self.match_char('/') => {
                self.current += 1;
                while self.peek() != '\n' && self.is_not_finished() {
                    self.advance();
                }
            }
            '/' => self.add_token(TokenType::Slash),
            '>' if self.match_char('=') => self.add_token(TokenType::GreaterEqual),
            '>' => self.add_token(TokenType::Greater),
            '<' if self.match_char('=') => self.add_token(TokenType::LessEqual),
            '<' => self.add_token(TokenType::Less),
            '=' if self.match_char('=') => self.add_token(TokenType::EqualEqual),
            '=' => self.add_token(TokenType::Equal),
            '!' if self.match_char('=') => self.add_token(TokenType::BangEqual),
            '!' => self.add_token(TokenType::Bang),
            '"' => self.string(),
            _ => {
                if c.is_ascii_digit() {
                    self.number()
                } else if self.is_name_char(c) {
                    self.identifier()
                } else {
                    self.add_token(TokenType::Error)
                }
            }
        };
    }

    fn advance(&mut self) -> char {
        let res = self.code.as_bytes()[self.current] as char;
        self.current += 1;
        return res;
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.code[self.start..self.current].to_string();
        let token = Token::new(token_type, text, self.line);
        self.tokens.push(token)
    }

    fn match_char(&mut self, expected: char) -> bool {
        let curr = self.code.as_bytes()[self.current] as char;
        if self.is_finished() || curr != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn string(&mut self) {
        while self.peek() != '"' && self.is_not_finished() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        self.advance();
        let value: String = self.code[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String(value));
    }

    fn peek(&self) -> char {
        if self.is_finished() {
            return '\0';
        }
        return self.code.as_bytes()[self.current] as char;
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let num: f64 = self.code[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number(num));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.code.len() {
            return '\0';
        }
        return self.code.as_bytes()[self.current + 1] as char;
    }
    fn is_name_char(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphabetic() {
            self.advance();
        }
        let word: &str = &self.code[self.start..self.current];
        if let Ok(token_type) = TokenType::from_str(word) {
            self.add_token(token_type)
        } else {
            println!(
                "This should never happen, error creating token type from string value {}",
                word
            );
            self.add_token(TokenType::Error)
        };
    }
}

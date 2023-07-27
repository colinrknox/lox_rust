use super::token::{Token, TokenType};

pub struct Scanner {
    tokens: Vec<Token>,
    code: String,
    line: usize,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(code: String) -> Scanner {
        Scanner {
            tokens: Vec::new(),
            code,
            line: 1,
            start: 0,
            current: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        println!("{}", self.code);
        while !self.is_finished() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        if c == ' ' || c == '\r' || c == '\t' {
            return;
        }
        if c == '\n' {
            self.line += 1;
            return;
        }
        if c == '/' && self.match_char('/') {
            self.current += 1;
            while self.peek() != '\n' && !self.is_finished() {
                self.advance();
            }
            return;
        }
        self.tokens.push(match c {
            '(' => self.create_token(TokenType::LeftParen),
            ')' => self.create_token(TokenType::RightParen),
            '{' => self.create_token(TokenType::LeftBrace),
            '}' => self.create_token(TokenType::RightBrace),
            ',' => self.create_token(TokenType::Comma),
            '.' => self.create_token(TokenType::Dot),
            '-' => self.create_token(TokenType::Minus),
            '+' => self.create_token(TokenType::Plus),
            '*' => self.create_token(TokenType::Star),
            ';' => self.create_token(TokenType::Semicolon),
            '/' => self.create_token(TokenType::Slash),
            '>' => {
                if self.match_char('=') {
                    self.current += 1;
                    self.create_token(TokenType::GreaterEqual)
                } else {
                    self.create_token(TokenType::Greater)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.current += 1;
                    self.create_token(TokenType::LessEqual)
                } else {
                    self.create_token(TokenType::Less)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.current += 1;
                    self.create_token(TokenType::EqualEqual)
                } else {
                    self.create_token(TokenType::Equal)
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.current += 1;
                    self.create_token(TokenType::BangEqual)
                } else {
                    self.create_token(TokenType::Bang)
                }
            }
            _ => self.create_token(TokenType::EOF),
        });
    }

    fn create_token(&self, r#type: TokenType) -> Token {
        self.create_token_literal(r#type, None)
    }

    fn create_token_literal(
        &self,
        r#type: TokenType,
        literal: Option<Box<dyn std::any::Any>>,
    ) -> Token {
        Token::new(r#type, "".to_string(), literal, self.line)
    }

    fn match_char(&self, expected: char) -> bool {
        let curr = self.code.as_bytes()[self.current] as char;
        if self.is_finished() || curr != expected {
            return false;
        }
        return true;
    }

    fn advance(&mut self) -> char {
        let res = self.code.as_bytes()[self.current] as char;
        self.current += 1;
        return res;
    }

    fn is_finished(&self) -> bool {
        self.current >= self.code.len()
    }

    fn peek(&self) -> char {
        if self.is_finished() {
            return '\0';
        }
        return self.code.as_bytes()[self.current] as char;
    }
}

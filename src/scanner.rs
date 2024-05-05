use std::str::FromStr;

use crate::{lox::Lox, token::TokenBuilder};

use super::token::{Token, TokenType, Tokens};

pub trait Scan {
    fn scan_tokens(&mut self) -> Tokens;
    fn get_errors(&self) -> Lox;
}

pub struct Scanner {
    tokens: Tokens,
    code: String,
    line: usize,
    start: usize,
    current: usize,
    errors: Lox,
}

impl Scan for Scanner {
    fn scan_tokens(&mut self) -> Tokens {
        while !self.is_finished() {
            self.start = self.current;
            self.scan_token();
        }

        let token = create_token(TokenType::EOF, "".to_string(), self.line);
        self.tokens.push(token);
        return self.tokens.clone();
    }

    fn get_errors(&self) -> Lox {
        return self.errors.clone();
    }
}

fn create_token(token_type: TokenType, lexeme: String, line: usize) -> Token {
    // let text = self.code[self.start..self.current].to_string();
    TokenBuilder::new()
        .token_type(token_type)
        .lexeme(lexeme)
        .line(line)
        .build()
}

impl Scanner {
    pub fn new(code: String, errors: Lox) -> Scanner {
        Scanner {
            tokens: Tokens::new(),
            code,
            line: 1,
            start: 0,
            current: 0,
            errors,
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        if c.is_ascii_whitespace() {
            self.handle_new_line(c);
            return;
        }
        if self.is_comment_line(c) {
            self.handle_comment_line();
            return;
        } else if self.is_block_comment(c) {
            self.handle_block_comment();
            return;
        }
        let token = self.create_token_from_char(c);
        self.tokens.push(token);
    }

    fn handle_new_line(&mut self, c: char) {
        if is_new_line(c) {
            self.line += 1;
        }
    }
}

fn is_new_line(c: char) -> bool {
    c == '\n'
}

impl Scanner {
    fn is_comment_line(&mut self, c: char) -> bool {
        c == '/' && self.match_char('/')
    }

    fn match_char(&mut self, expected: char) -> bool {
        let curr = self.code.as_bytes()[self.current] as char;
        if self.is_finished() || curr != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn is_finished(&self) -> bool {
        self.current >= self.code.len()
    }

    fn handle_comment_line(&mut self) {
        self.current += 1;
        while self.peek() != '\n' && !self.is_finished() {
            self.advance();
        }
    }

    fn peek(&self) -> char {
        if self.is_finished() {
            return '\0';
        }
        return self.code.as_bytes()[self.current] as char;
    }

    fn advance(&mut self) -> char {
        let res = self.code.as_bytes()[self.current] as char;
        self.current += 1;
        return res;
    }

    fn is_block_comment(&mut self, c: char) -> bool {
        c == '/' && self.match_char('*')
    }

    fn handle_block_comment(&mut self) {
        self.current += 1;
        while self.peek() != '*' && !self.match_char('/') && !self.is_finished() {
            self.advance();
        }
        self.advance();
        self.advance();
    }

    fn create_token_from_char(&mut self, c: char) -> Token {
        match c {
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
                    self.create_token(TokenType::GreaterEqual)
                } else {
                    self.create_token(TokenType::Greater)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.create_token(TokenType::LessEqual)
                } else {
                    self.create_token(TokenType::Less)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.create_token(TokenType::EqualEqual)
                } else {
                    self.create_token(TokenType::Equal)
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.create_token(TokenType::BangEqual)
                } else {
                    self.create_token(TokenType::Bang)
                }
            }
            '"' => self.string(),
            _ => {
                if c.is_ascii_digit() {
                    self.number()
                } else if is_name_char(c) {
                    self.identifier()
                } else {
                    self.errors
                        .error(self.line, "Invalid character".to_string());
                    self.create_token(TokenType::Error)
                }
            }
        }
    }

    fn create_token(&self, token_type: TokenType) -> Token {
        let text = self.code[self.start..self.current].to_string();
        create_token(token_type, text, self.line)
    }
}

fn is_name_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

impl Scanner {
    fn identifier(&mut self) -> Token {
        while self.peek().is_ascii_alphabetic() {
            self.advance();
        }
        let word: &str = &self.code[self.start..self.current];
        if let Ok(token_type) = TokenType::from_str(word) {
            self.create_token(token_type)
        } else {
            println!(
                "This should never happen, error creating token type from string value {}",
                word
            );
            self.create_token(TokenType::Error)
        }
    }

    fn number(&mut self) -> Token {
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
        return self.create_token(TokenType::Number(num));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.code.len() {
            return '\0';
        }
        return self.code.as_bytes()[self.current + 1] as char;
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_finished() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        self.advance();
        let value: String = self.code[self.start + 1..self.current - 1].to_string();
        return self.create_token(TokenType::String(value));
    }
}

use super::token::{keyword_map, Token, TokenType};

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
            .push(Token::new(TokenType::EOF, "".to_string(), self.line));
        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        if c.is_ascii_whitespace() {
            if c == '\n' {
                self.line += 1;
            }
            return;
        }
        if c == '/' && self.match_char('/') {
            self.current += 1;
            while self.peek() != '\n' && !self.is_finished() {
                self.advance();
            }
            return;
        }
        let token = match c {
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
                } else if self.is_alpha(c) {
                    self.identifier()
                } else {
                    self.create_token(TokenType::Error)
                }
            }
        };
        self.tokens.push(token);
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_ascii_alphabetic() {
            self.advance();
        }
        let word: String = self.code[self.start..self.current].to_string();
        self.create_token(keyword_map(word))
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

    fn create_token(&self, r#type: TokenType) -> Token {
        let text = self.code[self.start..self.current].to_string();
        Token::new(r#type, text, self.line)
    }

    fn match_char(&mut self, expected: char) -> bool {
        let curr = self.code.as_bytes()[self.current] as char;
        if self.is_finished() || curr != expected {
            return false;
        }
        self.current += 1;
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

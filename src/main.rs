use core::fmt;
use std::{env, fs, io};

#[derive(Debug)]
enum TokenType {
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

struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: Option<Box<dyn std::any::Any>>,
    line: usize,
}

impl Token {
    fn new(
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

    fn from_tokentype(r#type: TokenType) -> Token {
        Self::from_type_literal(r#type, None)
    }

    fn from_type_literal(r#type: TokenType, literal: Option<Box<dyn std::any::Any>>) -> Token {
        Self::new(r#type, "".to_string(), literal, 0)
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

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: lox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(file: &String) {
    let contents: String = fs::read_to_string(file).unwrap();
    run(contents);
}

fn run_prompt() {
    loop {
        print!("> ");
        if let Ok(line) = io::read_to_string(io::stdin()) {
            run(line);
        } else {
            println!("Error: invalid expression");
        };
    }
}

fn run(code: String) -> Result<(), String> {
    let mut scanner = Scanner::new(code);
    let tokens: &Vec<Token> = scanner.scan_tokens();

    for t in tokens {
        println!("{}", t);
    }
    Ok(())
}

struct Scanner {
    tokens: Vec<Token>,
    code: String,
    line: usize,
    start: usize,
    current: usize,
}

impl Scanner {
    fn new(code: String) -> Scanner {
        Scanner {
            tokens: Vec::new(),
            code,
            line: 0,
            start: 0,
            current: 0,
        }
    }

    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_finished(self.code.len()) {
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
        if c == '/' && self.r#match('/') {
            return;
        }
        self.tokens.push(match c {
            '(' => Token::from_tokentype(TokenType::LeftParen),
            ')' => Token::from_tokentype(TokenType::RightParen),
            '{' => Token::from_tokentype(TokenType::LeftBrace),
            '}' => Token::from_tokentype(TokenType::RightBrace),
            ',' => Token::from_tokentype(TokenType::Comma),
            '.' => Token::from_tokentype(TokenType::Dot),
            '-' => Token::from_tokentype(TokenType::Minus),
            '+' => Token::from_tokentype(TokenType::Plus),
            '*' => Token::from_tokentype(TokenType::Star),
            ';' => Token::from_tokentype(TokenType::Semicolon),
            '/' => Token::from_tokentype(TokenType::Slash),
            '>' => {
                if self.r#match('=') {
                    Token::from_tokentype(TokenType::GreaterEqual)
                } else {
                    Token::from_tokentype(TokenType::Greater)
                }
            }
            '<' => {
                if self.r#match('=') {
                    Token::from_tokentype(TokenType::LessEqual)
                } else {
                    Token::from_tokentype(TokenType::Less)
                }
            }
            '=' => {
                if self.r#match('=') {
                    Token::from_tokentype(TokenType::EqualEqual)
                } else {
                    Token::from_tokentype(TokenType::Equal)
                }
            }
            '!' => {
                if self.r#match('=') {
                    Token::from_tokentype(TokenType::BangEqual)
                } else {
                    Token::from_tokentype(TokenType::Bang)
                }
            }
            _ => Token::from_tokentype(TokenType::EOF),
        });
    }

    fn r#match(&mut self, expected: char) -> bool {
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
}

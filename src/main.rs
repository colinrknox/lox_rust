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

struct Token<T: fmt::Display> {
    r#type: TokenType,
    lexeme: String,
    literal: Option<T>,
    line: usize,
}

impl<T> Token<T> {
    fn new(r#type: TokenType, lexeme: String, literal: Option<T>, line: usize) -> Token<T> {
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

impl<T> fmt::Display for Token<T> {
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

fn run(code: String) {
    let tokens: Vec<Token> = scan_tokens(code);

    for t in tokens {
        println!("{}", t);
    }
}

struct Scanner {
    tokens: Vec<Token>,
    line: usize,
    start: usize,
    current: usize,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            tokens: Vec::new(),
            line: 0,
            start: 0,
            current: 0,
        }
    }

    fn scan_tokens(&mut self, code: String) -> Vec<Token> {
        while !self.is_finished(code.len()) {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
        return self.tokens;
    }

    fn scan_token(&mut self) {}

    fn is_finished(&self, file_length: usize) -> bool {
        self.current > file_length
    }
}

fn scan<T>(code: String) -> Token<T> {
    let c = code.as_bytes()[0];
    match c {
        '(' => add_token(TokenType::LeftParen),
        _ => println!("unknown token"),
    }
}

fn add_token(r#type: TokenType) -> Token {
    add_token(r#type, None)
}

fn add_token(r#type: TokenType, literal: Option<T>) -> Token<T> {
    Token::new(r#type, "".to_string(), literal, 0)
}

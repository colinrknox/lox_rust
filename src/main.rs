use std::{env, fmt, fs};

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
    line: usize,
}

impl Token {
    fn new(r#type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            r#type,
            lexeme,
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
        write!(f, "{} {} {}", self.r#type, self.lexeme, self.line)
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
        if let Ok(line) = std::io::read_to_string(std::io::stdin()) {
            run(line);
        } else {
            println!("Error: invalid expression");
        };
    }
}

fn run(code: String) {
    let mut tokens: Vec<Token> = scan_tokens(code);

    for t in tokens {
        println!("{}", t);
    }
}

fn scan_tokens(code: String) -> Vec<Token> {
    Vec::new()
}

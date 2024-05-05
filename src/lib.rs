use crate::{
    lox::Lox,
    scanner::{Scan, Scanner},
    token::Tokens,
};

use ast::{eval_stmt, Expr, Object, Stmt};
use parser::Parser;
use std::{
    fs,
    io::{self, Write},
    process,
};

pub mod ast;
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod token;

pub fn stdin_interactive() {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        if buffer == "exit\n" {
            break;
        }
        if let Ok(_tokens) = run(buffer.clone()) {
        } else {
            println!("Error");
            process::exit(69);
        }
    }
}

pub fn run_file(file: &String) {
    let contents: String = fs::read_to_string(file).unwrap();
    if let Ok(tokens) = run(contents) {
        println!("{}", tokens);
    } else {
        println!("Error");
        process::exit(69);
    }
}

fn run(code: String) -> Result<Tokens, String> {
    let scanner = Scanner::new(code, Lox::new());
    let t = match run_with_scanner(scanner) {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens.clone());
            let stmts = parser.parse();
            for stmt in stmts {
                execute(stmt);
            }
            tokens
        }
        Err(string) => {
            println!("{}", string);
            process::exit(69);
        }
    };
    Ok(t)
}

fn execute(stmt: Stmt) -> Result<Object, Expr> {
    Ok(eval_stmt(stmt)?)
}

pub fn run_with_scanner<S: Scan>(mut scanner: S) -> Result<Tokens, String> {
    let error = scanner.get_errors();
    let tokens: Tokens = scanner.scan_tokens();
    if error.had_error {
        let mut error_string: String = "".to_string();
        for e in error.errors.clone() {
            error_string = format!("{}\n{}", error_string, e);
        }
        return Err(format!("{}", error));
    }
    Ok(tokens)
}

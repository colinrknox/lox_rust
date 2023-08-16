use crate::{
    lox::Lox,
    scanner::{Scan, Scanner},
    token::Tokens,
};

use std::{
    fs,
    io::{self, Write},
    process,
};
use wasm_bindgen::prelude::*;

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
        if let Ok(tokens) = run(buffer.clone()) {
            println!("{}", tokens);
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
    run_with_scanner(scanner)
}

pub fn run_with_scanner<S: Scan>(mut scanner: S) -> Result<Tokens, String> {
    let error = scanner.get_errors();
    let tokens: Tokens = scanner.scan_tokens();
    if error.had_error {
        process::exit(1);
    }
    Ok(tokens)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn web_run(prompt: String) -> String {
    log("In web_run()");
    match run(prompt) {
        Ok(tokens) => format!("{}", tokens),
        Err(msg) => msg,
    }
}

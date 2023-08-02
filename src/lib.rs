use crate::{lox::Lox, scanner::Scanner, token::Token};

use std::{
    fs,
    io::{self, Write},
    process,
};
use wasm_bindgen::prelude::*;

pub mod lox;
pub mod scanner;
pub mod token;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn run_file(file: &String) {
    let contents: String = fs::read_to_string(file).unwrap();
    let _ = run(contents);
}

pub fn run_prompt() {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        if buffer == "exit\n" {
            break;
        }
        let _ = run(buffer);
    }
}

fn run(code: String) -> Result<(), String> {
    let mut scanner = Scanner::new(code, Lox::new());
    let error = scanner.get_errors();
    let tokens: &Vec<Token> = scanner.scan_tokens();
    if error.had_error {
        process::exit(1);
    }
    for t in tokens {
        println!("{}", t);
    }
    Ok(())
}

#[wasm_bindgen]
pub fn web_run() {
    log("In web_run()");
    run_prompt();
}

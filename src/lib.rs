extern crate cfg_if;
extern crate wasm_bindgen;
use crate::{lox::Lox, scanner::Scanner, token::Token};

use cfg_if::cfg_if;
use std::{
    fs,
    io::{self, Write},
    process,
};
use wasm_bindgen::prelude::*;

pub mod lox;
pub mod scanner;
pub mod token;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn run_file(file: &String) {
    let contents: String = fs::read_to_string(file).unwrap();
    let _ = run(contents);
}

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
            for t in tokens {
                println!("{}", t);
            }
        } else {
            println!("Error");
            process::exit(69);
        }
    }
}

fn run(code: String) -> Result<Vec<Token>, String> {
    let mut scanner = Scanner::new(code, Lox::new());
    let error = scanner.get_errors();
    let tokens: &Vec<Token> = scanner.scan_tokens();
    if error.had_error {
        process::exit(1);
    }
    Ok(tokens.to_vec())
}

#[wasm_bindgen]
pub fn web_run(prompt: String) -> String {
    log("In web_run()");
    match run(prompt) {
        Ok(tokens) => {
            let mut result = String::new();
            for token in tokens {
                result.push_str(format!("{}", token).as_str());
            }
            result
        }
        Err(msg) => msg,
    }
}
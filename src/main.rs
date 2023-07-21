use std::{env, fs};

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

fn run_file(file: &String) {}

fn run_prompt() {}

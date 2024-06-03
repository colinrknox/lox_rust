use clox;
use jlox;
use std::io::{self, Write};
pub(crate) use std::{fs, process};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: lox [script]");
    } else if args.len() == 2 {
        jlox::run_file(&args[1]);
    } else {
        jlox::stdin_interactive();
    }
}

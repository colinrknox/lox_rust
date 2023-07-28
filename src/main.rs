use lox_rust::{lox::Lox, scanner::Scanner, token::Token};
use std::{
    env, fs,
    io::{self, Write},
    process,
};

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
    let _ = run(contents);
}

fn run_prompt() {
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

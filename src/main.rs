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
            print!("Error: invalid expression");
        };
    }
}

fn run(code: String) {}

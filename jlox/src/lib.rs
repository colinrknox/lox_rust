use std::fs;
use std::io;
use std::io::Write;

use parser::RecursiveParser;
use parser::Visitor;
use scanner::Scanner;

pub mod parser;
pub mod scanner;

pub fn stdin_interactive() {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        if buffer == "exit\n" {
            break;
        }
        run(buffer.clone());
    }
}

pub fn run_file(file_path: &String) {
    let source = fs::read_to_string(file_path).expect("Unable to read file {file_path}");
    run(source);
}

fn run(code: String) {
    let mut scanner = Scanner::new(&code);
    let tokens = scanner.scan_tokens();

    match tokens {
        Ok(tokens) => {
            let mut parser = RecursiveParser::new(tokens.clone());
            for token in tokens {
                println!("{token}");
            }
            match parser.expression() {
                Ok(expr) => println!("{}", expr.visit_print()),
                Err(_) => println!("Parsing error"),
            }
        }
        Err(errors) => {
            for e in errors {
                error(e.token.line, format!("{} {}\n", e.message, e.token.lexeme));
            }
        }
    };
}

fn error(line: usize, message: String) {
    report(line, "".to_string(), message)
}

fn report(line: usize, r#where: String, message: String) {
    eprintln!("[line {line}] Error{where}: {message}");
}

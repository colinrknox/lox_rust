use std::fs;
use std::io;
use std::io::Write;

use scanner::Scanner;

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
            for token in tokens {
                println!("{token}");
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

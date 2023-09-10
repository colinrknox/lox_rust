use lox_rust::{run_file, stdin_interactive};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: lox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        stdin_interactive();
    }
}

use lox_rust::{
    ast::{Expr, Object},
    run_file, stdin_interactive,
    token::TokenBuilder,
};
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

    // let expr: Expr = Expr::Binary(
    //     Box::new(Expr::Unary(
    //         TokenBuilder::new().lexeme("-".to_string()).build(),
    //         Box::new(Expr::Literal(Object::Number(123.0))),
    //     )),
    //     TokenBuilder::new().lexeme("*".to_string()).build(),
    //     Box::new(Expr::Grouping(Box::new(Expr::Literal(Object::Number(
    //         45.67,
    //     ))))),
    // );
    // println!("{}", expr);
}

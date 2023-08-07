use core::fmt::Display;

use super::token::Token;

// macro_rules! ast {
//     ( $name:ident: $( $data_type:ident $field:ident ),* ) => {
//         pub struct $name {
//             $(
//                 /*
//                  * Not sure if having someone pass in a proper type is the correct
//                  * solution or if be cleverly wrapped by the macro when the
//                  * type size is unknown or whatever
//                  */
//                 // pub $field: ast!(@ast_field $name, $data_type),
//                 pub $field: $data_type,
//             )*
//         }
//
//         impl $name {
//             pub fn new($( $field: $data_type ),*) -> $name {
//                 $name {
//                     $( $field: $field ),*
//                 }
//             }
//         }
//     };
//     (@ast_field $name: ident, $data_type: ident ) => {
//         Box<dyn $data_type>
//     };
// }
// pub type Expr = Box<dyn Expression>;

// pub trait Expression {
//     fn accept(&self);
// }

// ast!(Binary: Expr left, Token operator, Expr right);
// ast!(Grouping: Expr expression);
// ast!(Literal: Object value);
// ast!(Unary: Token operator, Expr right);

pub enum Object {
    Number(f64),
    String(String),
    Nil,
}

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Object),
    Unary(Token, Box<Expr>),
}

pub fn visit(expr: Expr) -> String {
    format!("{}", expr)
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Nil => write!(f, "{}", "nil"),
            Object::Number(num) => write!(f, "{}", num),
            Object::String(string) => write!(f, "{}", string),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(lhs, op, rhs) => {
                write!(f, "({} {} {})", op.get_lexeme(), lhs, rhs)
            }
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Literal(obj) => write!(f, "{}", obj),
            Expr::Unary(op, rhs) => write!(f, "({} {})", op.get_lexeme(), rhs),
        }
    }
}

use core::fmt::Display;

use super::token::{Token, TokenType};

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone)]
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
            Object::Boolean(bool) => write!(f, "{}", bool),
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

pub fn eval(expr: Expr) -> Result<Object, Expr> {
    match expr {
        Expr::Unary(sign, e) => eval_unary(sign.clone(), *e),
        Expr::Literal(obj) => Ok(obj),
        Expr::Binary(lhs, op, rhs) => eval_binary(*lhs, op.clone(), *rhs),
        Expr::Grouping(e) => eval_grouping(*e),
    }
}

fn eval_grouping(expr: Expr) -> Result<Object, Expr> {
    eval(expr)
}

fn eval_binary(lhs: Expr, op: Token, rhs: Expr) -> Result<Object, Expr> {
    let lhs_res = eval(lhs.clone())?;
    let rhs_res = eval(rhs.clone())?;

    if TokenType::BangEqual == op.token_type {
        return Ok(Object::Boolean(lhs_res != rhs_res));
    } else if TokenType::EqualEqual == op.token_type {
        return Ok(Object::Boolean(lhs_res == rhs_res));
    }

    if let (Object::Number(lhs_res), Object::Number(rhs_res)) = (lhs_res.clone(), rhs_res.clone()) {
        return match op.token_type {
            TokenType::Minus => Ok(Object::Number(lhs_res - rhs_res)),
            TokenType::Slash => Ok(Object::Number(lhs_res / rhs_res)),
            TokenType::Star => Ok(Object::Number(lhs_res * rhs_res)),
            TokenType::Plus => Ok(Object::Number(lhs_res + rhs_res)),
            TokenType::Greater => Ok(Object::Boolean(lhs_res > rhs_res)),
            TokenType::GreaterEqual => Ok(Object::Boolean(lhs_res >= rhs_res)),
            TokenType::EqualEqual => Ok(Object::Boolean(lhs_res == rhs_res)),
            TokenType::LessEqual => Ok(Object::Boolean(lhs_res <= rhs_res)),
            TokenType::Less => Ok(Object::Boolean(lhs_res <= rhs_res)),
            _ => Err(Expr::Binary(Box::new(lhs), op, Box::new(rhs))),
        };
    } else if let (Object::String(lhs_res), Object::String(rhs_res)) = (lhs_res, rhs_res) {
        return match op.token_type {
            TokenType::Plus => Ok(Object::String(format!("{}{}", lhs_res, rhs_res))),
            _ => Err(Expr::Binary(Box::new(lhs), op, Box::new(rhs))),
        };
    } else {
        return Err(Expr::Binary(Box::new(lhs), op, Box::new(rhs)));
    }
}

fn eval_unary(token: Token, expr: Expr) -> Result<Object, Expr> {
    let res = eval(expr.clone())?;
    match token.token_type {
        TokenType::Minus if matches!(res, Object::Number(_)) => {
            if let Object::Number(n) = res {
                Ok(Object::Number(-1.0 * n))
            } else {
                Err(expr)
            }
        }
        TokenType::Bang => Ok(Object::Boolean(!is_truthy(res))),
        _ => Err(expr),
    }
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Nil => false,
        Object::Boolean(b) => b,
        _ => true,
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
}

pub fn eval_stmt(stmt: Stmt) -> Result<Object, Expr> {
    let result = eval(match stmt.clone() {
        Stmt::Print(e) | Stmt::Expression(e) => *e,
    })?;
    if let Stmt::Print(_) = stmt {
        println!("{}", result);
    }
    Ok(result)
}

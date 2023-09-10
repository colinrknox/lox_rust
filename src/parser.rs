use crate::ast::{Expr, Object, Stmt};
use crate::token::{TokenType, Tokens};

use super::token::Token;

type Result<T> = std::result::Result<T, ParseError>;

struct ParseError {
    pub token_type: TokenType,
    pub message: String,
}

impl ParseError {
    pub fn new(token_type: TokenType, message: String) -> ParseError {
        ParseError {
            token_type,
            message,
        }
    }
}

pub struct Parser {
    tokens: Tokens,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Tokens) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.at_end() {
            stmts.push(self.statement());
        }
        return stmts;
    }

    fn statement(&mut self) -> Stmt {
        if self.compare(vec![TokenType::Print]) {
            return self.print_stmt();
        }
        return self.expression_stmt();
    }

    fn print_stmt(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
        return Stmt::Print(Box::new(value));
    }

    fn expression_stmt(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
        return Stmt::Expression(Box::new(value));
    }

    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.compare(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();
        while self.compare(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();
        while self.compare(vec![TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();
        while self.compare(vec![TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.compare(vec![TokenType::Bang, TokenType::Minus]) {
            let op: Token = self.previous();
            let right: Expr = self.unary();
            return Expr::Unary(op, Box::new(right));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.compare(vec![TokenType::False]) {
            return Expr::Literal(Object::Boolean(false));
        }
        if self.compare(vec![TokenType::True]) {
            return Expr::Literal(Object::Boolean(true));
        }
        if self.compare(vec![TokenType::Nil]) {
            return Expr::Literal(Object::Nil);
        }

        if self.compare(vec![TokenType::String("".to_string())]) {
            return Expr::Literal(Object::String(self.previous().lexeme));
        }

        if self.compare(vec![TokenType::Number(0.0)]) {
            return Expr::Literal(Object::Number(
                self.previous().lexeme.parse::<f64>().unwrap(),
            ));
        }

        let expr: Expr = self.expression();
        let _ = self.consume(
            TokenType::RightParen,
            "Expect ')' after expression.".to_string(),
        );
        return Expr::Grouping(Box::new(expr));
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token> {
        if self.check(token_type.clone()) {
            return Ok(self.advance());
        }
        return Err(ParseError::new(token_type, message));
    }

    fn compare(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, t: TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        match t {
            TokenType::Number(_) if matches!(self.peek().token_type, TokenType::Number(_)) => {
                return true
            }
            TokenType::Number(_) => return false,
            TokenType::String(_) if matches!(self.peek().token_type, TokenType::String(_)) => {
                return true
            }
            TokenType::String(_) => return false,
            token_type => return self.peek().token_type == token_type,
        }
    }

    fn at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }
}

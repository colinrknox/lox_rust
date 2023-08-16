use crate::ast::{Expr, Object};
use crate::token::TokenType;

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
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        let expr = self.comparison();

        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let expr: Expr = self.term();
        return expr;
    }

    fn term(&mut self) -> Expr {
        let expr: Expr = self.factor();
        return expr;
    }

    fn factor(&mut self) -> Expr {
        let expr: Expr = self.unary();
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
            return Expr::Literal(Object::Boolean(false));
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
        self.consume(
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
        return self.peek().token_type == t;
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

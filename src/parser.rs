use crate::token::TokenType;

use super::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn expression(&self) -> Expr {
        return self.equality();
    }

    fn equality(&self) -> Expr {
        let expr = self.comparison();

        return expr;
    }

    fn comparison(&self) -> Expr {}

    fn check(&self, t: TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        return self.peek().token_type == t;
    }

    fn at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1];
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current];
    }
}

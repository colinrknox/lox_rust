use crate::scanner::{Token, TokenType};

pub trait Visitor {
    fn visit_print(&self) -> String;
    fn visit_eval(&self) -> Object;
}

pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Nil => write!(f, "{}", "nil"),
            Object::Number(num) => write!(f, "{}", num),
            Object::String(string) => write!(f, "{}", string),
            Object::Boolean(bool) => write!(f, "{}", bool),
        }
    }
}

pub enum Expr {
    Literal(Object),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(lhs, op, rhs) => {
                write!(f, "({} {} {})", op.lexeme, lhs, rhs)
            }
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Literal(obj) => write!(f, "{}", obj),
            Expr::Unary(op, rhs) => write!(f, "({} {})", op.lexeme, rhs),
        }
    }
}

impl Visitor for Expr {
    fn visit_print(&self) -> String {
        format!("{}", self)
    }
    fn visit_eval(&self) -> Object {
        todo!()
    }
}

pub struct RecursiveParser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParseError>,
}

#[derive(Clone)]
pub struct ParseError;

type Result = std::result::Result<Expr, ParseError>;

impl RecursiveParser {
    pub fn new(tokens: Vec<Token>) -> RecursiveParser {
        RecursiveParser {
            tokens,
            current: 0,
            errors: vec![],
        }
    }

    pub fn expression(&mut self) -> Result {
        match self.equality() {
            Ok(expr) => Ok(expr),
            Err(error) => {
                self.errors.push(error.clone());
                // continue through tokens until next statement
                self.synchronize();
                Err(error)
            }
        }
    }

    fn synchronize(&mut self) {
        while let Some(t) = self.advance() {
            if t.token_type == TokenType::Semicolon {
                break;
            }
            if let Some(t) = self.peek() {
                match t.token_type {
                    TokenType::For
                    | TokenType::Class
                    | TokenType::Fn
                    | TokenType::Var
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return => break,
                    _ => (),
                }
            }
        }
    }

    fn equality(&mut self) -> Result {
        let tokens = vec![TokenType::BangEqual, TokenType::EqualEqual];
        let mut expr = self.comparison();

        loop {
            match self.match_types(&tokens) {
                Some(t) => {
                    let operator = t.clone();
                    let right = self.comparison();
                    expr = Ok(Expr::Binary(Box::new(expr?), operator, Box::new(right?)));
                }
                None => break,
            }
        }

        expr
    }

    fn comparison(&mut self) -> Result {
        let tokens = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        let mut expr = self.term();
        loop {
            match self.match_types(&tokens) {
                Some(t) => {
                    let operator = t.clone();
                    let right = self.term();
                    expr = Ok(Expr::Binary(Box::new(expr?), operator, Box::new(right?)));
                }
                None => break,
            }
        }

        expr
    }

    fn term(&mut self) -> Result {
        let tokens = vec![TokenType::Plus, TokenType::Minus];
        let mut expr = self.factor();

        loop {
            match self.match_types(&tokens) {
                Some(t) => {
                    let operator = t.clone();
                    let right = self.factor();
                    expr = Ok(Expr::Binary(Box::new(expr?), operator, Box::new(right?)));
                }
                None => break,
            }
        }

        expr
    }

    fn factor(&mut self) -> Result {
        let tokens = vec![TokenType::Slash, TokenType::Star];
        let mut expr = self.unary();

        loop {
            match self.match_types(&tokens) {
                Some(t) => {
                    let operator = t.clone();
                    let right = self.unary();
                    expr = Ok(Expr::Binary(Box::new(expr?), operator, Box::new(right?)))
                }
                None => break,
            }
        }

        expr
    }

    fn unary(&mut self) -> Result {
        let tokens = vec![TokenType::Bang, TokenType::Minus];
        match self.match_types(&tokens) {
            Some(t) => {
                let operator = t.clone();
                let right = self.unary();
                return Ok(Expr::Unary(operator, Box::new(right?)));
            }
            None => (),
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result {
        if let Some(_) = self.match_type(TokenType::False) {
            return Ok(Expr::Literal(Object::Boolean(false)));
        }
        if let Some(_) = self.match_type(TokenType::True) {
            return Ok(Expr::Literal(Object::Boolean(true)));
        }
        if let Some(_) = self.match_type(TokenType::Nil) {
            return Ok(Expr::Literal(Object::Nil));
        }
        if let Some(t) = self.match_type(TokenType::Number(0.0)) {
            if let TokenType::Number(num) = t.token_type {
                return Ok(Expr::Literal(Object::Number(num)));
            }
        }
        if let Some(t) = self.match_type(TokenType::String("".to_string())) {
            if let TokenType::String(string) = &t.token_type {
                return Ok(Expr::Literal(Object::String(string.clone())));
            }
        }
        if let Some(_) = self.match_type(TokenType::LeftParen) {
            let expr = self.expression();
            match self.match_type(TokenType::RightParen) {
                Some(_) => return Ok(Expr::Grouping(Box::new(expr?))),
                None => return Err(ParseError),
            }
        }

        Err(ParseError)
    }

    fn match_type(&mut self, t: TokenType) -> Option<&Token> {
        let tokens = vec![t];
        self.match_types(&tokens)
    }

    fn match_types(&mut self, types: &Vec<TokenType>) -> Option<&Token> {
        for t in types {
            if self.check(&t) {
                return self.advance();
            }
        }
        None
    }

    fn check(&self, t: &TokenType) -> bool {
        match self.peek() {
            Some(token) => match (&token.token_type, t) {
                (TokenType::String(_), TokenType::String(_)) => true,
                (TokenType::Number(_), TokenType::Number(_)) => true,
                (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
                (x, y) => *x == *y,
            },
            None => false,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn advance(&mut self) -> Option<&Token> {
        self.current += 1;
        self.previous()
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::{Token, TokenType};

    use super::RecursiveParser;

    fn create_parser() -> RecursiveParser {
        let mut tokens = vec![];
        tokens.push(Token::new(
            TokenType::Identifier("word".to_string()),
            "word".to_string(),
            1,
        ));
        RecursiveParser::new(tokens)
    }

    #[test]
    fn test_check_ignoring_tuple_variant() {
        let parser = create_parser();
        let actual = parser.check(&TokenType::Identifier("".to_string()));

        assert!(actual);
    }

    #[test]
    fn test_check_no_match() {
        let parser = create_parser();
        let actual = parser.check(&TokenType::String("".to_string()));

        assert!(!actual);
    }
}

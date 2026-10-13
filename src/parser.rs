use std::iter::Peekable;
use std::vec::IntoIter;
use crate::expression::{Expr, Literal};
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum ParserError {
    Message(usize, String)
}

struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    current: usize,
}

type Result<T> = std::result::Result<T, ParserError>;
type ExprResult = Result<Expr>;

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            current: 0
        }
    }

    fn expression(&mut self) -> ExprResult {
        self.equality()
    }

    fn equality(&mut self) -> ExprResult {
        let mut expr = self.comparison()?;

        while let Some(operator) = self.match_advance(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let right = Box::new(self.comparison()?);
            expr = Expr::Binary { left: Box::new(expr), operator, right };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> ExprResult {
        let mut expr = self.term()?;
        while let Some(operator) =  self.match_advance(&[
            TokenType::GREATER, TokenType::GREATER_EQUAL,
            TokenType::LESS, TokenType::LESS_EQUAL,
        ]) {
            let right = Box::new(self.term()?);
            expr = Expr::Binary { left: Box::new(expr), operator, right };
        }
        Ok(expr)
    }

    fn term(&mut self) -> ExprResult {
        let mut expr = self.factor()?;
        while let Some(operator) = self.match_advance(&[TokenType::MINUS, TokenType::PLUS]) {
            let right = Box::new(self.factor()?);
            expr = Expr::Binary { left: Box::new(expr), operator, right };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> ExprResult {
        let mut expr = self.unary()?;
        while let Some(operator) = self.match_advance(&[TokenType::SLASH, TokenType::STAR]) {
            let right = Box::new(self.unary()?);
            expr = Expr::Binary { left: Box::new(expr), operator, right };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ExprResult {
        if let Some(operator) = self.match_advance(&[TokenType::BANG, TokenType::MINUS]) {
            let right = Box::new(self.unary()?);
            Ok(Expr::Unary { operator, right })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> ExprResult {
        let token = { self.peek().clone() };
        let result = match token.token_type {
            TokenType::FALSE => { Some(Expr::Literal { value: Literal::False }) },
            TokenType::TRUE => { Some(Expr::Literal { value: Literal::True }) },
            TokenType::STRING(s) => { Some(Expr::Literal { value: Literal::String(s) }) },
            TokenType::NUMBER(n) => { Some(Expr::Literal { value: Literal::Numeric(n) }) },
            TokenType::LEFT_PAREN => {
                let expression = Box::new(self.expression()?);
                self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
                Some(Expr::Grouping { expression })
            }
            _ => None
        };
        if let Some(_) = result {
            self.advance();
        }
        Ok(result.unwrap())
    }

    fn match_advance(&mut self, types: &[TokenType]) -> Option<Token> {
        if types.contains(&self.peek().token_type) {
            Some(self.advance())
        } else {
            None
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    // fn check(&mut self, token_types: &[TokenType]) -> bool {
    //     if self.is_at_end() { return false; }
    //     token_types.contains(&self.peek().token_type)
    // }

    fn advance(&mut self) -> Token {
        self.tokens.next().expect("EOF guarantee")
    }

    fn peek(&mut self) -> &Token {
        self.tokens.peek().expect("EOF guarantee")
    }

    // fn previous(&self) -> &Token {
    //     &self.tokens[self.current - 1]
    // }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token> {
        let token = self.peek();
        if token.token_type == token_type {
            Ok(self.advance())
        } else {
            Err(ParserError::Message(token.line, message.into()))
        }
    }
}
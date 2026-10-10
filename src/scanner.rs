use crate::token;
use crate::token::{Token, TokenType};

pub type ScannerResult<T> = Result<T, ScannerError>;

#[derive(Debug)]
pub enum ScannerError {
    UnexpectedCharacterError(usize, usize, char),
    ExpectedCharacterError(usize, usize, char),
}

pub struct Scanner<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token<'a>>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub line_start: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            line_start: 0,
        }
    }

    pub fn scan_tokens(&'a mut self) -> ScannerResult<&'a Vec<Token<'a>>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenType::EOF, "".into(), self.line));
        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> ScannerResult<()> {
        let c = self.advance();

        match c {
            '(' => Ok(self.add_token(TokenType::LEFT_PAREN)),
            ')' => Ok(self.add_token(TokenType::RIGHT_PAREN)),
            '{' => Ok(self.add_token(TokenType::LEFT_BRACE)),
            '}' => Ok(self.add_token(TokenType::RIGHT_BRACE)),
            ',' => Ok(self.add_token(TokenType::COMMA)),
            '.' => Ok(self.add_token(TokenType::DOT)),
            '-' => Ok(self.add_token(TokenType::MINUS)),
            '+' => Ok(self.add_token(TokenType::PLUS)),
            ';' => Ok(self.add_token(TokenType::SEMICOLON)),
            '*' => Ok(self.add_token(TokenType::STAR)),
            '!' if self.matches('=') => Ok(self.add_token(TokenType::BANG_EQUAL)),
            '!' => Ok(self.add_token(TokenType::BANG)),
            '=' if self.matches('=') => Ok(self.add_token(TokenType::EQUAL_EQUAL)),
            '=' => Ok(self.add_token(TokenType::EQUAL)),
            '<' if self.matches('=') => Ok(self.add_token(TokenType::LESS_EQUAL)),
            '<' => Ok(self.add_token(TokenType::LESS)),
            '>' if self.matches('=') => Ok(self.add_token(TokenType::GREATER_EQUAL)),
            '>' => Ok(self.add_token(TokenType::GREATER)),
            '/' if self.matches('/') =>  {
                while self.peek() != '\n' && !self.is_at_end() { let _ = self.advance(); }
                Ok(())
            },
            '/' => Ok(self.add_token(TokenType::SLASH)),
            ' ' | '\r' | '\t' => { Ok(()) }
            '\n' => { self.line += 1; self.line_start = self.start; Ok(()) }
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' => self.identifier(),
            _ => Err(ScannerError::UnexpectedCharacterError(self.line, 1, c))
        }
    }

    fn advance(&mut self) -> char {
        let result = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        result
    }

    fn add_token(&mut self, token_type: TokenType<'a>) {
        let value = Self::chars(self.source, self.start, self.current);
        self.tokens.push(Token::new(token_type, value, self.line));
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        if self.char_at(self.current).unwrap() != expected { return false }

        self.current += 1;
        true
    }

    fn char_at(&self, at: usize) -> Option<char> {
        self.source.char_indices().nth(at).map(|c| c.1)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { self.char_at(self.current).unwrap() }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0'; }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn number(&mut self) -> ScannerResult<()> {
        while self.peek().is_digit(10) { self.advance(); }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) { self.advance(); }
        }
        let value = Self::chars(self.source, self.start, self.current).parse()
            .map_err(|e| ScannerError::UnexpectedCharacterError(self.line, self.start, '.'))?;
        self.add_token( TokenType::NUMBER(value) );
        Ok(())
    }

    fn string(&mut self) -> ScannerResult<()> {
        let err_start = self.start;
        let err_line = self.line;
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScannerError::ExpectedCharacterError(err_line, err_start, '"'));
        }

        self.advance();

        let value = Self::chars(self.source, self.start, self.current);
        self.add_token(TokenType::STRING(value));
        Ok(())
    }

    fn identifier(&mut self) -> ScannerResult<()> {
        while self.peek().is_alphanumeric() { self.advance(); }
        let value = Self::chars(self.source, self.start, self.current);
        if let Some(v) = token::KEYWORDS.get(value) {
            self.add_token(v.clone());
        } else  {
            self.add_token(TokenType::IDENTIFIER(value));
        }
        Ok(())
    }

    fn chars(source: &str, start: usize, end: usize) -> &str {
        let mut indices = source.char_indices().map(|(i, _)| i);
        let start_byte = indices.nth(start);
        let end_byte = if start == end { start_byte } else { indices.nth(end - start - 1) };

        match (start_byte, end_byte) {
            (Some(b_start), Some(b_end)) => &source[b_start..b_end],
            (Some(b_start), None) => &source[b_start..],
            (None, _) => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should() {
        let code = r#"fun HelloWorld() {
          print("Hello");
        }"#;
        let mut s = Scanner::new(code);
        let tokens = s.scan_tokens().expect("Test failed");
        assert_eq!(*tokens, vec![
            Token::new(TokenType::FUN, "fun", 1),
            Token::new(TokenType::IDENTIFIER("HelloWorld"), "HelloWorld", 1),
            Token::new(TokenType::LEFT_PAREN, "(", 1),
            Token::new(TokenType::RIGHT_PAREN, ")", 1),
            Token::new(TokenType::LEFT_BRACE, "{", 1),
            Token::new(TokenType::PRINT, "print", 2),
            Token::new(TokenType::LEFT_PAREN, "(", 2),
            Token::new(TokenType::STRING("\"Hello\""), "\"Hello\"", 2),
            Token::new(TokenType::RIGHT_PAREN, ")", 2),
            Token::new(TokenType::SEMICOLON, ";", 2),
            Token::new(TokenType::RIGHT_BRACE, "}", 3),
            Token::new(TokenType::EOF, "", 3),
        ]);
    }

    #[test]
    fn it_should_scan_string() {
        let code = r#" !="hello" "#;
        let mut s = Scanner::new(code);
        let tokens = s.scan_tokens().expect("Test failed");
        assert_eq!(*tokens, vec![
            Token::new(TokenType::BANG_EQUAL, "!=".into(), 1),
            Token::new(TokenType::STRING("\"hello\""), "\"hello\"".into(), 1),
            Token::new(TokenType::EOF, "", 1),
        ]);
    }

    #[test]
    fn it_should_scan_number() {
        let code = r#" !=3.1 "#;
        let mut s = Scanner::new(code);
        let tokens = s.scan_tokens().expect("Test failed");
        assert_eq!(*tokens, vec![
            Token::new(TokenType::BANG_EQUAL, "!=".into(), 1),
            Token::new(TokenType::NUMBER(3.1), "3.1", 1),
            Token::new(TokenType::EOF, "", 1),
        ]);
    }

}

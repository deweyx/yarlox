use std::fmt::Display;
use phf::phf_map;
use yarlox_macro::{generate_ast};

pub(crate) static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::AND,
    "class" => TokenType::CLASS,
    "else" => TokenType::ELSE,
    "false" => TokenType::FALSE,
    "fun" => TokenType::FUN,
    "for" => TokenType::FOR,
    "if" => TokenType::IF,
    "nil" => TokenType::NIL,
    "or" => TokenType::OR,
    "print" => TokenType::PRINT,
    "return" => TokenType::RETURN,
    "super" => TokenType::SUPER,
    "this" => TokenType::THIS,
    "true" => TokenType::TRUE,
    "var" => TokenType::VAR,
    "while" => TokenType::WHILE,
};

#[allow(non_camel_case_types, unused)]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType<'a> {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER(&'a str),
    STRING(&'a str),
    NUMBER(f32),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl<'a> Display for TokenType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}",)
    }
}

pub trait Literal: Display {}
impl Literal for &str {}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub lexeme: &'a str,
    pub line: usize,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {}",
            self.token_type,
            self.lexeme,
        ))
    }
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType<'a>,
        lexeme: &'a str,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

#[generate_ast(
    Binary with left: Box<Self>, operator: Token<'a>, right: Box<Self>;
    Grouping with expression: Box<Self>;
)]
trait Expr {}
use yarlox_macro::generate_ast;
use crate::token::Token;

pub type BoxExpr = Box<Expr>;

#[generate_ast(
    Binary with left: BoxExpr, operator: Token, right: BoxExpr;
    Grouping with expression: BoxExpr;
    Unary with operator: Token, right: BoxExpr;
    Literal with value: Literal;
)]
pub enum Expr {}

pub enum Literal {
    True,
    False,
    Nil,
    Numeric(f32),
    String(String),
}
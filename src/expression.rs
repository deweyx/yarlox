use yarlox_macro::generate_ast;
use crate::token::Token;

pub type BoxExpr = Box<Expr>;

#[generate_ast(
    Binary with left: BoxExpr, operator: Token, right: BoxExpr;
    Grouping with expression: BoxExpr;
    Unary with operator: Token, right: BoxExpr;
    Literal with value: Literal;
)]
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    True,
    False,
    Nil,
    Numeric(f32),
    String(String),
}

impl Expr {
    pub fn accept<R, V: Visitor<R>>(&self, visitor: &V) -> R {
        visitor.visit(self)
    }
    
    pub fn accept_mut<R, V: VisitorMut<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_mut(self)
    }
}

pub trait Visitor<R> {
    fn visit(&self, expr: &Expr) -> R;
}

pub trait VisitorMut<R> {
    fn visit_mut(&mut self, expr: &Expr) -> R;
}
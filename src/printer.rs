use crate::expression::{Expr, Visitor};

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => self.parenthesize(&operator.lexeme, &[left.clone(), right.clone()]),
            Expr::Grouping { expression } => self.parenthesize("group", &[expression.clone()]),
            Expr::Literal { value } => format!("{value:?}"),
            Expr::Unary { operator, right } => self.parenthesize(&operator.lexeme, &[right.clone()]),
        }
    }
}

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, mut exprs: &[Box<Expr>]) -> String {
        let mut result = "".to_owned();
        result.push('(');
        result.push_str(name);
        exprs.iter().for_each(|e| {
            result.push(' ');
            result.push_str(&e.accept(self));
        });
        result.push(')');
        result
    }
}
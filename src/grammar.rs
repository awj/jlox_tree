use crate::token;

pub trait Visitor<'a, X: 'a> {
    fn visit(&mut self, expr: &'a Expr) -> X;
}

pub struct AstPrinter {}

impl<'a> Visitor<'a, String> for AstPrinter {
    fn visit(&mut self, expr: &'a Expr) -> String {
        match expr {
            Expr::Binary{left, operator, right} => {
                format!("({} {} {})", operator.lexeme, self.visit(left), self.visit(right))
            },
            Expr::Grouping{expression} => {
                format!("(group {})", self.visit(expression))
            },
            Expr::Literal{value} => {
                value.to_s()
            },
            Expr::Unary{operator, right} => {
                format!("({} {})", operator.lexeme, self.visit(right))
            }
        }
    }
}

pub enum Expr<'a> {
    Binary { left: &'a Expr<'a>, operator: token::Token, right: &'a Expr<'a> },
    Grouping { expression: &'a Expr<'a> },
    Literal { value: &'a token::Literal },
    Unary { operator: &'a token::Token, right: &'a Expr<'a> }
}

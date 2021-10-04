use crate::token;

pub trait Expr {}

pub struct Binary<'a> {
    left: &'a dyn Expr,
    operator: token::Token,
    right: &'a dyn Expr,
}

impl Expr for Binary<'_> {}

impl<'a> Binary<'a> {
    pub fn new(left: &'a dyn Expr, operator: token::Token, right: &'a dyn Expr) -> Binary<'a> {
        Binary {
            left,
            operator,
            right
        }
    }
}

pub struct Grouping<'a> {
    expression: &'a dyn Expr
}

impl<'a> Grouping<'a> {
    pub fn new(expression: &'a dyn Expr) -> Grouping<'a> {
        Grouping {
            expression
        }
    }
}

impl Expr for Grouping<'_> {}

pub struct Literal<'a> {
    value: &'a token::Literal
}

impl<'a> Literal<'a> {
    pub fn new(value: &'a token::Literal) -> Literal<'a> {
        Literal {
            value
        }
    }
}

impl Expr for Literal<'_> {}

pub struct Unary<'a> {
    operator: &'a token::Token,
    right: &'a dyn Expr
}

impl<'a> Unary<'a> {
    pub fn new(operator: &'a token::Token, right: &'a dyn Expr) -> Unary<'a> {
        Unary {
            operator,
            right
        }
    }
}

impl Expr for Unary<'_> {}

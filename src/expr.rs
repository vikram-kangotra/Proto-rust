use crate::token::{Token};

pub trait ExprVisitor {
    fn visit_literal(&mut self, literal: &Literal) -> f64;
    fn visit_unary(&mut self, unary: &Unary) -> f64;
    fn visit_binary(&mut self, binary: &Binary) -> f64;
}

pub trait Expr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> f64;
}

pub struct Literal {
    token: Token
}

impl Literal {

    pub fn new(token: Token) -> Self {
        Self { token }
    }

    pub fn get_token(&self) -> &Token { &self.token }
}

impl Expr for Literal {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> f64 {
        return visitor.visit_literal(self);
    }
}

pub struct Unary {
    op: Token,
    right: Box<dyn Expr>
}

impl Unary {

    pub fn new(op: Token, right: Box<dyn Expr>) -> Self {
        Self { op, right }
    }

    pub fn get_operator(&self) -> &Token { &self.op }
    pub fn get_right(&self) -> &Box<dyn Expr> { &self.right }
}

impl Expr for Unary {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> f64 {
        return visitor.visit_unary(self);
    }
}

pub struct Binary {
    op: Token,
    left: Box<dyn Expr>,
    right: Box<dyn Expr>
}

impl Binary {

    pub fn new(left: Box<dyn Expr>, op: Token, right: Box<dyn Expr>) -> Self {
        Self { op, left, right }
    }

    pub fn get_operator(&self) -> &Token { &self.op }
    pub fn get_left(&self) -> &Box<dyn Expr> { &self.left }
    pub fn get_right(&self) -> &Box<dyn Expr> { &self.right }
}

impl Expr for Binary {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> f64 {
        return visitor.visit_binary(self);
    }
}

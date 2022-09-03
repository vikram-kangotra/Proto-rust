use crate::token::Token;

pub trait ExprVisitor {
    fn visit_literal(&mut self, literal: &LiteralExpr) -> f64;
    fn visit_unary(&mut self, unary: &UnaryExpr) -> f64;
    fn visit_binary(&mut self, binary: &BinaryExpr) -> f64;
}

pub trait Expr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> f64;
}

pub struct LiteralExpr {
    token: Token,
}

impl LiteralExpr {
    pub fn new(token: Token) -> Self {
        Self {  token, }
    }
    pub fn get_token(&self) -> &Token { &self.token }
}

impl Expr for LiteralExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> f64 {
        return visitor.visit_literal(self);
    }
}

pub struct UnaryExpr {
    op: Token,
    right: Box<dyn Expr>,
}

impl UnaryExpr {
    pub fn new(op: Token, right: Box<dyn Expr>) -> Self {
        Self {  op,  right, }
    }
    pub fn get_op(&self) -> &Token { &self.op }
    pub fn get_right(&self) -> &Box<dyn Expr> { &self.right }
}

impl Expr for UnaryExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> f64 {
        return visitor.visit_unary(self);
    }
}

pub struct BinaryExpr {
    left: Box<dyn Expr>,
    op: Token,
    right: Box<dyn Expr>,
}

impl BinaryExpr {
    pub fn new(left: Box<dyn Expr>, op: Token, right: Box<dyn Expr>) -> Self {
        Self {  left,  op,  right, }
    }
    pub fn get_left(&self) -> &Box<dyn Expr> { &self.left }
    pub fn get_op(&self) -> &Token { &self.op }
    pub fn get_right(&self) -> &Box<dyn Expr> { &self.right }
}

impl Expr for BinaryExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> f64 {
        return visitor.visit_binary(self);
    }
}

pub trait StmtVisitor {
    fn visit_expression(&mut self, expression: &ExpressionStmt) -> f64;
    fn visit_print(&mut self, print: &PrintStmt) -> f64;
}

pub trait Stmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> f64;
}

pub struct ExpressionStmt {
    expression: Box<dyn Expr>,
}

impl ExpressionStmt {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self {  expression, }
    }
    pub fn get_expression(&self) -> &Box<dyn Expr> { &self.expression }
}

impl Stmt for ExpressionStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> f64 {
        return visitor.visit_expression(self);
    }
}

pub struct PrintStmt {
    expression: Box<dyn Expr>,
}

impl PrintStmt {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self {  expression, }
    }
    pub fn get_expression(&self) -> &Box<dyn Expr> { &self.expression }
}

impl Stmt for PrintStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> f64 {
        return visitor.visit_print(self);
    }
}


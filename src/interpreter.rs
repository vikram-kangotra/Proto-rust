use crate::expr::*;
use crate::token::*;

pub struct Interpreter {}

impl Interpreter {

    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, statements: &Vec<Box<dyn Stmt>>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    fn execute(&mut self, stmt: &Box<dyn Stmt>) {
        stmt.accept(self); 
    }

    fn evaluate(&mut self, expr: &Box<dyn Expr>) -> f64 {
        return expr.accept(self);
    }
}

impl StmtVisitor for Interpreter {

    fn visit_expression(&mut self, stmt: &ExpressionStmt) -> f64 {
        self.evaluate(stmt.get_expression());
        return 0.0;
    }

    fn visit_print(&mut self, stmt: &PrintStmt) -> f64 { 
        let value = self.evaluate(stmt.get_expression());
        print!("{}", value);
        return 0.0; 
    }
}

impl ExprVisitor for Interpreter {

    fn visit_literal(&mut self, literal: &LiteralExpr) -> f64 {
        return literal.get_token().literal.parse::<f64>().unwrap();
    }

    fn visit_unary(&mut self, unary: &UnaryExpr) -> f64 {
        let op = unary.get_op();
        let right = unary.get_right().accept(self);
        match op.token_type {
            TokenType::Plus => return right,
            TokenType::Minus => return -right,
            _ => panic!("Unexpected token: {:?}", op.token_type)
        }
    }

    fn visit_binary(&mut self, binary: &BinaryExpr) -> f64 {
        let left = binary.get_left().accept(self);
        let right = binary.get_right().accept(self);
        let op = binary.get_op();
        match op.token_type {
            TokenType::Plus => return left + right,
            TokenType::Minus => return left - right,
            TokenType::Star => return left * right,
            TokenType::Slash => return left / right,
            _ => panic!("Unexpected token: {:?}", op.token_type)
        }
    }
}

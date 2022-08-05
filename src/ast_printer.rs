use crate::expr::{ExprVisitor, Expr};
use crate::expr::{Literal, Unary, Binary};
use crate::token::TokenType;

pub struct AstPrinter {}

impl AstPrinter {
    pub fn eval(&mut self, expr: &Box<dyn Expr>) -> f64 {
        return expr.accept(self);
    }
}

impl ExprVisitor for AstPrinter {

    fn visit_literal(&mut self, literal: &Literal) -> f64 {
        return literal.get_token().literal.parse::<f64>().unwrap();
    }

    fn visit_unary(&mut self, unary: &Unary) -> f64 {
        let op = unary.get_op();
        let right_val = unary.get_right().accept(self);
        match op.token_type {
            TokenType::Plus => return right_val,
            TokenType::Minus => return -right_val,
            _ => panic!("unexpected token type")
        }
    }
  
    fn visit_binary(&mut self, binary: &Binary) -> f64 {
        let left_val = binary.get_left().accept(self);
        let right_val = binary.get_right().accept(self);
        let op = binary.get_op();
        match op.token_type {
            TokenType::Plus => return left_val + right_val,
            TokenType::Minus => return left_val - right_val,
            TokenType::Star => return left_val * right_val,
            TokenType::Slash => return left_val / right_val,
            _ => panic!("unexpected token type")
        }
    }
}

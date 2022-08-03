use crate::token::{Token, TokenType};
use crate::expr::{Expr, Literal, Unary, Binary};

// expr         ->    term
// term         ->    factor (('-' | '+') factor)*
// factor       ->    unary (('*' | '/') unary)*
// unary        ->    ('+' | '-') unary | primary
// primary      ->    num | '(' expr ')'

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Box<dyn Expr> {
        self.expr()
    }

    pub fn expr(&mut self) -> Box<dyn Expr> {
        self.term()
    }

    fn term(&mut self) -> Box<dyn Expr> {
        let mut left = self.factor();
        while self.match_token(TokenType::Plus) || self.match_token(TokenType::Minus) {
            let op = self.pull();
            let right = self.factor();
            left = Box::new(Binary::new(left, op, right));
        }
        return left;
    }

    fn factor(&mut self) -> Box<dyn Expr> {
        let mut left = self.unary();
        while self.match_token(TokenType::Star) || self.match_token(TokenType::Slash) {
            let op = self.pull();
            let right = self.unary();
            left = Box::new(Binary::new(left, op, right));
        }
        return left;
    }

    fn unary(&mut self) -> Box<dyn Expr> {
        if self.match_token(TokenType::Plus) || self.match_token(TokenType::Minus) {
            let op = self.pull();
            let right = self.primary();
            return Box::new(Unary::new(op, right));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Box<dyn Expr> {        
        if self.match_token(TokenType::NumberLiteral) {
            return Box::new(Literal::new(self.pull()));
        }
        if self.eat(TokenType::LeftParen) {
            let exp = self.expr();
            self.eat(TokenType::RightParen);
            return exp;
        }
        // TODO: throw error here
        return Box::new(Literal::new(Token::new(TokenType::Unknown, "".to_string())));
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn pull(&mut self) -> Token {
        self.tokens.remove(self.current)
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn eat(&mut self, token_type: TokenType) -> bool {
        if self.match_token(token_type) {
            self.pull();
            return true;
        }
        return false;
    }
    
    fn match_token(&self, token_type: TokenType) -> bool {
        return !self.is_at_end() && self.peek().token_type == token_type;
    }
}

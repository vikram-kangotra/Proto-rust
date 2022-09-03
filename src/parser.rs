use crate::token::{Token, TokenType};
use crate::expr::{Expr, LiteralExpr, UnaryExpr, BinaryExpr};
use crate::expr::{Stmt, ExpressionStmt, PrintStmt};

// program      ->  statement* EOF
// statement    ->  expr_stmt | print_stmt
// expr_stmt     ->  expr ";"
// print_stmt    ->  "print" expr ";"
// expr         ->  equality
// equality     ->  comparison (("!=" | "==") comparison)*
// comparison   ->  term ((">" | ">=" | "<" | "<=") term)*
// term         ->  factor (('-' | '+') factor)*
// factor       ->  unary (('*' | '/') unary)*
// unary        ->  ('+' | '-') unary | primary
// primary      ->  num | '(' expr ')'

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

    pub fn parse(&mut self) -> Vec<Box<dyn Stmt>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement());
        }
        return statements;
    }

    fn statement(&mut self) -> Box<dyn Stmt> {
        if self.eat(&TokenType::Print) {
            return self.print_stmt();
        }
        return self.expr_stmt();
    }

    fn expr_stmt(&mut self) -> Box<dyn Stmt> {
        let expr = self.expr();
        self.consume(&TokenType::Semicolon);
        return Box::new(ExpressionStmt::new(expr));
    }

    fn print_stmt(&mut self) -> Box<dyn Stmt> {
        let expr = self.expr();
        self.consume(&TokenType::Semicolon);
        return Box::new(PrintStmt::new(expr));
    }

    fn expr(&mut self) -> Box<dyn Expr> {
        return self.equality();
    }

    fn equality(&mut self) -> Box<dyn Expr> {
        let mut left = self.comparison();
        while self.match_token(&TokenType::BangEqual) ||
            self.match_token(&TokenType::EqualEqual) {
            let op = self.pull();
            let right = self.comparison();
            left = Box::new(BinaryExpr::new(left, op, right));
        }
        return left;
    }

    fn comparison(&mut self) -> Box<dyn Expr> {
        let mut left = self.term();
        while self.match_token(&TokenType::Less) ||
            self.match_token(&TokenType::LessEqual) ||
            self.match_token(&TokenType::Greater) ||
            self.match_token(&TokenType::GreaterEqual) {
            let op = self.pull();
            let right = self.term();
            left = Box::new(BinaryExpr::new(left, op, right));
        }
        return left;
    }

    fn term(&mut self) -> Box<dyn Expr> {
        let mut left = self.factor();
        while self.match_token(&TokenType::Plus) || self.match_token(&TokenType::Minus) {
            let op = self.pull();
            let right = self.factor();
            left = Box::new(BinaryExpr::new(left, op, right));
        }
        return left;
    }

    fn factor(&mut self) -> Box<dyn Expr> {
        let mut left = self.unary();
        while self.match_token(&TokenType::Star) || self.match_token(&TokenType::Slash) {
            let op = self.pull();
            let right = self.unary();
            left = Box::new(BinaryExpr::new(left, op, right));
        }
        return left;
    }

    fn unary(&mut self) -> Box<dyn Expr> {
        if self.match_token(&TokenType::Plus) || self.match_token(&TokenType::Minus) {
            let op = self.pull();
            let right = self.primary();
            return Box::new(UnaryExpr::new(op, right));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Box<dyn Expr> {        
        if self.match_token(&TokenType::NumberLiteral) {
            return Box::new(LiteralExpr::new(self.pull()));
        }
        if self.eat(&TokenType::LeftParen) {
            let exp = self.expr();
            self.eat(&TokenType::RightParen);
            return exp;
        }
        // TODO: throw error here
        return Box::new(LiteralExpr::new(Token::new(TokenType::Unknown, "".to_string())));
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

    fn eat(&mut self, token_type: &TokenType) -> bool {
        if self.match_token(token_type) {
            self.pull();
            return true;
        }
        return false;
    }

    fn consume(&mut self, token_type: &TokenType) {
        if self.eat(token_type) {
            return;
        }
        panic!("Couldn't consume {:?}. Current type: {:?}", token_type, self.peek().token_type);
    }
    
    fn match_token(&self, token_type: &TokenType) -> bool {
        return !self.is_at_end() && self.peek().token_type == *token_type;
    }
}

use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    start: usize,
    current: usize,
    line: usize
}

impl Lexer {

    pub fn new(input: String) -> Lexer {
        Lexer { input, start: 0, current: 0 , line: 1}
    }

    pub fn next_token(&mut self) -> Token {

        if self.peek_next() == '\0' { return Token::new(TokenType::Eof, String::new()); }

        self.skip_whitespace();

        self.start = self.current;

        let ch = self.advance();

        match ch {
            '(' => { return Token::new(TokenType::LeftParen, "(".to_owned()); }
            ')' => { return Token::new(TokenType::RightParen, ")".to_owned()); }
            '{' => { return Token::new(TokenType::LeftBrace, "{".to_owned()); }
            '}' => { return Token::new(TokenType::RightBrace, "}".to_owned()); }
            ';' => { return Token::new(TokenType::Semicolon, ";".to_owned()); }
            ',' => { return Token::new(TokenType::Comma, ",".to_owned()); }
            '.' => { return Token::new(TokenType::Dot, ".".to_owned()); }
            '-' => { return Token::new(TokenType::Minus, "-".to_owned()); }
            '+' => { return Token::new(TokenType::Plus, "+".to_owned()); }
            '*' => { return Token::new(TokenType::Star, "*".to_owned()); }
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    return Token::new(TokenType::EqualEqual, "==".to_owned());
                } else {
                    return Token::new(TokenType::Equal, "=".to_owned());
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    return Token::new(TokenType::BangEqual, "!=".to_owned());
                } else {
                    return Token::new(TokenType::Bang, "!".to_owned());
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    return Token::new(TokenType::LessEqual, "<=".to_owned());
                } else {
                    return Token::new(TokenType::Less, "<".to_owned());
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    return Token::new(TokenType::GreaterEqual, ">=".to_owned());
                } else {
                    return Token::new(TokenType::Greater, ">".to_owned());
                }
            }
            '"' => {
                return self.string();
            }
            '/' => {
                if self.peek() == '/' {
                    self.advance();
                    self.skip_comment();
                    return self.next_token();
                } else {
                    return Token::new(TokenType::Slash, "/".to_owned());
                }
            }
            'A'..='Z' | 'a'..='z' => { return self.identifier(); }
            '0'..='9' => { return self.number(); }
            _  => Token::new(TokenType::Unknown, String::new())
        }

    }

    pub fn tokens(&mut self) -> Vec<Token> {
        let mut ret : Vec<Token> = Vec::new();
        loop {
            let token = self.next_token();
            if token.token_type == TokenType::Eof {
                break;
            }
            ret.push(token);
        }
        ret.push(Token::new(TokenType::Eof, String::new()));
        return ret;
    }

    fn skip_whitespace(&mut self) {
        loop {
            let ch = self.peek();
            match ch {
                ' ' | '\r' | '\t' => { self.advance(); }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => { return }
            }
        }
    }

    fn skip_comment(&mut self) {
        loop {
            let ch = self.peek();
            match ch {
                '\0' => { break; }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    return;
                }
                _ => { self.advance(); }
            }
        }
    }

    fn peek_next(&self) -> char {
        let peek_index = self.current + 1;
        if peek_index >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(peek_index).unwrap()
        }
    }

    fn peek(&self) -> char {
        if self.current >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.current).unwrap()
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.input.chars().nth(self.current - 1).unwrap()
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphabetic() || self.peek().is_digit(10) { 
            self.advance(); 
        }
        return Token::new(TokenType::Identifier, 
                          self.input.chars()
                            .skip(self.start)
                            .take(self.current - self.start)
                            .collect());
    }

    fn number(&mut self) -> Token { 
        while self.peek().is_digit(10) { self.advance(); }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) { self.advance(); }
        }

        return Token::new(TokenType::NumberLiteral, 
                        self.input.chars()
                            .skip(self.start)
                            .take(self.current - self.start)
                            .collect());
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && self.peek() != '\0' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.peek() == '\0' {
            return Token::new(TokenType::Unknown, "Unterimated string".to_owned());
        }
        self.advance();
        return Token::new(TokenType::StringLiteral, 
                          self.input.chars()
                            .skip(self.start)
                            .take(self.current - self.start)
                            .collect());
    }
}

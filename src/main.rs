use std::io;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;

mod lexer;
mod token;
mod expr;
mod parser;
mod interpreter;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;

fn eval(code: &str) {
    let mut lexer = Lexer::new(code.to_owned());

    let tokens = lexer.tokens();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&statements);
}

fn repl() {

    loop {
        let mut code = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut code).unwrap();
        eval(&code);
        println!()
    }
}

fn read(path: &str) {

    let mut code = String::new();

    let mut file = File::open(path).expect("file not found");
    file.read_to_string(&mut code).expect("something went wrong reading the file");
        
    eval(&code);
}

fn main() {

    if args().len() == 1 {
        repl();
    } else {
        read(&args().nth(1).unwrap());
    }
}

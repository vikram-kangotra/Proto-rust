use std::io;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;

fn eval(code: &str) {
    print!("{}", code);
}

fn repl() {

    loop {
        let mut code = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut code).unwrap();
        eval(&code);
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

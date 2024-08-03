mod ast;
mod interpreter;
mod parser;
mod scanner;

use std::{
    error::Error,
    io::{stdin, Write},
};

use interpreter::Interpreter;
use scanner::Tokens;

fn main() {
    if let Err(err) = repl() {
        eprintln!("fuck: {err}");
    }
}

fn repl() -> Result<(), Box<dyn Error>> {
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        std::io::stdout().flush()?;
        for line in stdin().lines() {
            let line = line.unwrap();
            // println!("got: {line}");
            run(line, &mut interpreter);
        }
    }
}

fn run(script: String, interpreter: &mut Interpreter) {
    let tokens = Tokens::new(script);
    // println!("scanned: {tokens:#?}");

    let stmts = parser::parse(tokens);
    // println!("parsed: {stmts:#?}");
    stmts.for_each(|x| interpreter.evaluate(x));
}

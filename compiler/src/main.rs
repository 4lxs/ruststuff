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

        let mut line = String::new();
        stdin().read_line(&mut line)?;
        run(line, &mut interpreter);
    }
}

fn run(script: String, interpreter: &mut Interpreter) {
    // println!("running {script}");
    let tokens = Tokens::new(script);
    // println!("scanned: {tokens:#?}");

    let stmts = parser::parse(tokens);
    // println!("parsed: {stmts:#?}");
    stmts.for_each(|x| interpreter.evaluate(x));
}

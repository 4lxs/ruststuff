mod ast;
mod parser;
mod scanner;

use std::{error::Error, io::stdin};

use scanner::Tokens;

fn main() {
    if let Err(err) = repl() {
        eprintln!("fuck: {err}");
    }
}

fn repl() -> Result<(), Box<dyn Error>> {
    loop {
        print!("> ");
        for line in stdin().lines() {
            let line = line.unwrap();
            println!("got: {line}");
            run(line);
        }
    }
}

fn run(script: String) {
    let tokens = Tokens::new(script);
    println!("scanned: {tokens:#?}");

    let stmts = parser::parse(tokens);
    for expr in stmts {
        println!("expr: {expr:#?}");
    }
}

mod ast;
mod parser;
mod scanner;

use std::{error::Error, fmt::Binary, io::stdin};

use ast::Expr;
use scanner::{scan_tokens, Location, Token};

fn main() {
    let expr = Expr::Binary(
        Box::new(Expr::Literal(Token {
            token_type: scanner::TokenType::Integer(5),
            location_start: Location::default(),
            location_end: Location::default(),
            lexeme: "".into(),
        })),
        Token {
            token_type: scanner::TokenType::Plus,
            location_start: Location::default(),
            location_end: Location::default(),
            lexeme: "".into(),
        },
        Box::new(Expr::Literal(Token {
            token_type: scanner::TokenType::Integer(5),
            location_start: Location::default(),
            location_end: Location::default(),
            lexeme: "".into(),
        })),
    );
    println!("{expr:#?}");

    return;
    if let Err(err) = repl() {
        eprintln!("fuck: {err}");
    }
}

fn repl() -> Result<(), Box<dyn Error>> {
    loop {
        println!("> ");
        for line in stdin().lines() {
            let line = line.unwrap();
            println!("got: {line}");
            run(&line);
        }
    }
}

fn run(script: &String) {
    match scan_tokens(script) {
        Ok(tokens) => {
            for token in tokens {
                println!("token: {token:?}");
            }
        }
        Err(err) => eprintln!("err: {err}"),
    }
}

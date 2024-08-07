mod ast;
mod interpreter;
mod parser;
mod scanner;

use std::io::{stdin, Write};

use interpreter::Interpreter;

fn main() -> anyhow::Result<()> {
    repl()
}

fn repl() -> anyhow::Result<()> {
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut line = String::new();
        stdin().read_line(&mut line)?;

        if let Err(e) = run(line, &mut interpreter) {
            eprintln!("{e}");
        }
    }
}

fn run(script: String, interpreter: &mut Interpreter) -> anyhow::Result<()> {
    // println!("running {script}");
    let tokens = scanner::scan(script)?;
    // println!("scanned: {tokens:#?}");

    let stmts = parser::parse(tokens)?;
    // println!("parsed: {stmts:#?}");
    stmts.for_each(|x| interpreter.evaluate(x));
    Ok(())
}

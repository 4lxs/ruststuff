use crate::scanner::Token;

#[derive(Debug)]
pub struct Ident {
    pub name: String,
    pub token: Token,
}

impl Ident {
    pub fn new(name: String, token: Token) -> Self {
        Self { name, token }
    }
}

#[derive(Debug)]
pub enum Declaration {
    Var(Ident, Option<Expr>),
    Statement(Statement),
}

#[derive(Debug)]
pub enum Statement {
    Print(Expr),
    Expr(Expr),
    Empty,
}

#[derive(Debug)]
pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
}

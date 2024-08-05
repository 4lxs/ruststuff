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
pub enum Statement {
    Var(Ident, Option<Expr>),
    Print(Expr),
    Expr(Expr),
    Block(Vec<Statement>),
    Empty,
}

#[derive(Debug)]
pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Assignment(Ident, Token, Box<Expr>),
}

impl Expr {
    pub fn as_literal(&self) -> Option<&Token> {
        if let Self::Literal(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

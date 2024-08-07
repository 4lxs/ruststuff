use crate::scanner::Token;

#[derive(Debug, Clone)]
pub struct Ident {
    token: Token,
    name: String,
}

impl Ident {
    pub fn new(token: Token) -> anyhow::Result<Self, Token> {
        if let Some(name) = token.token_type.as_identifier() {
            let name = name.clone();
            Ok(Self { token, name })
        } else {
            Err(token)
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn into_name(self) -> String {
        self.token
            .token_type
            .try_into_identifier()
            .expect("checked in constructor")
    }
}

#[derive(Debug)]
pub enum Statement {
    Block(Vec<Statement>),
    If(Expr, Box<Statement>, Option<Box<Statement>>),
    While(Expr, Box<Statement>),
    Var(Ident, Option<Expr>),
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
    Assignment(Ident, Token, Box<Expr>),
}

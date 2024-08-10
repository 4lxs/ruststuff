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

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Vec<Statement>),
    If(Expr, Vec<Statement>, Option<Vec<Statement>>),
    While(Expr, Vec<Statement>),
    Var(Ident, Option<Expr>),
    Print(Expr),
    Expr(Expr),
    Function(Ident, Vec<Ident>, Vec<Statement>),
    Return(Expr),
    Empty,
}

impl Statement {
    pub fn try_into_block(self) -> Result<Vec<Statement>, Self> {
        if let Self::Block(b) = self {
            Ok(b)
        } else {
            Err(self)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Assignment(Ident, Token, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
}

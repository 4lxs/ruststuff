use std::{fmt::Display, vec};

use anyhow::anyhow;

use crate::{
    ast::{Expr, Ident, Statement},
    scanner::{Token, TokenType, Tokens},
};

#[derive(Debug)]
enum Error {
    IdentifierExpected(Token),
    SemicolonExpected(Token),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IdentifierExpected(next) => {
                write!(f, "identifier expected: before {next:?}")
            }
            Error::SemicolonExpected(next) => {
                write!(f, "semicolon expected: before {next:?}")
            }
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct Statements {
    statements: vec::IntoIter<Statement>,
}

impl Iterator for Statements {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        self.statements.next()
    }
}

pub fn parse(tokens: Tokens) -> anyhow::Result<Statements> {
    let mut parser = Parser::new(tokens);
    let mut decls = Vec::new();
    while let Some(decl) = parser.decleration()? {
        decls.push(decl);
    }
    Ok(Statements {
        statements: decls.into_iter(),
    })
}

pub struct Parser {
    tokens: Tokens,
}

impl Parser {
    fn new(tokens: Tokens) -> Self {
        Self { tokens }
    }

    fn decleration(&mut self) -> anyhow::Result<Option<Statement>> {
        if self.consume(&[TokenType::Eof]).is_some() {
            return Ok(None);
        }

        if self.consume(&[TokenType::Var]).is_some() {
            return Ok(Some(self.var_decl()?));
        }

        Ok(Some(self.statement()?))
    }

    fn var_decl(&mut self) -> anyhow::Result<Statement> {
        let ident = self.ident(None)?;

        let tok = self
            .consume(&[TokenType::Semicolon, TokenType::Equal])
            .ok_or_else(|| self.unexpected("expected ';' or '='"))?;
        if tok.token_type == TokenType::Semicolon {
            return Ok(Statement::Var(ident, None));
        }
        let val = *self.expression()?;
        self.semicolon()?;
        Ok(Statement::Var(ident, Some(val)))
    }

    fn unexpected(&mut self, msg: &str) -> anyhow::Error {
        anyhow!("unexpected token '{:?}': {msg}", self.peek())
    }

    fn peek(&mut self) -> &Token {
        self.tokens
            .peek()
            .expect("scanners last token should be EoF")
    }

    fn next(&mut self) -> Token {
        self.tokens
            .next()
            .expect("scanners last token should be EoF")
    }

    fn ident(&mut self, tok: Option<Token>) -> anyhow::Result<Ident> {
        let tok = tok.unwrap_or_else(|| {
            self.tokens
                .next()
                .expect("last token from scanner should be EOF")
        });
        Ok(Ident::new(tok).map_err(Error::IdentifierExpected)?)
    }

    fn semicolon(&mut self) -> anyhow::Result<Token> {
        Ok(self
            .consume(&[TokenType::Semicolon])
            .ok_or_else(|| Error::SemicolonExpected(self.next()))?)
    }

    fn statement(&mut self) -> anyhow::Result<Statement> {
        if self.consume(&[TokenType::Print]).is_some() {
            let expr = *self.expression()?;
            self.semicolon()?;
            return Ok(Statement::Print(expr));
        }

        if let Some(block) = self.block()? {
            return Ok(block);
        }

        if let Some(cond) = self.conditional()? {
            return Ok(cond);
        }

        if self.consume(&[TokenType::Semicolon]).is_some() {
            return Ok(Statement::Empty);
        }

        let expr = self.expression()?;
        self.semicolon()?;
        Ok(Statement::Expr(*expr))
    }

    fn block(&mut self) -> anyhow::Result<Option<Statement>> {
        if self.consume(&[TokenType::LeftBrace]).is_none() {
            return Ok(None);
        }
        let mut stmts = vec![];
        while self
            .tokens
            .peek()
            .is_some_and(|x| !x.token_type.is_right_brace())
        {
            let stmt = self
                .decleration()?
                .ok_or(anyhow!("unexpected EoF. block not closed"))?;
            stmts.push(stmt);
        }

        self.consume(&[TokenType::RightBrace])
            .ok_or_else(|| self.unexpected("expected '}' to close block"))?;
        Ok(Some(Statement::Block(stmts)))
    }

    fn expression(&mut self) -> anyhow::Result<Box<Expr>> {
        self.assignment()
    }

    fn conditional(&mut self) -> anyhow::Result<Option<Statement>> {
        if self.consume(&[TokenType::If]).is_none() {
            return Ok(None);
        }

        let condition = self.expression()?;

        let when_true = self
            .block()?
            .ok_or_else(|| self.unexpected("expected '{'"))?;

        if self.consume(&[TokenType::Else]).is_none() {
            return Ok(Some(Statement::If(*condition, Box::new(when_true), None)));
        }

        let when_false = self
            .block()?
            .ok_or_else(|| self.unexpected("expected '{'"))?;

        Ok(Some(Statement::If(
            *condition,
            Box::new(when_true),
            Some(Box::new(when_false)),
        )))
    }

    fn assignment(&mut self) -> anyhow::Result<Box<Expr>> {
        let expr = self.logic_or()?;

        if let Some(eq) = self.consume(&[TokenType::Equal]) {
            let rhs = self.expression()?;
            if let Expr::Literal(token) = *expr {
                if token.token_type.is_identifier() {
                    return Ok(Box::new(Expr::Assignment(
                        self.ident(Some(token))?,
                        eq,
                        rhs,
                    )));
                }
            }
            panic!("invalid assignment target");
        }

        Ok(expr)
    }

    fn logic_or(&mut self) -> anyhow::Result<Box<Expr>> {
        self.match_binary(&[TokenType::Or], Parser::logic_and)
    }

    fn logic_and(&mut self) -> anyhow::Result<Box<Expr>> {
        self.match_binary(&[TokenType::And], Parser::equality)
    }

    fn equality(&mut self) -> anyhow::Result<Box<Expr>> {
        self.match_binary(
            &[TokenType::BangEqual, TokenType::EqualEqual],
            Parser::comparison,
        )
    }

    fn comparison(&mut self) -> anyhow::Result<Box<Expr>> {
        self.match_binary(
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
            Parser::term,
        )
    }

    fn term(&mut self) -> anyhow::Result<Box<Expr>> {
        self.match_binary(&[TokenType::Minus, TokenType::Plus], Parser::factor)
    }

    fn factor(&mut self) -> anyhow::Result<Box<Expr>> {
        self.match_binary(&[TokenType::Slash, TokenType::Star], Parser::unary)
    }

    fn unary(&mut self) -> anyhow::Result<Box<Expr>> {
        if let Some(operator) = self.consume(&[TokenType::Bang, TokenType::Minus]) {
            let right = self.unary()?;
            Ok(Box::new(Expr::Unary(operator, right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> anyhow::Result<Box<Expr>> {
        if let Some(operator) = self.consume(&[TokenType::False, TokenType::True, TokenType::Nil]) {
            return Ok(Box::new(Expr::Literal(operator)));
        }

        match self.peek().token_type {
            TokenType::Integer(_)
            | TokenType::String(_)
            | TokenType::Decimal(_)
            | TokenType::Identifier(_) => return Ok(Box::new(Expr::Literal(self.next()))),
            _ => (),
        };

        if self.consume(&[TokenType::LeftParen]).is_some() {
            let expr = self.expression()?;
            self.consume(&[TokenType::RightParen])
                .ok_or_else(|| self.unexpected("expected ')'"))?;
            return Ok(Box::new(Expr::Grouping(expr)));
        }

        Err(self.unexpected("idk what you did"))
    }

    fn match_binary(
        &mut self,
        tokens: &[TokenType],
        f: fn(&mut Parser) -> anyhow::Result<Box<Expr>>,
    ) -> anyhow::Result<Box<Expr>> {
        let mut expr = f(self)?;

        while let Some(operator) = self.consume(tokens) {
            let right = f(self)?;
            expr = Box::new(Expr::Binary(expr, operator, right))
        }
        Ok(expr)
    }

    fn consume(&mut self, tokens: &[TokenType]) -> Option<Token> {
        let curr = self.peek();
        tokens.iter().find(|x| curr.token_type == **x)?;
        self.tokens.next()
    }
}

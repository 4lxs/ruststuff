use std::vec;

use crate::{
    ast::{Expr, Ident, Statement},
    scanner::{Token, TokenType, Tokens},
};

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

pub fn parse(tokens: Tokens) -> Statements {
    let mut parser = Parser::new(tokens);
    let mut decls = Vec::new();
    while let Some(decl) = parser.parse_decleration() {
        decls.push(decl);
    }
    Statements {
        statements: decls.into_iter(),
    }
}

pub struct Parser {
    tokens: Tokens,
}

impl Parser {
    fn new(tokens: Tokens) -> Self {
        Self { tokens }
    }

    fn parse_decleration(&mut self) -> Option<Statement> {
        if self.consume(&[TokenType::Var]).is_some() {
            let ident = self.ident()?;

            let tok = self.consume(&[TokenType::Semicolon, TokenType::Equal])?;
            if tok.token_type == TokenType::Semicolon {
                return Some(Statement::Var(ident, None));
            }
            let val = *self.expression()?;
            self.semicolon()?;
            return Some(Statement::Var(ident, Some(val)));
        }

        Some(self.statement()?)
    }
}

impl Parser {
    fn ident(&mut self) -> Option<Ident> {
        let tok = self.tokens.peek()?;
        if let TokenType::Identifier(id) = &tok.token_type {
            let id = id.clone();
            let tok = self.tokens.next()?;
            Some(Ident::new(id, tok))
        } else {
            None
        }
    }

    fn semicolon(&mut self) -> Option<Token> {
        self.consume(&[TokenType::Semicolon]).or_else(|| {
            eprintln!("expected ';'");
            None
        })
    }

    fn statement(&mut self) -> Option<Statement> {
        if self.consume(&[TokenType::Print]).is_some() {
            let expr = *self.expression()?;
            self.semicolon()?;
            return Some(Statement::Print(expr));
        }

        if self.consume(&[TokenType::LeftBrace]).is_some() {
            let mut stmts = vec![];
            while !self
                .tokens
                .peek()
                .is_some_and(|x| x.token_type.is_right_brace())
            {
                if let Some(stmt) = self.statement() {
                    stmts.push(stmt);
                } else {
                    panic!("expected '}}'");
                }
            }
            self.consume(&[TokenType::RightBrace])
                .expect("loop ends with '}'");
            return Some(Statement::Block(stmts));
        }

        if let Some(expr) = self.expression() {
            self.semicolon()?;
            return Some(Statement::Expr(*expr));
        }
        self.consume(&[TokenType::Semicolon])?;
        Some(Statement::Empty)
    }

    fn expression(&mut self) -> Option<Box<Expr>> {
        self.assignment()
    }

    fn assignment(&mut self) -> Option<Box<Expr>> {
        let expr = self.equality()?;

        if let Some(eq) = self.consume(&[TokenType::Equal]) {
            let rhs = self.expression();

            if let Some(rhs) = rhs {
                if let Expr::Literal(token) = *expr {
                    if let TokenType::Identifier(name) = token.token_type.clone() {
                        return Some(Box::new(Expr::Assignment(Ident { name, token }, eq, rhs)));
                    }
                }
            }
            panic!("invalid assignment target");
        }

        Some(expr)
    }

    fn equality(&mut self) -> Option<Box<Expr>> {
        self.match_binary(
            &[TokenType::BangEqual, TokenType::EqualEqual],
            Parser::comparison,
        )
    }

    fn comparison(&mut self) -> Option<Box<Expr>> {
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

    fn term(&mut self) -> Option<Box<Expr>> {
        self.match_binary(&[TokenType::Minus, TokenType::Plus], Parser::factor)
    }

    fn factor(&mut self) -> Option<Box<Expr>> {
        self.match_binary(&[TokenType::Slash, TokenType::Star], Parser::unary)
    }

    fn unary(&mut self) -> Option<Box<Expr>> {
        if let Some(operator) = self.consume(&[TokenType::Bang, TokenType::Minus]) {
            let right = self.unary()?;
            Some(Box::new(Expr::Unary(operator, right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Option<Box<Expr>> {
        if let Some(operator) = self.consume(&[TokenType::False, TokenType::True, TokenType::Nil]) {
            return Some(Box::new(Expr::Literal(operator)));
        }

        let matched_token = self.tokens.peek().and_then(|curr| match curr.token_type {
            TokenType::Integer(_) => Some(curr),
            TokenType::String(_) => Some(curr),
            TokenType::Decimal(_) => Some(curr),
            TokenType::Identifier(_) => Some(curr),
            _ => None,
        });
        if let Some(_) = matched_token {
            let tok = self.tokens.next()?;
            return Some(Box::new(Expr::Literal(tok)));
        }

        if self.consume(&[TokenType::LeftParen]).is_some() {
            let expr = self.expression()?;
            self.consume(&[TokenType::RightParen])?;
            return Some(Box::new(Expr::Grouping(expr)));
        }

        None
    }

    fn match_binary(
        &mut self,
        tokens: &[TokenType],
        f: fn(&mut Parser) -> Option<Box<Expr>>,
    ) -> Option<Box<Expr>> {
        let mut expr = f(self)?;

        while let Some(operator) = self.consume(tokens) {
            let right = f(self)?;
            expr = Box::new(Expr::Binary(expr, operator, right))
        }
        Some(expr)
    }

    fn consume(&mut self, tokens: &[TokenType]) -> Option<Token> {
        let curr = self.tokens.peek()?;
        tokens.iter().find(|x| curr.token_type == **x)?;
        self.tokens.next()
    }
}

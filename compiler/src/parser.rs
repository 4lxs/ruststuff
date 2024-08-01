use std::{iter::Peekable, vec};

use crate::{
    ast::Expr,
    scanner::{Token, TokenType, Tokens},
};

pub struct Statements {
    statements: Peekable<vec::IntoIter<Expr>>,
}

impl Statements {
    pub fn peek(&mut self) -> Option<&Expr> {
        self.statements.peek()
    }
}

impl Iterator for Statements {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        self.statements.next()
    }
}

pub fn parse(tokens: Tokens) -> Statements {
    let mut parser = Parser::new(tokens);
    let mut statements = Vec::new();
    while let Some(expr) = parser.parse_statement() {
        statements.push(*expr);
    }
    Statements {
        statements: statements.into_iter().peekable(),
    }
}

pub struct Parser {
    tokens: Tokens,
}

impl Parser {
    fn new(tokens: Tokens) -> Self {
        Self { tokens }
    }

    fn parse_statement(&mut self) -> Option<Box<Expr>> {
        self.expression()
    }
}

impl Parser {
    fn expression(&mut self) -> Option<Box<Expr>> {
        self.equality()
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

        let matched_token = self.tokens.next().and_then(|curr| match curr.token_type {
            TokenType::Integer(_) => Some(curr),
            TokenType::String(_) => Some(curr),
            TokenType::Decimal(_) => Some(curr),
            _ => None,
        });
        if let Some(tok) = matched_token {
            return Some(Box::new(Expr::Literal(tok)));
        }

        if let Some(operator) = self.consume(&[TokenType::LeftParen]) {
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

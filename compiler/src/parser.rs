use crate::{
    ast::Expr,
    scanner::{Token, TokenType},
};

#[derive(Debug)]
struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn expression(&self) -> Box<Expr> {
        self.equality()
    }

    fn equality(&self) -> Box<Expr> {
        self.match_binary(
            &[TokenType::BangEqual, TokenType::EqualEqual],
            Parser::comparison,
        )
    }

    fn comparison(&self) -> Box<Expr> {
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

    fn term(&self) -> Box<Expr> {
        self.match_binary(&[TokenType::Minus, TokenType::Plus], Parser::factor)
    }

    fn factor(&self) -> Box<Expr> {
        self.match_binary(&[TokenType::Slash, TokenType::Star], Parser::unary)
    }

    fn unary(&self) -> Box<Expr> {
        if let Some(operator) = self.consume(&[TokenType::Bang, TokenType::Minus]) {
            let right = self.unary();
            Box::new(Expr::Unary(operator, right))
        } else {
            self.primary()
        }
    }

    fn primary(&self) -> Box<Expr> {
        if let Some(operator) = self.consume(&[TokenType::False, TokenType::True, TokenType::Nil]) {
            return Box::new(Expr::Literal(operator));
        }

        let matched_token = self.tokens.get(self.current).and_then(|curr| {
            self.current += 1;
            match curr.token_type {
                TokenType::Integer(_) => Some(*curr),
                TokenType::String(_) => Some(*curr),
                TokenType::Decimal(_) => Some(*curr),
                _ => None,
            }
        });
        if let Some(tok) = matched_token {
            return Box::new(Expr::Literal(tok));
        }

        if let Some(operator) = self.consume(&[TokenType::LeftParen]) {
            let expr = self.expression();
            if self.consume(&[TokenType::RightParen]).is_some() {
                return Box::new(Expr::Grouping(expr));
            } else {
                panic!("fuck");
            }
        }

        panic!("fuck");
    }

    fn match_binary(
        &'a mut self,
        tokens: &[TokenType],
        f: fn(&'a Parser<'a>) -> Box<Expr>,
    ) -> Box<Expr> {
        let mut expr = f(self);

        while let Some(operator) = self.consume(tokens) {
            let right = f(self);
            expr = Box::new(Expr::Binary(expr, operator, right))
        }
        return expr;
    }

    fn consume(&'a mut self, tokens: &[TokenType]) -> Option<Token> {
        self.tokens.get(self.current).and_then(|curr| {
            self.current += 1;
            tokens.iter().find_map(|x| {
                if curr.token_type == *x {
                    Some(curr.clone())
                } else {
                    None
                }
            })
        })
    }
}

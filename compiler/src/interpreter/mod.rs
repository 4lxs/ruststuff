mod environment;
mod value;

use environment::Environment;
use value::{RValue, Value};

use crate::{
    ast::{Declaration, Expr},
    scanner::TokenType,
};

#[derive(Default)]
pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn evaluate(&mut self, decl: Declaration) {
        match decl {
            Declaration::Var(ident, expr) => {
                let val = expr.map(|e| self.expr(e).into_rval(&self.env));
                self.var_decl(ident, val);
            }
            Declaration::Statement(stmt) => self.statement(stmt),
        }
    }
}

impl Interpreter {
    fn var_decl(&mut self, ident: crate::ast::Ident, val: Option<RValue>) {
        println!("setting {} = {val:?}", ident.name);
        self.env.new_var(ident.name, val)
    }

    fn statement(&mut self, stmt: crate::ast::Statement) {
        match stmt {
            crate::ast::Statement::Print(expr) => {
                let val = self.expr(expr);
                self.print_stmt(val.as_rval(&self.env));
            }
            crate::ast::Statement::Expr(expr) => drop(self.expr(expr)),
            crate::ast::Statement::Empty => (),
        }
    }

    fn expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Unary(tok, expr) => match tok.token_type {
                _ => panic!("unexpected token type: {tok:?}"),
            },
            Expr::Binary(l, tok, r) => {
                let lhs = self.expr(*l);
                let rhs = self.expr(*r);
                match tok.token_type {
                    TokenType::Plus => Value::R(lhs.as_rval(&self.env) + rhs.as_rval(&self.env)),
                    TokenType::Minus => Value::R(lhs.as_rval(&self.env) - rhs.as_rval(&self.env)),
                    _ => panic!("unexpected token type: {tok:?}"),
                }
            }
            Expr::Grouping(expr) => self.expr(*expr),
            Expr::Literal(l) => Value::new(l),
        }
    }

    fn print_stmt(&self, val: &RValue) {
        println!("val={val:?}");
    }
}

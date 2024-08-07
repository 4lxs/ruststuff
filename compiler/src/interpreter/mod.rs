mod environment;
mod value;

use environment::Environment;
use value::{RValue, Value};

use crate::{
    ast::{Expr, Statement},
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

    pub fn evaluate(&mut self, stmt: Statement) {
        self.statement(stmt);
    }
}

impl Interpreter {
    fn var_decl(&mut self, ident: crate::ast::Ident, val: Option<RValue>) {
        println!("setting {} = {val:?}", ident.name());
        self.env.new_var(ident.into_name(), val)
    }

    fn statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::Print(expr) => {
                let val = self.expr(expr);
                self.print_stmt(val.as_rval(&self.env));
            }
            Statement::Expr(expr) => drop(self.expr(expr)),
            Statement::Var(ident, expr) => {
                let val = expr.map(|e| self.expr(e).into_rval(&self.env));
                self.var_decl(ident, val);
            }
            Statement::Block(stmts) => {
                self.env.new_scope();
                stmts.into_iter().for_each(|x| self.statement(x));
                self.env.end_scope();
            }
            Statement::If(cond, when_true, when_false) => {
                if self.expr(cond).as_rval(&self.env).is_truthy() {
                    self.statement(*when_true)
                } else if let Some(when_false) = when_false {
                    self.statement(*when_false)
                }
            }
            Statement::Empty => (),
        }
    }

    fn expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Unary(tok, _expr) => match tok.token_type {
                _ => panic!("unexpected token type: {tok:?}"),
            },
            Expr::Binary(l, tok, r) => {
                let lhs = self.expr(*l);
                let rhs = self.expr(*r);
                match tok.token_type {
                    TokenType::Plus => Value::R(lhs.as_rval(&self.env) + rhs.as_rval(&self.env)),
                    TokenType::Minus => Value::R(lhs.as_rval(&self.env) - rhs.as_rval(&self.env)),
                    TokenType::Star => Value::R(lhs.as_rval(&self.env) * rhs.as_rval(&self.env)),
                    TokenType::Slash => Value::R(lhs.as_rval(&self.env) / rhs.as_rval(&self.env)),
                    _ => panic!("unexpected token type: {tok:?}"),
                }
            }
            Expr::Grouping(expr) => self.expr(*expr),
            Expr::Literal(l) => Value::new(l),
            Expr::Assignment(lhs, _, rhs) => {
                let val = self.expr(*rhs).into_rval(&self.env);
                self.env.set_var(lhs.name().clone(), val);
                Value::L(lhs.into_name())
            }
        }
    }

    fn print_stmt(&self, val: &RValue) {
        println!("val={val:?}");
    }
}

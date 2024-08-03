mod environment;
mod value;

use value::Value;

use crate::{
    ast::{Declaration, Expr},
    scanner::TokenType,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&mut self, decl: Declaration) {
        match decl {
            Declaration::Var(ident, expr) => {
                let val = expr.map(|e| self.expr(e));
                self.var_decl(ident, val);
            }
            Declaration::Statement(stmt) => self.statement(stmt),
        }
    }
}

impl Interpreter {
    fn var_decl(&mut self, _ident: crate::ast::Ident, _val: Option<Value>) {
        todo!()
    }

    fn statement(&mut self, stmt: crate::ast::Statement) {
        match stmt {
            crate::ast::Statement::Print(expr) => {
                let val = self.expr(expr);
                self.print_stmt(val);
            }
            crate::ast::Statement::Expr(expr) => drop(self.expr(expr)),
        }
    }

    fn expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Unary(tok, expr) => match tok.token_type {
                _ => panic!("unexpected token type: {tok:?}"),
            },
            Expr::Binary(l, tok, r) => {
                let lval = self.expr(*l);
                let rval = self.expr(*r);
                match tok.token_type {
                    TokenType::Plus => lval + rval,
                    TokenType::Minus => lval - rval,
                    _ => panic!("unexpected token type: {tok:?}"),
                }
            }
            Expr::Grouping(expr) => self.expr(*expr),
            // Expr::Literal(l) if l.token_type.is_identifier() => self.ident
            Expr::Literal(l) => Value::new(l),
        }
    }

    fn print_stmt(&self, val: Value) {
        println!("{val:?}");
    }
}

mod environment;
mod value;

use environment::Environment;
use value::{RValue, Value};

use crate::{
    ast::{Expr, Ident, Statement},
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
        println!(
            "setting {} = {}",
            ident.name(),
            val.as_ref().unwrap_or(&RValue::Null)
        );
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
                    self.statement(Statement::Block(when_true))
                } else if let Some(when_false) = when_false {
                    self.statement(Statement::Block(when_false))
                }
            }
            Statement::While(cond, body) => {
                let cond = self.expr(cond);
                while cond.as_rval(&self.env).is_truthy() {
                    self.statement(Statement::Block(body.clone()));
                }
            }
            Statement::Function(name, params, body) => {
                self.var_decl(name, Some(RValue::Function(params, body)));
            }
            Statement::Return(_) => panic!("unexpected return. not in function"),
            Statement::Empty => (),
        }
    }

    fn expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Unary(tok, expr) => match tok.token_type {
                TokenType::Bang => Value::R(!self.expr(*expr).into_rval(&self.env)),
                _ => panic!("unexpected token type: {tok:?}"),
            },
            Expr::Binary(l, tok, r) => {
                let lhs = self.expr(*l);
                match tok.token_type {
                    TokenType::Or => {
                        if lhs.as_rval(&self.env).is_truthy() {
                            return Value::from(true);
                        } else {
                            return Value::from(self.expr(*r).as_rval(&self.env).is_truthy());
                        }
                    }
                    TokenType::And => {
                        if !lhs.as_rval(&self.env).is_truthy() {
                            return Value::from(false);
                        } else {
                            return Value::from(self.expr(*r).as_rval(&self.env).is_truthy());
                        }
                    }
                    _ => (),
                }
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
            Expr::Call(fun, args) => {
                let Some((params, body)) = self.expr(*fun).into_rval(&self.env).into_funk() else {
                    panic!("can't call non-funky types");
                };

                if args.len() != params.len() {
                    panic!("expected {} arguments, got {}", params.len(), args.len());
                }

                let args = args.into_iter().map(|x| self.expr(x).into_rval(&self.env));

                let params = params.into_iter().zip(args).collect();

                Value::R(self.funk_call(params, body).expect("errored in function"))
            }
        }
    }

    fn print_stmt(&self, val: &RValue) {
        println!("{val}");
    }

    fn funk_call(
        &mut self,
        args: Vec<(Ident, RValue)>,
        body: Vec<Statement>,
    ) -> anyhow::Result<RValue> {
        self.env.new_scope();

        for (ident, value) in args {
            self.env.new_var(ident.into_name(), Some(value));
        }

        let mut ret = RValue::Null;

        for stmt in body {
            if let Statement::Return(value) = stmt {
                ret = self.expr(value).into_rval(&self.env);
                break;
            }
            self.statement(stmt);
        }

        self.env.end_scope();
        Ok(ret)
    }
}

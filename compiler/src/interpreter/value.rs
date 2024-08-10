use std::fmt::Display;

use crate::{
    ast::{Ident, Statement},
    scanner::{Token, TokenType},
};

use super::environment::Environment;

#[derive(Clone)]
pub enum RValue {
    Function(Vec<Ident>, Vec<Statement>),
    Boolean(bool),
    String(String),
    Int(i64),
    Decimal(f64),
    Null,
}

impl RValue {
    pub fn is_truthy(&self) -> bool {
        match *self {
            RValue::Boolean(b) => b,
            RValue::Int(i) => i != 0,
            RValue::Null => false,
            _ => panic!("can't establish truthyness for {self}"),
        }
    }

    pub fn is_funky(&self) -> bool {
        matches!(self, Self::Function(..))
    }

    pub fn into_funk(self) -> Option<(Vec<Ident>, Vec<Statement>)> {
        if let RValue::Function(idents, body) = self {
            Some((idents, body))
        } else {
            None
        }
    }
}

impl Display for RValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RValue::Function(..) => write!(f, "funk"),
            RValue::Boolean(b) => write!(f, "{b}"),
            RValue::String(s) => write!(f, "{s}"),
            RValue::Int(i) => write!(f, "{i}"),
            RValue::Decimal(d) => write!(f, "{d}"),
            RValue::Null => write!(f, "nothing"),
        }
    }
}

pub type LValue = String;

pub enum Value {
    R(RValue),
    L(LValue),
}

impl Value {
    pub fn new(val: Token) -> Self {
        match val.token_type {
            TokenType::Identifier(d) => Self::L(d),

            TokenType::Nil => Self::R(RValue::Null),
            TokenType::String(s) => Self::R(RValue::String(s)),
            TokenType::Integer(i) => Self::R(RValue::Int(i)),
            TokenType::Decimal(d) => Self::R(RValue::Decimal(d)),
            _ => panic!("unexpected token type {val:?}"),
        }
    }

    pub fn as_rval<'a>(&'a self, env: &'a Environment) -> &'a RValue {
        match self {
            Self::R(rval) => rval,
            Self::L(lval) => env.get_var(lval),
        }
    }

    pub fn into_rval(self, env: &Environment) -> RValue {
        match self {
            Self::R(rval) => rval,
            Self::L(lval) => env.get_var(&lval).clone(),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::R(RValue::Boolean(value))
    }
}

impl<'l, 'r> std::ops::Add<&'r RValue> for &'l RValue {
    type Output = RValue;

    fn add(self, rhs: &'r RValue) -> Self::Output {
        match (self, rhs) {
            (RValue::Int(x), RValue::Int(y)) => RValue::Int(x + y),
            (RValue::Int(i), RValue::Decimal(d)) => RValue::Decimal(*i as f64 + d),
            (RValue::Decimal(d), RValue::Int(i)) => RValue::Decimal(d + *i as f64),
            (RValue::String(a), RValue::String(b)) => RValue::String(a.clone() + b),
            _ => panic!("Invalid types for addition"),
        }
    }
}

impl<'l, 'r> std::ops::Sub<&'r RValue> for &'l RValue {
    type Output = RValue;

    fn sub(self, rhs: &'r RValue) -> Self::Output {
        match (self, rhs) {
            (RValue::Int(x), RValue::Int(y)) => RValue::Int(x - y),
            (RValue::Int(i), RValue::Decimal(d)) => RValue::Decimal(*i as f64 - d),
            (RValue::Decimal(d), RValue::Int(i)) => RValue::Decimal(d - *i as f64),
            _ => panic!("Invalid types for subtraction"),
        }
    }
}

impl<'l, 'r> std::ops::Mul<&'r RValue> for &'l RValue {
    type Output = RValue;

    fn mul(self, rhs: &'r RValue) -> Self::Output {
        match (self, rhs) {
            (RValue::Int(x), RValue::Int(y)) => RValue::Int(x * y),
            (RValue::Int(i), RValue::Decimal(d)) => RValue::Decimal(*i as f64 * d),
            (RValue::Decimal(d), RValue::Int(i)) => RValue::Decimal(d * *i as f64),
            _ => panic!("Invalid types for subtraction"),
        }
    }
}

impl<'l, 'r> std::ops::Div<&'r RValue> for &'l RValue {
    type Output = RValue;

    fn div(self, rhs: &'r RValue) -> Self::Output {
        match (self, rhs) {
            (RValue::Int(x), RValue::Int(y)) => RValue::Int(x / y),
            (RValue::Int(i), RValue::Decimal(d)) => RValue::Decimal(*i as f64 / d),
            (RValue::Decimal(d), RValue::Int(i)) => RValue::Decimal(d / *i as f64),
            _ => panic!("Invalid types for subtraction"),
        }
    }
}

impl std::ops::Not for RValue {
    type Output = RValue;

    fn not(self) -> Self::Output {
        RValue::Boolean(!self.is_truthy())
    }
}

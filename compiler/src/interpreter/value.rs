use crate::scanner::{Token, TokenType};

use super::environment::Environment;

#[derive(Debug, Clone)]
pub enum RValue {
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
            _ => panic!("can't establish truthyness for {self:?}"),
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

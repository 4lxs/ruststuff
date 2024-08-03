use crate::scanner::{Token, TokenType};

#[derive(Debug)]
pub enum Value {
    String(String),
    Int(i64),
    Decimal(f64),
}

impl Value {
    pub fn new(val: Token) -> Self {
        match val.token_type {
            TokenType::String(s) => Self::String(s),
            TokenType::Integer(i) => Self::Int(i),
            TokenType::Decimal(d) => Self::Decimal(d),
            _ => panic!("unexpected token type {val:?}"),
        }
    }
}

impl std::ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Self::Int(x), Self::Int(y)) => Self::Int(x + y),
            (Self::Int(i), Self::Decimal(d)) => Self::Decimal(i as f64 + d),
            (Self::Decimal(d), Self::Int(i)) => Self::Decimal(d + i as f64),
            (Self::String(a), Self::String(b)) => Self::String(a + &b),
            _ => panic!("Invalid types for addition"),
        }
    }
}

impl std::ops::Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Self::Int(x), Self::Int(y)) => Self::Int(x - y),
            (Self::Int(i), Self::Decimal(d)) => Self::Decimal(i as f64 - d),
            (Self::Decimal(d), Self::Int(i)) => Self::Decimal(d - i as f64),
            _ => panic!("Invalid types for subtraction"),
        }
    }
}

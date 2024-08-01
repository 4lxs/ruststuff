use std::{
    fmt::Display,
    iter::Peekable,
    ops::{self, Add, Div, Mul, Sub as _},
};

use itertools::{EitherOrBoth, Itertools};

struct Rpn {
    stack: Vec<Number>,
}

impl Rpn {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn eval(&mut self) -> Number {
        if self.stack.len() != 1 {
            panic!("exactly one number should be left over");
        }

        self.stack.pop().unwrap()
    }

    fn parse<T>(&mut self, mut chars: Peekable<T>)
    where
        T: Iterator<Item = char>,
    {
        while let Some(c) = chars.next() {
            match c {
                '+' => self.add(),
                '-' => self.sub(),
                '*' => self.mul(),
                '/' => self.div(),
                '^' => self.pow(),
                c if c.is_whitespace() => continue,
                c => {
                    let mut s = c.to_string();
                    while let Some(&c) = chars.peek() {
                        if c.is_whitespace() {
                            break;
                        }
                        chars.next();

                        s.push(c);
                    }

                    self.push_value(Number::from(s));
                }
            }
        }
    }

    fn push_value(&mut self, val: Number) {
        self.stack.push(val);
    }

    fn binary(&mut self, op: impl Fn(Number, Number) -> Number) {
        if let Some(b) = self.stack.pop() {
            if let Some(a) = self.stack.pop() {
                // top of the stack is the later number
                self.stack.push(op(a, b));
                return;
            }
        }
        panic!("you need 2 numbers to do a binary op");
    }

    fn add(&mut self) {
        self.binary(Number::add);
    }

    fn sub(&mut self) {
        self.binary(Number::sub);
    }

    fn mul(&mut self) {
        self.binary(Number::mul);
    }

    fn div(&mut self) {
        self.binary(Number::div);
    }

    fn pow(&mut self) {
        self.binary(Number::pow);
    }
}

const BASE: i32 = 1_000_000_000;

struct Number {
    value: Vec<i32>,
}

impl Number {
    fn pow(self, _rhs: Number) -> Number {
        todo!()
    }
}

impl From<String> for Number {
    fn from(val: String) -> Self {
        let value = (0..(val.len()))
            .step_by(9)
            .map(|i| {
                let to = (i + 9).min(val.len());
                val[i..to].parse().unwrap()
            })
            .rev()
            .collect();

        Self { value }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.value
                .iter()
                .map(|x| x.to_string())
                .rev()
                .collect::<String>()
        )
    }
}

impl ops::Add<Number> for Number {
    type Output = Self;

    fn add(self, rhs: Number) -> Self::Output {
        let lnums = self.value.into_iter();
        let rnums = rhs.value.into_iter();
        let mut carry = 0;
        let mut value: Vec<i32> = lnums
            .zip_longest(rnums)
            .map(|x| {
                let v = match x {
                    EitherOrBoth::Left(a) => a,
                    EitherOrBoth::Right(a) => a,
                    EitherOrBoth::Both(a, b) => a + b,
                };
                let v = v + carry;
                carry = if v >= BASE { 1 } else { 0 };
                v
            })
            .collect();
        if carry == 1 {
            value.push(1);
        }
        Self::Output { value }
    }
}

impl ops::Sub<Number> for Number {
    type Output = Self;

    fn sub(self, _rhs: Number) -> Self::Output {
        todo!()
    }
}

impl ops::Mul<Number> for Number {
    type Output = Self;

    fn mul(self, _rhs: Number) -> Self::Output {
        todo!()
    }
}

impl ops::Div<Number> for Number {
    type Output = Self;

    fn div(self, _rhs: Number) -> Self::Output {
        todo!()
    }
}

fn main() {
    for line in std::io::stdin().lines() {
        let mut rpn = Rpn::new();
        println!("parsing");
        rpn.parse(line.unwrap().chars().peekable());
        println!("got {}", rpn.eval());
    }
}

use std::{
    collections::HashMap,
    error::Error,
    io::{self, Read},
    process::exit,
    str::Chars,
};

#[derive(Debug, PartialEq)]
enum Value {
    Array(Vec<Value>),
    Object(HashMap<String, Box<Value>>),
    String(String),
    False,
    True,
    Null,
}

#[derive(Debug, PartialEq)]
enum Token {
    Value(Value),
    ArrayTerminator,
    ObjectTerminator,
    Comma,
    Colon,
    None,
}

fn parse_value(line: &str, linenum: usize) -> Result<Token, Box<dyn Error>> {
    let mut parser = Parser {
        linenum,
        chars: line.chars(),
    };

    let val = parser.parse_value();
    if parser.parse_value() != Token::None {
        Err("one value allowed".into())
    } else {
        Ok(val)
    }
}

struct Parser<'a> {
    linenum: usize,
    chars: Chars<'a>,
}

impl Parser<'_> {
    fn err(&self, reason: &str) {
        eprintln!("{}: {reason}", self.linenum);
        exit(1);
    }
    fn parse_value(&mut self) -> Token {
        while let Some(c) = self.chars.next() {
            match c {
                '[' => {
                    let mut arr = Vec::new();
                    let mut expect_val = true;
                    loop {
                        match self.parse_value() {
                            Token::Value(v) => {
                                if !expect_val {
                                    self.err("expecting comma");
                                }
                                arr.push(v);
                            }
                            Token::Comma => {
                                if expect_val {
                                    self.err("expecting value");
                                }
                            }
                            Token::ArrayTerminator => {
                                // trainling comma not allowed
                                if expect_val && !arr.is_empty() {
                                    self.err("terminating comma not allowed");
                                }
                                break;
                            }
                            Token::None => {
                                self.err("unterminated array");
                            }
                            t => {
                                self.err(&format!("unexpected token: {t:?}"));
                            }
                        }
                        expect_val = !expect_val;
                    }
                    return Token::Value(Value::Array(arr));
                }
                '{' => {
                    enum Expect {
                        FieldName,
                        Colon,
                        FieldValue,
                        TerminatorOrComma,
                    }
                    let mut expect = Expect::FieldName;
                    let mut field_name = None;
                    let mut obj = HashMap::new();
                    loop {
                        let tok = self.parse_value();
                        match expect {
                            Expect::FieldName => {
                                if let Token::Value(Value::String(name)) = tok {
                                    field_name = Some(name);
                                } else {
                                    self.err("expected field name");
                                }
                                expect = Expect::Colon;
                            }
                            Expect::Colon => {
                                if tok != Token::Colon {
                                    self.err("expected ':'");
                                }
                                expect = Expect::FieldValue;
                            }
                            Expect::FieldValue => {
                                if let Token::Value(v) = tok {
                                    let old_value = obj.insert(
                                        field_name
                                            .take()
                                            .expect("field value has to be after field name"),
                                        Box::new(v),
                                    );
                                    if old_value.is_some() {
                                        self.err("duplicate field name");
                                    }
                                } else {
                                    self.err("expected value");
                                }
                                expect = Expect::TerminatorOrComma;
                            }
                            Expect::TerminatorOrComma => match tok {
                                Token::ObjectTerminator => {
                                    break;
                                }
                                Token::Comma => {
                                    expect = Expect::FieldName;
                                }
                                t => {
                                    self.err(&format!("expected '}}' or ','. got {t:?}",));
                                }
                            },
                        }
                    }
                    return Token::Value(Value::Object(obj));
                }
                ']' => return Token::ArrayTerminator,
                '}' => return Token::ObjectTerminator,
                ':' => return Token::Colon,
                ',' => return Token::Comma,
                '"' => {
                    let mut s = String::new();
                    for c in self.chars.by_ref() {
                        if c == '"' {
                            return Token::Value(Value::String(s));
                        }
                        s.push(c);
                    }
                    self.err(&format!(
                        "missing string terminator in line {}",
                        self.linenum
                    ));
                }
                // whitespace
                '\x20' | '\x09' | '\x0a' | '\x0d' => continue,
                // literals
                c => match c {
                    'f' => {
                        for c in "alse".chars() {
                            if Some(c) != self.chars.next() {
                                self.err("invalid literal");
                            }
                        }
                        return Token::Value(Value::False);
                    }
                    'n' => {
                        for c in "ull".chars() {
                            if Some(c) != self.chars.next() {
                                self.err("invalid literal");
                            }
                        }
                        return Token::Value(Value::Null);
                    }
                    't' => {
                        for c in "rue".chars() {
                            if Some(c) != self.chars.next() {
                                self.err("invalid literal");
                            }
                        }
                        return Token::Value(Value::True);
                    }
                    _ => {
                        self.err("invalid literal");
                    }
                },
            }
        }
        Token::None
    }
}

/// json parser as defined by https://datatracker.ietf.org/doc/html/rfc8259

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    parse_value(&buf, 0).map(|val| println!("value: {:?}", val))
}

use std::{
    collections::HashMap,
    error, fmt,
    iter::Peekable,
    num::{ParseFloatError, ParseIntError},
    ops,
    str::Chars,
    vec,
};

pub fn scan(script: String) -> anyhow::Result<Tokens> {
    let tokens = scan_tokens(&script)?.into_iter().peekable();
    Ok(Tokens { tokens })
}

#[derive(Debug)]
pub enum ScanError {
    UnexpectedToken(char),
    UnterminatedString,
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UnexpectedToken(c) => write!(f, "encountered an unexpected token {c}"),
            Self::UnterminatedString => write!(f, "encountered an unterminated string"),
            ref x => x.fmt(f),
        }
    }
}

impl error::Error for ScanError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::ParseInt(ref e) => Some(e),
            Self::ParseFloat(ref e) => Some(e),
            _ => None,
        }
    }
}

struct Scanner<'a> {
    source: &'a String,
    chars: Chars<'a>,

    /// current position of .source in full source
    /// actually it's 2 chars before .source as they're
    /// consumed into curr_char and next_char
    location: Location,

    /// the character that was scanned last
    curr_char: char,
    /// the character that will be scanned on
    /// next loop
    next_char: Option<char>,

    /// if false, we're in invalid state
    started: bool,
}

impl<'a> Scanner<'a> {
    /// scanner begins in an invalid state. you
    /// need to call advance() first
    fn new(source: &'a String) -> Self {
        let mut ret = Self {
            source,
            chars: source.chars(),
            location: Location::default(),
            curr_char: '\0',
            next_char: None,
            started: false,
        };
        ret.next_char = ret.chars.next();
        ret
    }

    /// advance the scanner by one character
    /// returns false when it reaches eof
    fn advance(&mut self) -> Option<char> {
        let newline = self.curr_char == '\n';
        if let Some(c) = self.next_char {
            self.next_char = self.chars.next();
            if self.started {
                self.location.advance(newline);
            } else {
                self.started = true;
            }
            Some(c)
        } else {
            None
        }
    }

    fn substring(&self, start: Location, end: Location) -> String {
        self.source[start.char as usize..(end.char + 1) as usize].to_string()
    }
}

#[derive(Debug)]
pub struct Tokens {
    tokens: Peekable<vec::IntoIter<Token>>,
}

impl Tokens {
    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
}

impl Iterator for Tokens {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next()
    }
}

fn scan_tokens(script: &String) -> Result<Vec<Token>, ScanError> {
    let keywords: HashMap<&str, TokenType> = HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ]);

    let mut scanner = Scanner::new(script);
    let mut tokens = Vec::new();

    while let Some(c) = scanner.advance() {
        let location_start = scanner.location;

        let mut if_next = |ch, rif, relse| {
            if scanner.next_char == Some(ch) {
                scanner.advance();
                rif
            } else {
                relse
            }
        };

        let token_type = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,

            '!' => if_next('=', TokenType::BangEqual, TokenType::Bang),
            '=' => if_next('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => if_next('=', TokenType::LessEqual, TokenType::Less),
            '>' => if_next('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => {
                if scanner.next_char == Some('/') {
                    while let Some(c) = scanner.advance() {
                        if c == '\n' {
                            break;
                        }
                    }
                    continue;
                } else {
                    TokenType::Slash
                }
            }

            ' ' | '\t' | '\n' | '\r' => continue,

            '"' => {
                loop {
                    match scanner.advance() {
                        Some(c) => {
                            if c == '"' {
                                break;
                            }
                        }
                        None => return Err(ScanError::UnterminatedString),
                    }
                }
                TokenType::String(scanner.substring(location_start + 1, scanner.location - 1))
            }

            '0'..='9' => {
                let mut saw_dot = false;
                while let Some(c) = scanner.next_char {
                    if !c.is_numeric() && (c != '.' || saw_dot) {
                        break;
                    }
                    if c == '.' {
                        saw_dot = true;
                    }
                    scanner.advance();
                }
                let num = scanner.substring(location_start, scanner.location);
                if saw_dot {
                    match num.parse::<f64>() {
                        Ok(val) => TokenType::Decimal(val),
                        Err(err) => return Err(ScanError::ParseFloat(err)),
                    }
                } else {
                    match num.parse::<i64>() {
                        Ok(val) => TokenType::Integer(val),
                        Err(err) => return Err(ScanError::ParseInt(err)),
                    }
                }
            }

            c if c.is_alphabetic() => {
                while let Some(c) = scanner.next_char {
                    if !c.is_alphabetic() && c != '_' {
                        break;
                    }
                    scanner.advance();
                }
                let ident = scanner.substring(location_start, scanner.location);
                match keywords.get(&ident[..]) {
                    Some(keyword) => (*keyword).clone(),
                    _ => TokenType::Identifier(ident),
                }
            }

            _ => {
                return Err(ScanError::UnexpectedToken(c));
            }
        };
        let location_end = scanner.location;
        let lexeme = scanner.substring(location_start, location_end);
        tokens.push(Token {
            token_type,
            location_start,
            location_end,
            lexeme,
        });
    }
    tokens.push(Token {
        token_type: TokenType::Eof,
        location_start: scanner.location,
        location_end: scanner.location,
        lexeme: "".into(),
    });
    Ok(tokens)
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Location {
    /// source[char] == current character
    char: u64,

    /// number of newline characters up to now
    line: u64,

    /// number of characters since the last newline
    column: u64,
}

impl Location {
    fn advance(&mut self, newline: bool) {
        self.char += 1;
        self.column += 1;
        if newline {
            self.column = 0;
            self.line += 1;
        }
    }
}

impl ops::Add<u64> for Location {
    type Output = Location;

    fn add(self, rhs: u64) -> Self::Output {
        Location {
            char: self.char + rhs,
            column: self.column + rhs,
            ..self
        }
    }
}

impl ops::Sub<u64> for Location {
    type Output = Location;

    fn sub(self, rhs: u64) -> Self::Output {
        Location {
            char: self.char - rhs,
            column: self.column - rhs,
            ..self
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub location_start: Location,
    pub location_end: Location,
    pub lexeme: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Integer(i64),
    Decimal(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl TokenType {
    /// Returns `true` if the token type is [`RightBrace`].
    ///
    /// [`RightBrace`]: TokenType::RightBrace
    #[must_use]
    pub fn is_right_brace(&self) -> bool {
        matches!(self, Self::RightBrace)
    }

    /// Returns `true` if the token type is [`Identifier`].
    ///
    /// [`Identifier`]: TokenType::Identifier
    #[must_use]
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(..))
    }

    pub fn as_identifier(&self) -> Option<&String> {
        if let Self::Identifier(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_identifier(self) -> Result<String, Self> {
        if let Self::Identifier(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

use anyhow::bail;

#[derive(Debug, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Slash,
    Space,
    Tab,
    NewLine,
    String(String),
    Number(Number),
    Identifier(String),
    KeyWord(KeyWord),
}

impl PartialEq for TokenType {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        match (self, other) {
            (TokenType::LeftParen, TokenType::LeftParen)
            | (TokenType::RightParen, TokenType::RightParen)
            | (TokenType::LeftBrace, TokenType::LeftBrace)
            | (TokenType::RightBrace, TokenType::RightBrace)
            | (TokenType::Comma, TokenType::Comma)
            | (TokenType::Dot, TokenType::Dot)
            | (TokenType::Minus, TokenType::Minus)
            | (TokenType::Plus, TokenType::Plus)
            | (TokenType::Semicolon, TokenType::Semicolon)
            | (TokenType::Star, TokenType::Star)
            | (TokenType::Bang, TokenType::Bang)
            | (TokenType::BangEqual, TokenType::BangEqual)
            | (TokenType::Equal, TokenType::Equal)
            | (TokenType::EqualEqual, TokenType::EqualEqual)
            | (TokenType::Greater, TokenType::Greater)
            | (TokenType::GreaterEqual, TokenType::GreaterEqual)
            | (TokenType::Less, TokenType::Less)
            | (TokenType::LessEqual, TokenType::LessEqual)
            | (TokenType::Slash, TokenType::Slash)
            | (TokenType::Space, TokenType::Space)
            | (TokenType::Tab, TokenType::Tab)
            | (TokenType::NewLine, TokenType::NewLine) => true,
            (TokenType::String(s1), TokenType::String(s2)) => s1 == s2,
            (TokenType::Number(n1), TokenType::Number(n2)) => n1 == n2,
            (TokenType::Identifier(s1), TokenType::Identifier(s2)) => s1 == s2,
            (TokenType::KeyWord(k1), TokenType::KeyWord(k2)) => k1 == k2,
            _ => false,
        }
    }
}

impl TokenType {
    pub fn from_char(s: char) -> anyhow::Result<Self> {
        match s {
            '(' => Ok(TokenType::LeftParen),
            ')' => Ok(TokenType::RightParen),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            ',' => Ok(TokenType::Comma),
            '.' => Ok(TokenType::Dot),
            '-' => Ok(TokenType::Minus),
            '+' => Ok(TokenType::Plus),
            ';' => Ok(TokenType::Semicolon),
            '*' => Ok(TokenType::Star),
            '!' => Ok(TokenType::Bang),
            '=' => Ok(TokenType::Equal),
            '>' => Ok(TokenType::Greater),
            '<' => Ok(TokenType::Less),
            '/' => Ok(TokenType::Slash),
            ' ' => Ok(TokenType::Space),
            '\t' => Ok(TokenType::Tab),
            '\n' => Ok(TokenType::NewLine),
            _ => bail!("Invalid token: {}", s),
        }
    }

    pub fn is_skippable(&self) -> bool {
        matches!(self, TokenType::Space | TokenType::Tab | TokenType::NewLine)
    }
}

impl Display for TokenType {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        let string = match self {
            TokenType::LeftParen => "(".to_owned(),
            TokenType::RightParen => ")".to_owned(),
            TokenType::LeftBrace => "{".to_owned(),
            TokenType::RightBrace => "}".to_owned(),
            TokenType::Comma => ",".to_owned(),
            TokenType::Dot => ".".to_owned(),
            TokenType::Minus => "-".to_owned(),
            TokenType::Plus => "+".to_owned(),
            TokenType::Semicolon => ";".to_owned(),
            TokenType::Star => "*".to_owned(),
            TokenType::Bang => "!".to_owned(),
            TokenType::BangEqual => "!=".to_owned(),
            TokenType::Equal => "=".to_owned(),
            TokenType::EqualEqual => "==".to_owned(),
            TokenType::Greater => ">".to_owned(),
            TokenType::GreaterEqual => ">=".to_owned(),
            TokenType::Less => "<".to_owned(),
            TokenType::LessEqual => "<=".to_owned(),
            TokenType::Slash => "/".to_owned(),
            TokenType::Space => " ".to_owned(),
            TokenType::Tab => "\t".to_owned(),
            TokenType::NewLine => "\n".to_owned(),
            TokenType::String(s) => s.clone(),
            TokenType::Number(number) => number.to_string(),
            TokenType::Identifier(s) => s.clone(),
            TokenType::KeyWord(keyword) => keyword.to_string(),
        };
        write!(f, "{}", string)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl Add for Number {
    type Output = Number;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(i1), Number::Integer(i2)) => Number::Integer(i1 + i2),
            (Number::Float(f1), Number::Float(f2)) => Number::Float(f1 + f2),
            _ => panic!("Cannot add integer and float"),
        }
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(i1), Number::Integer(i2)) => Number::Integer(i1 - i2),
            (Number::Float(f1), Number::Float(f2)) => Number::Float(f1 - f2),
            _ => panic!("Cannot subtract integer and float"),
        }
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(i1), Number::Integer(i2)) => Number::Integer(i1 * i2),
            (Number::Float(f1), Number::Float(f2)) => Number::Float(f1 * f2),
            _ => panic!("Cannot multiply integer and float"),
        }
    }
}

impl Div for Number {
    type Output = Number;

    fn div(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(i1), Number::Integer(i2)) => Number::Integer(i1 / i2),
            (Number::Float(f1), Number::Float(f2)) => Number::Float(f1 / f2),
            _ => panic!("Cannot divide integer and float"),
        }
    }
}

impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Number::Integer(i) => Number::Integer(-i),
            Number::Float(f) => Number::Float(-f),
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        match (self, other) {
            (Number::Integer(i1), Number::Integer(i2)) => i1.partial_cmp(i2),
            (Number::Float(f1), Number::Float(f2)) => f1.partial_cmp(f2),
            _ => panic!("Cannot compare integer and float"),
        }
    }
}

impl Display for Number {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{}", i),
            Number::Float(fl) => write!(f, "{}", fl),
        }
    }
}

impl PartialEq for Number {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        match (self, other) {
            (Number::Integer(i1), Number::Integer(i2)) => i1 == i2,
            (Number::Float(f1), Number::Float(f2)) => f1 == f2,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyWord {
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
}

impl FromStr for KeyWord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "and" => Ok(KeyWord::And),
            "class" => Ok(KeyWord::Class),
            "else" => Ok(KeyWord::Else),
            "false" => Ok(KeyWord::False),
            "fun" => Ok(KeyWord::Fun),
            "for" => Ok(KeyWord::For),
            "if" => Ok(KeyWord::If),
            "nil" => Ok(KeyWord::Nil),
            "or" => Ok(KeyWord::Or),
            "print" => Ok(KeyWord::Print),
            "return" => Ok(KeyWord::Return),
            "super" => Ok(KeyWord::Super),
            "this" => Ok(KeyWord::This),
            "true" => Ok(KeyWord::True),
            "var" => Ok(KeyWord::Var),
            "while" => Ok(KeyWord::While),
            _ => bail!("Invalid keyword: {}", s),
        }
    }
}

impl Display for KeyWord {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        let string = match self {
            KeyWord::And => "and".to_owned(),
            KeyWord::Class => "class".to_owned(),
            KeyWord::Else => "else".to_owned(),
            KeyWord::False => "false".to_owned(),
            KeyWord::Fun => "fun".to_owned(),
            KeyWord::For => "for".to_owned(),
            KeyWord::If => "if".to_owned(),
            KeyWord::Nil => "nil".to_owned(),
            KeyWord::Or => "or".to_owned(),
            KeyWord::Print => "print".to_owned(),
            KeyWord::Return => "return".to_owned(),
            KeyWord::Super => "super".to_owned(),
            KeyWord::This => "this".to_owned(),
            KeyWord::True => "true".to_owned(),
            KeyWord::Var => "var".to_owned(),
            KeyWord::While => "while".to_owned(),
        };
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn number() {
        let n1 = super::Number::Integer(1);
        let n2 = super::Number::Integer(2);
        let f1 = super::Number::Float(1.1);
        let f2 = super::Number::Float(2.2);
        let res1 = n1 + n2;
        let res2 = f1 + f2;
        println!("{}", res1);
        println!("{}", res2);
    }
}

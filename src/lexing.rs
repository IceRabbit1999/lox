use std::str::FromStr;

use anyhow::{bail, Context};

use crate::token::{KeyWord, Number, TokenType};

pub fn lexing(path: &str) -> anyhow::Result<Vec<TokenType>> {
    let content = std::fs::read_to_string(path)?;
    let mut iter = content.chars().peekable();

    let mut vec = Vec::new();
    while let Some(&c) = iter.peek() {
        match c {
            '=' => {
                iter.next();
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        vec.push(TokenType::EqualEqual);
                    }
                    _ => vec.push(TokenType::Equal),
                }
            }
            '!' => {
                iter.next();
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        vec.push(TokenType::BangEqual);
                    }
                    _ => vec.push(TokenType::Bang),
                }
            }
            '>' => {
                iter.next();
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        vec.push(TokenType::GreaterEqual);
                    }
                    _ => vec.push(TokenType::Greater),
                }
            }
            '<' => {
                iter.next();
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        vec.push(TokenType::LessEqual);
                    }
                    _ => vec.push(TokenType::Less),
                }
            }
            '/' => {
                iter.next();
                match iter.peek() {
                    Some('/') => {
                        iter.next();
                        while let Some(&c) = iter.peek() {
                            if c == '\n' {
                                break;
                            }
                            iter.next();
                        }
                    }
                    _ => vec.push(TokenType::Slash),
                }
            }
            '"' => {
                iter.next();
                let mut string_literal = String::new();
                let mut valid = false;
                while let Some(&c) = iter.peek() {
                    if c == '"' {
                        iter.next(); // Consume the closing quote
                        vec.push(TokenType::String(string_literal));
                        valid = true;
                        break;
                    }
                    string_literal.push(c);
                    iter.next();
                }
                if iter.peek().is_none() && !valid {
                    bail!("UnterminatedString");
                }
            }
            c if c.is_ascii_digit() => {
                let mut number = String::new();
                let mut is_float = false;
                while let Some(&c) = iter.peek() {
                    if c.is_ascii_digit() {
                        number.push(c);
                        iter.next();
                    } else if c == '.' {
                        if is_float {
                            bail!("DoubleDot");
                        }
                        is_float = true;
                        number.push(c);
                        iter.next();
                    } else {
                        break;
                    }
                }
                if is_float {
                    vec.push(TokenType::Number(Number::Float(number.parse::<f64>().context("Parse Error")?)));
                } else {
                    vec.push(TokenType::Number(Number::Integer(number.parse::<i64>().context("Parse Error")?)));
                }
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut identifier = String::new();
                while let Some(&c) = iter.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        identifier.push(c);
                        iter.next();
                    } else {
                        break;
                    }
                }
                if let Ok(keyword) = KeyWord::from_str(&identifier) {
                    vec.push(TokenType::KeyWord(keyword));
                } else {
                    vec.push(TokenType::Identifier(identifier));
                }
            }
            ' ' => {
                vec.push(TokenType::Space);
                iter.next();
            }
            '\n' => {
                vec.push(TokenType::NewLine);
                iter.next();
            }
            '\t' => {
                vec.push(TokenType::Tab);
                iter.next();
            }
            _ => {
                vec.push(TokenType::from_char(c).context("Scan Error")?);
                iter.next();
            }
        }
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scanning() {
        let path = "tests/scan.lox";
        let tokens = super::lexing(path).unwrap();
        println!("{:?}", tokens);
    }
}

use std::ops::Add;

use crate::{ast::AstNode};
use crate::token::Number;

impl AstNode {
    pub fn evaluate(&self) -> EvaluateResult {
        match self {
            Self::Boolean(v) => EvaluateResult::Boolean(*v),
            Self::Number(v) => EvaluateResult::Number(*v),
            Self::String(v) => EvaluateResult::String(v.clone()),
            Self::Nil => EvaluateResult::Nil,
            Self::Binary { .. } => self.evaluate_binary(),
            Self::Unary { .. } => self.evaluate_unary(),
            Self::Group(node) => node.evaluate(),
        }
    }

    fn evaluate_binary(&self) -> EvaluateResult {
        match self {
            Self::Binary { operator, left, right } => {
                let left = left.evaluate();
                let right = right.evaluate();
                match (left, right) {
                    (EvaluateResult::Number(left), EvaluateResult::Number(right)) => match operator.as_str() {
                        "+" => EvaluateResult::Number(left + right),
                        "-" => EvaluateResult::Number(left - right),
                        "*" => EvaluateResult::Number(left * right),
                        "/" => EvaluateResult::Number(left / right),
                        ">" => EvaluateResult::Boolean(left > right),
                        "<" => EvaluateResult::Boolean(left < right),
                        "==" => EvaluateResult::Boolean(left == right),
                        "!=" => EvaluateResult::Boolean(left != right),
                        ">=" => EvaluateResult::Boolean(left >= right),
                        "<=" => EvaluateResult::Boolean(left <= right),
                        _ => panic!("Invalid operator"),
                    },
                    (EvaluateResult::String(left), EvaluateResult::String(right)) => match operator.as_str() {
                        "+" => EvaluateResult::String(left.add(&right)),
                        _ => panic!("Invalid operator"),
                    },
                    _ => panic!("Invalid operands"),
                }
            }
            _ => panic!("Invalid binary node"),
        }
    }

    fn evaluate_unary(&self) -> EvaluateResult {
        match self {
            Self::Unary { operator, operand } => {
                let op = operand.evaluate();
                match op {
                    EvaluateResult::Number(number) => match operator {
                        '-' => EvaluateResult::Number(-number),
                        _ => panic!("Invalid operator"),
                    },
                    EvaluateResult::Boolean(v) => match operator {
                        '!' => EvaluateResult::Boolean(!v),
                        _ => panic!("Invalid operator"),
                    },
                    _ => panic!("Invalid operand"),
                }
            }
            _ => panic!("Invalid unary node"),
        }
    }
}

#[derive(Debug)]
pub enum EvaluateResult {
    Boolean(bool),
    Number(Number),
    String(String),
    Nil,
}

#[cfg(test)]
mod tests {
    use crate::{
      
        parsing::Parser,
    };
    use crate::lexing::lexing;
    use crate::token::TokenType;

    #[test]
    fn evaluate() {
        let tokens = lexing("tests/evaluate.lox").unwrap();
        let tokens = tokens.into_iter().filter(|token| !token.is_skippable()).collect::<Vec<TokenType>>();
        println!("{:?}", tokens);
        let ast = Parser::new(tokens).parse().unwrap();
        println!("{}", ast);
        let result = ast.evaluate();
        println!("{:?}", result);
    }
}

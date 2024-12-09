use std::ops::Add;

use crate::{ast::AstNode, token::Number};

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
            Self::Print(expr) => expr.evaluate(),
            Self::Variable { value, .. } => {
                if let Some(v) = value {
                    v.evaluate()
                } else {
                    EvaluateResult::Nil
                }
            }
            // The result of Block is now the result of the last expression in the block.
            Self::Block(nodes) => {
                let mut result = EvaluateResult::Nil;
                for node in nodes {
                    result = node.evaluate();
                }
                result
            }
            Self::If { condition: _, exec_branch } => match exec_branch {
                Some(node) => node.evaluate(),
                None => EvaluateResult::Nil,
            },
            Self::Or { left, right } => {
                let left = left.evaluate();
                if let EvaluateResult::Boolean(v) = left {
                    if v {
                        return EvaluateResult::Boolean(true);
                    }
                }
                let right = right.evaluate();
                if let EvaluateResult::Boolean(v) = right {
                    EvaluateResult::Boolean(v)
                } else {
                    panic!("Invalid right operand");
                }
            }
            Self::And { left, right } => {
                let left = left.evaluate();
                if let EvaluateResult::Boolean(v) = left {
                    if !v {
                        return EvaluateResult::Boolean(false);
                    }
                }
                let right = right.evaluate();
                if let EvaluateResult::Boolean(v) = right {
                    EvaluateResult::Boolean(v)
                } else {
                    panic!("Invalid right operand");
                }
            }
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
                        "==" => EvaluateResult::Boolean(left == right),
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

#[derive(Debug, PartialEq)]
pub enum EvaluateResult {
    Boolean(bool),
    Number(Number),
    String(String),
    Nil,
}

#[cfg(test)]
mod tests {
    use crate::{lexing::lexing, parsing::Parser, token::TokenType};

    #[test]
    fn evaluate() {
        let tokens = lexing("tests/evaluate.lox").unwrap();
        let tokens = tokens.into_iter().filter(|token| !token.is_skippable()).collect::<Vec<TokenType>>();
        println!("{:?}", tokens);
        let ast = Parser::new(tokens).parse().unwrap();
        for node in ast {
            println!("{}", node);
            let result = node.evaluate();
            println!("{:?}", result);
        }
    }
}

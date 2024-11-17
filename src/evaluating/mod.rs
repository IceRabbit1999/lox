use std::ops::Add;
use crate::lexer::token::Number;
use crate::parsing::ast::AstNode;

impl AstNode {
    pub fn evaluate(&self) -> EvaluateResult {
        match self {
            Self::Boolean(v) => {
                EvaluateResult::Boolean(*v)
            },
            Self::Number(v) => {
                EvaluateResult::Number(*v)
            },
            Self::String(v) => {
                EvaluateResult::String(v.clone())
            },
            Self::Nil => {
                EvaluateResult::Nil
            },
            Self::Binary { .. } => {
                self.evaluate_binary()
            }
            Self::Unary { .. } => {
                self.evaluate_unary()
            },
            Self::Group(node) => {
                node.evaluate()
            }
        }
    }
    
    fn evaluate_binary(&self) -> EvaluateResult {
        match self {
            Self::Binary { operator, left, right } => {
                let left = left.evaluate();
                let right = right.evaluate();
                match (left, right) {
                    (EvaluateResult::Number(left), EvaluateResult::Number(right)) => {
                        match operator {
                            '+' => EvaluateResult::Number(left + right),
                            '-' => EvaluateResult::Number(left - right),
                            '*' => EvaluateResult::Number(left * right),
                            '/' => EvaluateResult::Number(left / right),
                            _ => panic!("Invalid operator")
                        }
                    },
                    _ => panic!("Invalid operands")
                }
            },
            _ => panic!("Invalid binary node")
        }
    }
    
    fn evaluate_unary(&self) -> EvaluateResult {
        match self {
            Self::Unary { operator, operand } => {
                let op = operand.evaluate();
                match op {
                    EvaluateResult::Number(number) => {
                        match operator {
                            '-' => {
                                EvaluateResult::Number(-number)
                            },
                            _ => panic!("Invalid operator")
                        }
                    },
                    EvaluateResult::Boolean(v) => {
                        match operator {
                            '!' => EvaluateResult::Boolean(!v),
                            _ => panic!("Invalid operator")
                        }
                    }
                    _ => panic!("Invalid operand")
                }
            },
            _ => panic!("Invalid unary node")
        }
    }
}


pub enum EvaluateResult {
    Boolean(bool),
    Number(Number),
    String(String),
    Nil
}

#[cfg(test)]
mod tests {
    use crate::lexer::token::Number;
    use crate::parsing::ast::AstNode;

    #[test]
    fn arithmetic() {
        let node = AstNode::Binary {
            operator: '+',
            left: Box::new(AstNode::Number(Number::Integer(1))),
            right: Box::new(AstNode::Number(Number::Integer(2)))
        };
        let result = node.evaluate();
       
    }
}
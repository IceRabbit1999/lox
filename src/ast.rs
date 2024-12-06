use std::fmt::{Display, Formatter};

use crate::token::Number;
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;

#[derive(Clone, Debug)]
pub enum AstNode {
    Binary {
        left: Box<AstNode>,
        operator: String,
        right: Box<AstNode>,
    },
    Boolean(bool),
    Group(Box<AstNode>),
    Nil,
    Number(Number),
    String(String),
    Unary {
        operator: char,
        operand: Box<AstNode>,
    },
    Print(Box<AstNode>),
    Variable {
        name: String,
        value: Option<Box<AstNode>>,
    },
    Block(Vec<AstNode>),
    If {
        condition: Box<AstNode>,
        exec_branch: Option<Box<AstNode>>,
    },
}

impl Display for AstNode {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            AstNode::Binary { left, operator, right } => write!(f, "({} {} {})", operator, left, right),
            AstNode::Boolean(v) => write!(f, "{}", v),
            AstNode::Group(v) => write!(f, "(group {})", v),
            AstNode::Nil => write!(f, "nil"),
            AstNode::Number(number) => {
                write!(f, "{}", number)
            }
            AstNode::String(s) => write!(f, "{}", s),
            AstNode::Unary { operator, operand } => write!(f, "({} {})", operator, operand),
            AstNode::Print(v) => write!(f, "Print {}", v),
            AstNode::Variable { name, value } => {
                if let Some(value) = value {
                    write!(f, "Variable {} = {}", name, value)
                } else {
                    write!(f, "Variable {} = None", name)
                }
            }
            AstNode::Block(v) => {
                write!(f, "Block [")?;
                for node in v {
                    write!(f, "{}, ", node)?;
                }
                write!(f, "]")
            }
            AstNode::If { condition, exec_branch } => {
                write!(f, "If (condition: {}, exec_branch: {:?})", condition, exec_branch)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::AstNode, token::Number};

    #[test]
    fn display() {
        // 42 + 80 - 94
        let ast = AstNode::Binary {
            left: Box::new(AstNode::Binary {
                left: Box::new(AstNode::Number(Number::Float(42.42))),
                operator: "+".to_string(),
                right: Box::new(AstNode::Number(Number::Integer(80))),
            }),
            operator: "-".to_string(),
            right: Box::new(AstNode::Number(Number::Integer(94))),
        };

        println!("{}", ast);
    }
}

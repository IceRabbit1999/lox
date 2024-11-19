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
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::AstNode;
    use crate::token::Number;

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

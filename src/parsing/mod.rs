pub mod ast;

use anyhow::bail;

use crate::{
    lexer::token::{KeyWord, TokenType},
    parsing::ast::AstNode,
};
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

pub struct Parser {
    tokens: Vec<TokenType>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> anyhow::Result<AstNode> {
        self.expression()
    }

    fn expression(&mut self) -> anyhow::Result<AstNode> {
        self.equality()
    }

    fn equality(&mut self) -> anyhow::Result<AstNode> {
        // equality -> comparison ( ( "!=" | "==" ) comparison )* ;

        let mut node = self.comparison()?;

        loop {
            let token = self.peek();
            if token == &TokenType::BangEqual || token == &TokenType::EqualEqual {
                let operator = token.to_string();
                self.forward()?;
                let right = self.comparison()?;
                node = AstNode::Binary {
                    left: Box::new(node),
                    operator: operator.parse()?,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(node)
    }

    fn comparison(&mut self) -> anyhow::Result<AstNode> {
        // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;

        let mut node = self.term()?;

        loop {
            let token = self.peek();
            if token == &TokenType::Greater || token == &TokenType::GreaterEqual || token == &TokenType::Less || token == &TokenType::LessEqual {
                let operator = token.to_string();
                self.forward()?;
                let right = self.term()?;
                node = AstNode::Binary {
                    left: Box::new(node),
                    operator: operator.parse()?,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn term(&mut self) -> anyhow::Result<AstNode> {
        // term -> factor ( ( "-" | "+" ) factor )* ;
        let mut node = self.factor()?;

        loop {
            let token = self.peek();
            if token == &TokenType::Minus || token == &TokenType::Plus {
                let operator = token.to_string();
                self.forward()?;
                let right = self.factor()?;
                node = AstNode::Binary {
                    left: Box::new(node),
                    operator: operator.parse()?,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(node)
    }

    fn factor(&mut self) -> anyhow::Result<AstNode> {
        // factor -> unary ( ( "/" | "*" ) unary )* ;

        let mut left = self.unary()?;

        loop {
            let token = self.peek().clone();

            if token == TokenType::Slash || token == TokenType::Star {
                let operator = token.to_string();
                self.forward()?;
                let right = self.unary()?;
                left = AstNode::Binary {
                    left: Box::new(left),
                    operator: operator.parse()?,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn unary(&mut self) -> anyhow::Result<AstNode> {
        // unary -> ( "!" | "-" ) unary | primary ;
        let token = self.peek();
        if token == &TokenType::Bang || token == &TokenType::Minus {
            let operator = token.to_string();
            self.forward()?;
            let operand = self.unary()?;
            return Ok(AstNode::Unary {
                operator: operator.parse()?,
                operand: Box::new(operand),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> anyhow::Result<AstNode> {
        // primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

        let token = self.peek().clone();
        let node = match token {
            TokenType::Number(number) => AstNode::Number(number),
            TokenType::String(string) => AstNode::String(string.clone()),
            TokenType::KeyWord(keyword) => match keyword {
                KeyWord::True => AstNode::Boolean(true),
                KeyWord::False => AstNode::Boolean(false),
                KeyWord::Nil => AstNode::Nil,
                _ => {
                    bail!("Unexpected keyword {:?}", keyword)
                }
            },
            TokenType::LeftParen => {
                self.forward()?;
                let expr = self.expression()?;
                if self.peek() != &TokenType::RightParen {
                    bail!("Expected ')' after expression")
                }
                AstNode::Group(Box::new(expr))
            }
            TokenType::RightParen => {
                bail!("Unexpected ')' in parsing primary")
            }
            _ => {
                bail!("Expected expression in parsing primary")
            }
        };
        match self.forward() {
            Ok(_) => {}
            Err(_) => {
                println!("last token")
            }
        }
        Ok(node)
    }

    fn peek(&self) -> &TokenType {
        &self.tokens[self.current]
    }

    fn forward(&mut self) -> anyhow::Result<()> {
        if self.current == self.tokens.len() - 1 {
            bail!("Already at the end of the tokens");
        }
        self.current += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{lexing, token::TokenType},
        parsing::Parser,
    };

    #[test]
    fn test_parse() {
        let path = "tests/parse.lox";
        let tokens = lexing(path).unwrap();
        let tokens = tokens.into_iter().filter(|token| !token.is_skippable()).collect::<Vec<TokenType>>();

        println!("{:?}", tokens);

        let mut parser = Parser::new(tokens);
        let node = parser.parse().unwrap();
        println!("{}", node);
    }
}

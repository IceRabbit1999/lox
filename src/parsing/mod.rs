pub mod ast;

use anyhow::bail;
use crate::lexer::token::{KeyWord, TokenType};
use crate::parsing::ast::AstNode;
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;


pub fn parsing(parser: &mut Parser) -> anyhow::Result<()> {

    Ok(())
}

pub struct Parser {
    tokens: Vec<TokenType>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> anyhow::Result<AstNode> {
        
        todo!()
    }
    
    pub fn factor(&mut self) -> anyhow::Result<AstNode> {
        todo!()
    }
    
    pub fn expression(&mut self) -> anyhow::Result<AstNode> {
        todo!()
    }
    
    pub fn equality(&mut self) -> anyhow::Result<AstNode> {
        todo!()
    }
    
    pub fn comparison(&mut self) -> anyhow::Result<AstNode> {
        todo!()
    }
    
    pub fn term(&mut self) -> anyhow::Result<AstNode> {
        todo!()
    }
    
    pub fn unary(&mut self) -> anyhow::Result<AstNode> {
        todo!()
    }
    
    pub fn primary(&mut self) -> anyhow::Result<AstNode> {
        match self.peek() {
            TokenType::Number(number) => {
                return Ok(AstNode::Number(*number))
            },
            TokenType::String(string) => {
                return Ok(AstNode::String(string.clone()))
            },
            TokenType::KeyWord(keyword) => {
                match keyword {
                    KeyWord::True => {
                        return Ok(AstNode::Boolean(true))
                    },
                    KeyWord::False => {
                        return Ok(AstNode::Boolean(false))
                    },
                    KeyWord::Nil => {
                        return Ok(AstNode::Nil)
                    },
                    _ => bail!("{} is not a primary", keyword)
                }
            },
            _ => {
                // todo: expression
                bail!("Not a primary")
            }
        }
        
        todo!()
    }
    
    fn previous(&self) -> &TokenType {
        &self.tokens[self.current - 1]
    }
    
    fn peek(&self) -> &TokenType {
        &self.tokens[self.current]
    }
    
    fn next(&mut self) -> Option<&TokenType> {
        if self.current == self.tokens.len() {
            return None;
        }
        
        self.current += 1;
        Some(&self.tokens[self.current - 1])
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexing;
    use crate::lexer::token::TokenType;
    use crate::parsing::Parser;

 

    #[test]
    fn test_parse() {
        let path = "tests/parse.lox";
        let tokens = lexing(path).unwrap();
        let tokens = tokens.into_iter().filter(|token|
            !token.is_skippable()
        ).collect::<Vec<TokenType>>();
        
        println!("{:?}", tokens);
        
        let mut parser = Parser::new(tokens);
        let node = parser.primary().unwrap();
        println!("{}", node);
    }
}
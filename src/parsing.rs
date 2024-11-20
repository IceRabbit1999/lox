use std::collections::HashMap;

use anyhow::bail;

use crate::{
    ast::AstNode,
    token::{KeyWord, TokenType},
};

// program        → statement* EOF ;
//
// statement      → exprStmt
//                | printStmt ;
//
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;

// expression     → assignment ;
// assignment     → IDENTIFIER "=" assignment
//                | equality ;

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
    variables: HashMap<String, AstNode>,
}
impl Parser {
    fn find_var(
        &self,
        name: &str,
    ) -> Option<&AstNode> {
        self.variables.get(name)
    }

    fn has_var(
        &self,
        name: &str,
    ) -> bool {
        self.variables.contains_key(name)
    }

    fn insert_var(
        &mut self,
        var: AstNode,
    ) -> anyhow::Result<()> {
        match var {
            AstNode::Variable { name, value } => {
                if let Some(old) = self.variables.insert(name.clone(), AstNode::Variable { name, value }) {
                    println!("Variable {} got shadowed", old);
                };
                Ok(())
            }
            _ => {
                bail!("Expected variable")
            }
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Self {
            tokens,
            current: 0,
            variables: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> anyhow::Result<Vec<AstNode>> {
        self.program()
    }

    fn program(&mut self) -> anyhow::Result<Vec<AstNode>> {
        let mut vec = Vec::new();
        while self.current < self.tokens.len() - 1 {
            let node = self.statement()?;
            vec.push(node);
        }
        Ok(vec)
    }

    fn statement(&mut self) -> anyhow::Result<AstNode> {
        let token = self.peek();
        match token {
            TokenType::KeyWord(keyword) => match keyword {
                KeyWord::Print => {
                    self.forward()?;
                    let expr = self.expression()?;
                    if self.peek() != &TokenType::Semicolon {
                        bail!("Expected ';' after expression")
                    }
                    if let Err(_e) = self.forward() {
                        println!("reach the end of the tokens, last token is {}", self.peek())
                    }
                    Ok(AstNode::Print(Box::new(expr)))
                }
                KeyWord::Var => {
                    self.forward()?;
                    let name = self.peek().clone();
                    if let TokenType::Identifier(_) = name {
                        self.forward()?;
                    } else {
                        bail!("Expected identifier after var")
                    }
                    if self.peek() == &TokenType::Semicolon {
                        let var = AstNode::Variable {
                            name: name.to_string(),
                            value: None,
                        };
                        self.insert_var(var.clone())?;
                        if let Err(_e) = self.forward() {
                            println!("reach the end of the tokens, last token is {}", self.peek())
                        }
                        return Ok(var);
                    }
                    if self.peek() == &TokenType::Equal {
                        self.forward()?;
                    }
                    let expr = self.expression()?;
                    if self.peek() != &TokenType::Semicolon {
                        bail!("Expected ';' after expression")
                    }
                    if let Err(_e) = self.forward() {
                        println!("reach the end of the tokens, last token is {}", self.peek())
                    }
                    let var = AstNode::Variable {
                        name: name.to_string(),
                        value: Some(Box::new(expr)),
                    };
                    self.insert_var(var.clone())?;
                    Ok(var)
                }
                _ => {
                    bail!("Unexpected keyword {:?}", keyword)
                }
            },
            _ => {
                let expr = self.expression()?;
                if self.peek() != &TokenType::Semicolon {
                    bail!("Expected ';' after expression")
                }
                if let Err(_e) = self.forward() {
                    println!("reach the end of the tokens, last token is {}", self.peek())
                }
                Ok(expr)
            }
        }
    }

    fn expression(&mut self) -> anyhow::Result<AstNode> {
        self.assignment()
    }

    fn assignment(&mut self) -> anyhow::Result<AstNode> {
        // assignment -> IDENTIFIER "=" assignment | equality ;
        let token = self.peek().clone();
        match token {
            TokenType::Identifier(var_name) => {
                if !self.has_var(&var_name) {
                    return self.equality();
                }
                self.forward()?;
                let token = self.peek();
                match token {
                    TokenType::Equal => {
                        self.forward()?;
                        let value = self.assignment()?;
                        let var = AstNode::Variable {
                            name: var_name.clone(),
                            value: Some(Box::new(value.clone())),
                        };
                        self.insert_var(var.clone())?;

                        todo!()
                    }
                    _ => {
                        bail!("Expected '=' after variable")
                    }
                }
            }
            _ => self.equality(),
        }
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
                    operator,
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
                    operator,
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
                    operator,
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
                    operator,
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
            TokenType::Identifier(var_name) => {
                if let Some(var) = self.find_var(&var_name) {
                    var.clone()
                } else {
                    bail!("Variable {} not declared", var_name)
                }
            }
            _ => {
                println!("{:?}", token);
                println!("{:?}", self.current);
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
    use crate::{lexing::lexing, parsing::Parser, token::TokenType};

    #[test]
    fn test_parse() {
        let path = "tests/parse.lox";
        let tokens = lexing(path).unwrap();
        let tokens = tokens.into_iter().filter(|token| !token.is_skippable()).collect::<Vec<TokenType>>();

        println!("{:?}", tokens);

        let mut parser = Parser::new(tokens);
        let nodes = parser.parse().unwrap();
        for node in nodes {
            println!("{}", node);
        }
    }

    #[test]
    fn statement() {
        let path = "tests/statement.lox";
        let tokens = lexing(path).unwrap();
        let tokens = tokens.into_iter().filter(|token| !token.is_skippable()).collect::<Vec<TokenType>>();

        println!("{:?}", tokens);

        let mut parser = Parser::new(tokens);
        let node = parser.parse().unwrap();
        for n in node {
            println!("{}", n);
            let result = n.evaluate();
            println!("{:?}", result);
        }
    }
}

use std::collections::HashMap;

use anyhow::bail;

use crate::{
    ast::AstNode,
    evaluating::EvaluateResult,
    token::{KeyWord, TokenType},
};
// program        → declaration* EOF ;

// declaration    → varDeclaration | statement ;

// varDeclaration -> "var" IDENTIFIER ( "=" expression )? ";" ;

// statement      -> exprStmt | printStmt | ifStmt | block ;

// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;
// ifStmt         → "if" expression statement ( "else" statement )? ; (Not a fan of `()` after`if`
// so I personally removed it) block          -> "{" declaration* "}" ;

// expression     → assignment ;
// assignment     -> IDENTIFIER "=" assignment | logic_or ;
// logic_or       -> logic_and ( "or" logic_and )* ;
// logic_and      -> equality ( "and" equality )* ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;

pub struct Parser {
    tokens: Vec<TokenType>,
    current: usize,
    scope: Scope,
}

#[derive(Clone)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    vars: HashMap<String, AstNode>,
}

impl Scope {
    pub fn get_var(
        &mut self,
        name: &str,
    ) -> Option<&mut AstNode> {
        let local = self.vars.get_mut(name);
        match local {
            Some(var) => Some(var),
            None => match &mut self.parent {
                Some(parent) => parent.get_var(name),
                None => None,
            },
        }
    }

    pub fn add_var(
        &mut self,
        var: AstNode,
    ) -> anyhow::Result<()> {
        match var {
            AstNode::Variable { name, value } => {
                let old_var = self.get_var(&name);
                if old_var.is_some() {
                    let old_var = old_var.unwrap();
                    *old_var = AstNode::Variable { name, value };
                } else {
                    self.vars.insert(name.clone(), AstNode::Variable { name, value });
                }
                Ok(())
            }
            _ => bail!("var: {} is not a AstNode::Variable", var),
        }
    }

    pub fn expire(self) -> Self {
        match self.parent {
            Some(parent) => *parent,
            None => self,
        }
    }

    pub fn forward(self) -> Self {
        let parent = Some(Box::new(self));
        let vars = HashMap::new();
        Self { parent, vars }
    }
}

impl Parser {
    fn get_var(
        &mut self,
        name: &str,
    ) -> Option<&mut AstNode> {
        self.scope.get_var(name)
    }

    fn add_var(
        &mut self,
        var: AstNode,
    ) -> anyhow::Result<()> {
        self.scope.add_var(var)
    }

    pub fn forward_scope(&mut self) {
        self.scope = self.scope.clone().forward();
    }

    pub fn expire_scope(&mut self) {
        self.scope = self.scope.clone().expire();
    }
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Self {
            tokens,
            current: 0,
            scope: Scope {
                parent: None,
                vars: HashMap::new(),
            },
        }
    }

    pub fn parse(&mut self) -> anyhow::Result<Vec<AstNode>> {
        self.program()
    }

    fn program(&mut self) -> anyhow::Result<Vec<AstNode>> {
        let mut vec = Vec::new();
        while self.current < self.tokens.len() - 1 {
            let node = self.declaration()?;
            vec.push(node);
        }
        Ok(vec)
    }

    fn declaration(&mut self) -> anyhow::Result<AstNode> {
        // declaration    → varDeclaration | statement ;
        let token = self.peek();
        match token {
            TokenType::KeyWord(KeyWord::Var) => self.var_declaration(),
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> anyhow::Result<AstNode> {
        // varDeclaration -> "var" IDENTIFIER ( "=" expression )? ";" ;
        self.forward()?;
        let token = self.peek().clone();
        let node = match token {
            TokenType::Identifier(var_name) => {
                self.forward()?;
                if self.peek() == &TokenType::Equal {
                    self.forward()?;
                    let value = self.expression()?;
                    if self.peek() != &TokenType::Semicolon {
                        bail!("Expected ';' after expression in var declaration")
                    }
                    let _ = self.forward();
                    let var = AstNode::Variable {
                        name: var_name.clone(),
                        value: Some(Box::new(value)),
                    };
                    self.add_var(var.clone())?;
                    var
                } else {
                    let var = AstNode::Variable {
                        name: var_name.clone(),
                        value: None,
                    };
                    self.add_var(var.clone())?;
                    if self.peek() != &TokenType::Semicolon {
                        bail!("Expected ';' after var declaration")
                    }
                    let _ = self.forward();
                    var
                }
            }
            _ => {
                bail!("Expected identifier after var")
            }
        };

        Ok(node)
    }

    fn statement(&mut self) -> anyhow::Result<AstNode> {
        // statement      -> exprStmt | printStmt | ifStmt |block;
        let token = self.peek();
        match token {
            TokenType::KeyWord(KeyWord::Print) => self.print_statement(),
            TokenType::LeftBrace => self.block(),
            TokenType::KeyWord(KeyWord::If) => self.if_statement(),
            _ => self.expression(),
        }
    }

    fn print_statement(&mut self) -> anyhow::Result<AstNode> {
        self.forward()?;
        let expr = self.expression()?;
        if self.peek() != &TokenType::Semicolon {
            bail!("Expected ';' after expression in print statement")
        }
        let _ = self.forward();
        Ok(AstNode::Print(Box::new(expr)))
    }

    fn block(&mut self) -> anyhow::Result<AstNode> {
        // block          -> "{" declaration* "}" ;
        self.forward()?;
        self.forward_scope();
        let mut vec = Vec::new();
        while self.peek() != &TokenType::RightBrace {
            let node = self.declaration()?;
            vec.push(node);
        }
        self.expire_scope();
        if self.peek() != &TokenType::RightBrace {
            bail!("Expected '}}' after block")
        }
        let _ = self.forward();
        Ok(AstNode::Block(vec))
    }

    fn if_statement(&mut self) -> anyhow::Result<AstNode> {
        // ifStmt         -> "if" expression statement ( "else" statement )? ;
        self.forward()?;
        let condition = self.expression()?;
        let exec_branch;

        println!("condition: {} -> {:?}", condition, condition.evaluate());
        if condition.evaluate() == EvaluateResult::Boolean(true) {
            exec_branch = Some(Box::new(self.statement()?));
            println!("exec_branch: {:?}", exec_branch);
            if self.peek() == &TokenType::KeyWord(KeyWord::Else) {
                // assume the else branch is end of by `}`
                while self.peek() != &TokenType::RightBrace {
                    self.forward()?;
                }
                let _ = self.forward();
            }
            Ok(AstNode::If {
                condition: Box::new(condition),
                exec_branch,
            })
        } else {
            // skip then branch
            while self.peek() != &TokenType::RightBrace {
                self.forward()?;
            }
            self.forward()?;
            if self.peek() == &TokenType::KeyWord(KeyWord::Else) {
                self.forward()?;
                exec_branch = Some(Box::new(self.statement()?));
            } else {
                exec_branch = None
            }

            Ok(AstNode::If {
                condition: Box::new(condition),
                exec_branch,
            })
        }
    }

    fn expression(&mut self) -> anyhow::Result<AstNode> {
        // expression     → assignment ;
        self.assignment()
    }

    fn assignment(&mut self) -> anyhow::Result<AstNode> {
        // assignment     -> IDENTIFIER "=" assignment | logic_or ;
        let token = self.peek().clone();
        match token {
            TokenType::Identifier(var_name) => {
                let v = self.get_var(&var_name);
                if v.is_some() {
                    if self.next().unwrap() == &TokenType::Equal {
                        self.forward()?;
                        self.forward()?;
                        let value = self.assignment()?;
                        let var = AstNode::Variable {
                            name: var_name.clone(),
                            value: Some(Box::new(value)),
                        };
                        self.add_var(var.clone())?;
                        if self.peek() != &TokenType::Semicolon {
                            bail!("Expected ';' after assignment")
                        }
                        if self.next().is_some() {
                            self.forward()?;
                        }

                        Ok(var)
                    }
                    else {
                        self.logic_or()
                    }
                } else {
                    bail!("Variable {} not declared", var_name)
                }
            }
            _ => self.logic_or(),
        }
    }

    fn logic_or(&mut self) -> anyhow::Result<AstNode> {
        // logic_or       -> logic_and ( "or" logic_and )* ;
        let mut left = self.logic_and()?;
        while self.peek() == &TokenType::KeyWord(KeyWord::Or) {
            self.forward()?;
            let right = self.logic_and()?;
            left = AstNode::Or {
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn logic_and(&mut self) -> anyhow::Result<AstNode> {
        // logic_and      -> equality ( "and" equality )* ;
        let mut left = self.equality()?;
        while self.peek() == &TokenType::KeyWord(KeyWord::And) {
            self.forward()?;
            let right = self.equality()?;
            left = AstNode::And {
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
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

        while self.peek() == &TokenType::Greater
            || self.peek() == &TokenType::GreaterEqual
            || self.peek() == &TokenType::Less
            || self.peek() == &TokenType::LessEqual
            || self.peek() == &TokenType::BangEqual
            || self.peek() == &TokenType::EqualEqual
        {
            let operator = self.peek().to_string();
            self.forward()?;
            let right = self.term()?;
            node = AstNode::Binary {
                left: Box::new(node),
                operator,
                right: Box::new(right),
            };
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
                if let Some(var) = self.get_var(&var_name) {
                    var.clone()
                } else {
                    bail!("Variable {} not declared", var_name)
                }
            }
            _ => {
                bail!("Expected expression in parsing primary but found {:?}", token)
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

    fn next(&self) -> Option<&TokenType> {
        if self.current == self.tokens.len() - 1 {
            return None;
        }
        Some(&self.tokens[self.current + 1])
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
        println!("{}", node.len());
        for n in node {
            println!("{}", n);
            let result = n.evaluate();
            println!("{:?}", result);
        }
    }

    #[test]
    fn if_stmt() {
        let path = "tests/if.lox";
        let tokens = lexing(path).unwrap();
        let tokens = tokens.into_iter().filter(|token| !token.is_skippable()).collect::<Vec<TokenType>>();

        let mut parser = Parser::new(tokens);

        let node = parser.parse().unwrap();

        for n in node {
            let res = n.evaluate();
            println!("{} -> {:?}", n, res);
        }
    }

    #[test]
    fn logic() {
        let path = "tests/logic.lox";
        let tokens = lexing(path).unwrap();
        let tokens = tokens.into_iter().filter(|token| !token.is_skippable()).collect::<Vec<TokenType>>();
        let mut parser = Parser::new(tokens);

        let node = parser.parse().unwrap();

        for n in node {
            let res = n.evaluate();
            println!("{} -> {:?}", n, res);
        }
    }
}

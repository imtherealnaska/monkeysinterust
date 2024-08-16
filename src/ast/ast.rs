use std::{any::Any, rc::Rc};

use crate::lexer::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;

    // This is needed for downcasting
    fn as_any(&self) -> &dyn Any;
}

// Equivalent to Statement interface
pub trait Statement: Node {
    fn statement_node(&self);
}

// Equivalent to Expression interface
pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Rc<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Rc<Identifier>,
    pub value: Option<Rc<dyn Expression>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

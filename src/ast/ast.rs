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

use std::fmt;

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program {{")?;
        writeln!(f, "  statements: [")?;
        for (i, stmt) in self.statements.iter().enumerate() {
            writeln!(f, "    {}: {:?},", i, stmt)?;
        }
        writeln!(f, "  ]")?;
        write!(f, "}}")
    }
}

// You'll also need to implement Debug for your Statement types
impl fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Statement({})", self.token_literal())
    }
}

// Implement Debug for LetStatement
impl fmt::Debug for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LetStatement {{ name: {:?}, value: {:?} }}",
            self.name, self.value
        )
    }
}

// Implement Debug for Identifier
impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Identifier({})", self.value)
    }
}

// If you have an Expression trait, implement Debug for it as well
impl fmt::Debug for dyn Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expression({})", self.token_literal())
    }
}

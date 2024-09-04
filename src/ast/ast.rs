use std::fmt::{self, Display};
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
pub trait Expression: Node + fmt::Display {
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

pub struct ReturnStatements {
    pub token: Token,
    pub value: Option<Rc<dyn Expression>>,
}

impl Statement for ReturnStatements {
    fn statement_node(&self) {}
}

impl Node for ReturnStatements {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expr: Option<Rc<dyn Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}
//=======================Display impls =================================

// impl fmt::Display for Program {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

//     }
// }
// use std::fmt::{self, Display};

impl fmt::Display for dyn Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(let_stmt) = self.as_any().downcast_ref::<LetStatement>() {
            write!(f, "{}", let_stmt)
        } else if let Some(ret_stmt) = self.as_any().downcast_ref::<ReturnStatements>() {
            write!(f, "{}", ret_stmt)
        } else if let Some(expr_stmt) = self.as_any().downcast_ref::<ExpressionStatement>() {
            write!(f, "{}", expr_stmt)
        } else {
            write!(f, "Unknown statement type")
        }
    }
}

impl Display for ReturnStatements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.token_literal())?;
        if let Some(value) = &self.value {
            write!(f, "{}", value)?;
        }
        write!(f, ";")
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(expr) = &self.expr {
            write!(f, "{}", expr)
        } else {
            Ok(())
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} = ", self.token.literal, self.name)?;
        if let Some(value) = &self.value {
            write!(f, "{}", value)?;
        }
        write!(f, ";")
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

//=======================Debug impls =================================

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

// Implement Debug for ReturnStatements
impl fmt::Debug for ReturnStatements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LetStatement {{ name: {:?}, value: {:?} }}",
            self.token, self.value
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


#[cfg(test)]
mod tests {
    use crate::lexer::token::TokenType;

    use super::*;
    #[test]
    fn test_program_string() {
        let program = Program {
            statements: vec![Rc::new(LetStatement {
                token: Token {
                    type_: TokenType::Let,
                    literal: "let".to_string(),
                },
                name: Rc::new(Identifier {
                    token: Token {
                        type_: TokenType::Ident,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                }),
                value: Some(Rc::new(Identifier {
                    token: Token {
                        type_: TokenType::Ident,
                        literal: "anotherVar".to_string(),
                    },
                    value: "anotherVar".to_string(),
                }) as Rc<dyn Expression>),
            }) as Rc<dyn Statement>],
        };

        println!("Program: {:?}", program);
        println!("First statement: {:?}", program.statements[0]);
        let actual_output = program.to_string();
        println!("Actual output: '{}'", actual_output);

        assert_eq!(
            actual_output, "let myVar = anotherVar;",
            "program.to_string() wrong. got='{}'",
            actual_output
        );
    }
}

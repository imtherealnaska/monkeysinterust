use std::rc::Rc;

use crate::ast::ast::{Identifier, LetStatement, Statement};
use crate::lexer::lexer::Lexer;
use crate::lexer::token::TokenType;
use crate::{
    ast::ast::Program,
    lexer::{lexer::LexerTrait, token::Token},
};

pub(crate) struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
}
impl Parser {
    fn parse_let_statement(&mut self) -> Option<Rc<LetStatement>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = Rc::new(Identifier {
            value: self.cur_token.literal.clone(),
            token: self.cur_token.clone(),
        });

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        self.next_token();
        while !self.cur_token_is(TokenType::Semicolon) {
            if self.cur_token_is(TokenType::Eof) {
                return None;
            }
            self.next_token()
        }

        // self.next_token();

        Some(Rc::new(LetStatement {
            token,
            name,
            value: None,
        }))
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token.type_ == t {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.type_ == t
    }
}

pub trait ParserTrait {
    fn parse_statement(&mut self) -> Option<Rc<dyn Statement>>;
    fn new(l: Lexer) -> Self;
    fn next_token(&mut self);
    fn parse_program(&mut self) -> Program;
}

impl ParserTrait for Parser {
    fn new(l: Lexer) -> Self {
        let mut p = Parser {
            l,
            cur_token: Token {
                type_: TokenType::Eof,
                literal: "".to_string(),
            },
            peek_token: Token {
                type_: TokenType::Eof,
                literal: "".to_string(),
            },
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, self.l.next_token());
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };
        while self.cur_token.type_ != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Rc<dyn Statement>> {
        match self.cur_token.type_ {
            TokenType::Let => self.parse_let_statement().map(|s| s as Rc<dyn Statement>),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ast::Node;
    use crate::{
        ast::ast::{LetStatement, Statement},
        lexer::lexer::Lexer,
        parser::parser::Parser,
    };

    #[test]
    fn test_let_statements() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        let tests = ["x", "y", "foobar"];

        for (i, expected_identifier) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            assert!(
                test_let_statement(stmt.as_ref(), expected_identifier),
                "test_let_statement failed for index {}",
                i
            );
        }
    }

    fn test_let_statement(stmt: &dyn Statement, expected_identifier: &str) -> bool {
        if stmt.token_literal() != "let" {
            println!("stmt.token_literal not 'let'. got={}", stmt.token_literal());
            return false;
        }

        let let_stmt = match stmt.as_any().downcast_ref::<LetStatement>() {
            Some(ls) => ls,
            None => {
                println!(
                    "stmt is not LetStatement. got={:?}",
                    std::any::type_name::<dyn Statement>()
                );
                return false;
            }
        };

        if let_stmt.name.value != expected_identifier {
            println!(
                "let_stmt.name.value not '{}'. got={}",
                expected_identifier, let_stmt.name.value
            );
            return false;
        }

        if let_stmt.name.token_literal() != expected_identifier {
            println!(
                "let_stmt.name.token_literal() not '{}'. got={}",
                expected_identifier,
                let_stmt.name.token_literal()
            );
            return false;
        }

        true
    }
}

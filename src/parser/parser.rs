use std::rc::Rc;

use crate::ast::ast::{Identifier, LetStatement, ReturnStatements, Statement};
use crate::lexer::lexer::Lexer;
use crate::lexer::token::TokenType;
use crate::{
    ast::ast::Program,
    lexer::{lexer::LexerTrait, token::Token},
};

use super::errors::{ParseError, ParseErrorKind, ParseErrors};

pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: ParseErrors,
}

impl Parser {
    pub fn parse_let_statement(&mut self) -> Option<Rc<LetStatement>> {
        println!("parsing let statement");
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            println!("expected ident , got {:?}", self.peek_token);
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

    pub fn parse_ret_statement(&mut self) -> Option<Rc<ReturnStatements>> {
        println!("parsing return statement");
        let token = self.cur_token.clone();
        self.next_token();

        if !self.expect_peek(TokenType::Semicolon) {
            println!("expected semicolon , got {:?}", self.peek_token);
            return None;
        }

        Some(Rc::new(ReturnStatements {
            token,
            // name,
            value: None,
        }))
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token.type_ == t {
            self.next_token();
            true
        } else {
            self.peek_errors(t);
            false
        }
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.type_ == t
    }

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
            errors: vec![],
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
        println!("Advanced to token: {:?}", self.cur_token);
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };
        while self.cur_token.type_ != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                println!("{:?}", stmt);
                program.statements.push(stmt);
                println!(
                    "Parsed statement. Total statements: {}",
                    program.statements.len()
                );
            }
            self.next_token();
        }
        println!("{:?}", program);
        program
    }

    fn parse_statement(&mut self) -> Option<Rc<dyn Statement>> {
        match self.cur_token.type_ {
            TokenType::Let => self.parse_let_statement().map(|s| s as Rc<dyn Statement>),
            TokenType::Return => self.parse_ret_statement().map(|s| s as Rc<dyn Statement>),
            _ => None,
        }
    }

    fn errors(self) -> ParseErrors {
        self.errors.clone()
    }

    fn peek_errors(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {} , got {} instead",
            t, self.peek_token.type_
        );
        let err_struct = ParseError {
            msg,
            kind: ParseErrorKind::UnexpectedToken,
        };
        println!("error_struct : {err_struct}");
        self.errors.push(err_struct);
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;
    use crate::ast::ast::{Node, ReturnStatements};
    use crate::{
        ast::ast::{LetStatement, Statement},
        lexer::lexer::Lexer,
        parser::parser::Parser,
    };

    #[test]
    fn test_let_statements() {
        // FIXME: There is issue with input not being in the same line .
        let input = "
let x = 5 ; let y = 10 ; let foobar = 838383 ;
";

        println!("Lexer");
        let l = Lexer::new(input);
        println!("parser");
        let mut p = Parser::new(l);
        println!("program");
        let program = p.parse_program();
        check_parser_errors(p);

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

    #[test]
    fn test_ret_statements() {
        // FIXME: There is issue with input not being in the same line .
        let input = "
        return 5 ; return 10 ; return 993322 ;
";

        println!("Lexer");
        let l = Lexer::new(input);
        println!("parser");
        let mut p = Parser::new(l);
        println!("program");
        let program = p.parse_program();
        check_parser_errors(p);

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        for stmt in &program.statements {
            let ret_statements = stmt
                .as_any()
                .downcast_ref::<ReturnStatements>()
                .unwrap_or_else(|| {
                    panic!("Statement not return statement got={:?}", stmt);
                });

            assert_eq!(
                ret_statements.token_literal(),
                "return",
                "returnStatemt.token_literla not 'return' got {}",
                ret_statements.token_literal()
            );
        }
    }

    fn check_parser_errors(p: Parser) {
        let errs = p.errors();
        if errs.is_empty() {
            return;
        }
        eprintln!("parser has {} errors", errs.len());

        for m in errs {
            eprintln!("parser error {}", m.msg);
        }
        panic!("Fix above issues");
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

use super::token::{self, Token, TokenType, Tokens};

pub(crate) struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

pub trait LexerTrait {
    fn new(input: &str) -> Self;
    fn read_char(&mut self);
    fn next_token(&mut self) -> Token;
}

// fn new_token(token_type: TokenType, ch: u8) -> Token {
//     Token {
//         type_: todo!(),
//         literal: todo!(),
//     }
// }

impl LexerTrait for Lexer {
    fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input[self.read_position..].chars().next();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let tok;
        match self.ch {
            Some('=') => tok = Token::new(&Tokens::Assign.to_string(), "="),
            Some(';') => tok = Token::new(&Tokens::Semicolon.to_string(), ";"),
            Some('(') => tok = Token::new(&Tokens::LParen.to_string(), "("),
            Some(')') => tok = Token::new(&Tokens::RParen.to_string(), ")"),
            Some(',') => tok = Token::new(&Tokens::Comma.to_string(), ","),
            Some('+') => tok = Token::new(&Tokens::Plus.to_string(), "+"),
            Some('{') => tok = Token::new(&Tokens::LBrace.to_string(), "{"),
            Some('}') => tok = Token::new(&Tokens::RBrace.to_string(), "}"),
            None => tok = Token::new(&Tokens::Eof.to_string(), ""),
            _ => tok = Token::new("ILLEGAL", &self.ch.unwrap().to_string()),
        };
        self.read_char();
        tok
    }
}

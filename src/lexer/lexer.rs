use crate::lexer::token::lookup_ident;

use super::token::{Token, Tokens};

pub(crate) struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

pub trait LexerTrait {
    fn read_number(&mut self) -> String;
    fn new(input: &str) -> Self;
    fn read_char(&mut self);
    fn next_token(&mut self) -> Token;
    fn read_identifier(&mut self) -> String;
    fn skip_whitespace(&mut self);
    fn peek_char(&self) -> u8;
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphanumeric() || ch == b'_'
}

impl LexerTrait for Lexer {
    fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[pos..self.position].to_string()
    }

    fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        // dbg!(&self.input[self.position..]);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    // let ch = self.ch;
                    self.read_char();
                    let literal = "==".to_string();
                    Token::new(&Tokens::EQ.to_string(), &literal)
                } else {
                    Token::new(&Tokens::Assign.to_string(), "=")
                }
            }
            b';' => Token::new(&Tokens::Semicolon.to_string(), ";"),
            b'(' => Token::new(&Tokens::LParen.to_string(), "("),
            b')' => Token::new(&Tokens::RParen.to_string(), ")"),
            b',' => Token::new(&Tokens::Comma.to_string(), ","),
            b'+' => Token::new(&Tokens::Plus.to_string(), "+"),
            b'{' => Token::new(&Tokens::LBrace.to_string(), "{"),
            b'}' => Token::new(&Tokens::RBrace.to_string(), "}"),
            b'!' => {
                if self.peek_char() == b'=' {
                    // let ch = self.ch;
                    self.read_char();
                    // let literal = ch.to_string() + &self.ch.to_string();
                    let literal = "!=".to_string();
                    Token::new(&Tokens::NotEq.to_string(), &literal)
                } else {
                    Token::new(&Tokens::BANG.to_string(), "!")
                }
            }
            b'*' => Token::new(&Tokens::ASTERISK.to_string(), "*"),
            b'<' => Token::new(&Tokens::LT.to_string(), "<"),
            b'>' => Token::new(&Tokens::GT.to_string(), ">"),
            b'/' => Token::new(&Tokens::SLASH.to_string(), "/"),
            b'-' => Token::new(&Tokens::MINUS.to_string(), "-"),
            _c if is_letter(self.ch) => {
                let literal = self.read_identifier();
                //can use lookup_ident() but this seems ok
                //TODO: Check why this works
                let _type = lookup_ident(&literal);
                // match literal.as_str() {
                //     "let" => Token::new(&Tokens::Let.to_string(), &literal),
                //     "fn" => Token::new(&Tokens::Function.to_string(), &literal),
                //     _ => Token::new(&Tokens::Ident(literal.clone()).to_string(), &literal),
                // }
                return Token::new(&_type.to_string(), &literal);
            }
            _ch if self.ch.is_ascii_digit() => {
                let literal = self.read_number();
                Token::new(&Tokens::Int(literal.clone()).to_string(), &literal)
            }

            b'\0' => Token::new(&Tokens::Eof.to_string(), ""),
            _ => Token::new(&Tokens::Illegal.to_string(), "ILLEGAL"),
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        String::from(&self.input[pos..self.position])
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
                println!("skipping whitespace");
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }
}

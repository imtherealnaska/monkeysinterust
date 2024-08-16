use super::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
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
        dbg!(&self.input[pos..self.position]);
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
                    self.read_char();
                    // let literal = "==".to_string();
                    Token::new(TokenType::EQ, "==")
                } else {
                    dbg!("inside =");
                    Token::new(TokenType::Assign, "=")
                }
            }
            b';' => {
                println!("inside semi");
                Token::new(TokenType::Semicolon, ";")
            }
            b'(' => Token::new(TokenType::LParen, "("),
            b')' => Token::new(TokenType::RParen, ")"),
            b',' => Token::new(TokenType::Comma, ","),
            b'+' => Token::new(TokenType::Plus, "+"),
            b'{' => Token::new(TokenType::LBrace, "{"),
            b'}' => Token::new(TokenType::RBrace, "}"),
            b'!' => {
                if self.peek_char() == b'=' {
                    // let ch = self.ch;
                    self.read_char();
                    // let literal = ch.to_string() + &self.ch.to_string();
                    let literal = "!=".to_string();
                    Token::new(TokenType::NotEq, &literal)
                } else {
                    Token::new(TokenType::BANG, "!")
                }
            }
            b'*' => Token::new(TokenType::ASTERISK, "*"),
            b'<' => Token::new(TokenType::LT, "<"),
            b'>' => Token::new(TokenType::GT, ">"),
            b'/' => Token::new(TokenType::SLASH, "/"),
            b'-' => Token::new(TokenType::MINUS, "-"),
            b'0'..=b'9' => {
                let literal = self.read_number();
                Token::new(TokenType::Int, &literal)
            }
            _c if is_letter(self.ch) => {
                let literal = self.read_identifier();
                //can use lookup_ident() but this seems ok
                //TODO: Check why this works
                let _type = lookup_ident(&literal);
                // println!("literal {literal}");
                // println!("type {_type}");
                // match literal.as_str() {
                //     "let" => Token::new(TokenType::Let, &literal),
                //     "fn" => Token::new(TokenType::Function, &literal),
                //     _ => Token::new(TokenType::Ident, &literal),
                // }
                return Token::new(_type, &literal);
            }
            // _ch if is_digit(self.ch) => {
            b'\0' => Token::new(TokenType::Eof, ""),
            _ => Token::new(TokenType::Illegal, "ILLEGAL"),
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while is_letter(self.ch) || self.ch.is_ascii_digit() {
            self.read_char();
        }
        // dbg!(&self.input[pos..self.position]);
        String::from_utf8_lossy(self.input[pos..self.position].as_bytes()).to_string()
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

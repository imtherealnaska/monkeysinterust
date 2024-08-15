use super::token::{Token, Tokens};

pub(crate) struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

pub trait LexerTrait {
    fn read_number(&mut self) -> String;
    fn new(input: &str) -> Self;
    fn read_char(&mut self);
    fn next_token(&mut self) -> Token;
    fn read_identifier(&mut self) -> String;
    fn skip_whitespace(&mut self);
}

fn is_letter(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

fn is_digit(ch: Option<char>) -> bool {
    let dif = ch.unwrap();
    dif.is_ascii_digit()
}

impl LexerTrait for Lexer {
    fn read_number(&mut self) -> String {
        let pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        self.input[pos..self.position].to_string()
    }

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
        dbg!(&self.input[self.position..]);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let tok;
        self.skip_whitespace();
        println!("once");
        match self.ch {
            Some('=') => tok = Token::new(&Tokens::Assign.to_string(), "="),
            Some(';') => {
                print!("reading ;");
                tok = Token::new(&Tokens::Semicolon.to_string(), ";")
            }
            Some('(') => tok = Token::new(&Tokens::LParen.to_string(), "("),
            Some(')') => tok = Token::new(&Tokens::RParen.to_string(), ")"),
            Some(',') => tok = Token::new(&Tokens::Comma.to_string(), ","),
            Some('+') => tok = Token::new(&Tokens::Plus.to_string(), "+"),
            Some('{') => tok = Token::new(&Tokens::LBrace.to_string(), "{"),
            Some('}') => tok = Token::new(&Tokens::RBrace.to_string(), "}"),
            Some(ch) if is_letter(ch) => {
                let literal = self.read_identifier();
                //can use lookup_ident() but this seems ok
                tok = match literal.as_str() {
                    "let" => Token::new(&Tokens::Let.to_string(), &literal),
                    "fn" => Token::new(&Tokens::Function.to_string(), &literal),
                    _ => Token::new(&Tokens::Ident(literal.clone()).to_string(), &literal),
                };
            }
            Some(ch) if ch.is_digit(10) => {
                let literal = self.read_number();
                tok = Token::new(&Tokens::Int(literal.clone()).to_string(), &literal);
            }

            None => tok = Token::new(&Tokens::Eof.to_string(), ""),
            _ => {
                tok = Token::new(
                    &Tokens::Illegal.to_string(),
                    &self.ch.unwrap_or('\0').to_string(),
                )
            }
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while is_letter(self.ch.unwrap()) {
            self.read_char();
        }
        String::from(&self.input[pos..self.position])
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if self.ch == Some(' ')
                || self.ch == Some('\t')
                || self.ch == Some('\n')
                || self.ch == Some('\r')
            {
                println!("skipping whitespace");
                self.read_char();
            } else {
                break;
            }
        }
    }
}

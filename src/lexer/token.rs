pub type TokenType = String;

pub struct Token {
    type_: String,
    literal: String,
}

impl Token {
    pub fn new(type_: &str, literal: &str) -> Self {
        Self {
            type_: type_.to_string(),
            literal: literal.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tokens {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident(String), // add, foobar, x, y, ...
    Int(String),   // 1343456
    // Operators
    Assign,
    Plus,
    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // Keywords
    Function,
    Let,
}

impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tokens::Illegal => write!(f, "ILLEGAL"),
            Tokens::Eof => write!(f, "EOF"),
            Tokens::Ident(ident) => write!(f, "{}", ident),
            Tokens::Int(int) => write!(f, "{}", int),
            Tokens::Assign => write!(f, "="),
            Tokens::Plus => write!(f, "+"),
            Tokens::Comma => write!(f, ","),
            Tokens::Semicolon => write!(f, ";"),
            Tokens::LParen => write!(f, "("),
            Tokens::RParen => write!(f, ")"),
            Tokens::LBrace => write!(f, "{{"),
            Tokens::RBrace => write!(f, "}}"),
            Tokens::Function => write!(f, "FUNCTION"),
            Tokens::Let => write!(f, "LET"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::lexer::lexer::LexerTrait;

    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = vec![
            (Tokens::Assign, "="),
            (Tokens::Plus, "+"),
            (Tokens::LParen, "("),
            (Tokens::RParen, ")"),
            (Tokens::LBrace, "{"),
            (Tokens::RBrace, "}"),
            (Tokens::Comma, ","),
            (Tokens::Semicolon, ";"),
            (Tokens::Eof, ""),
        ];

        let mut lexer = Lexer::new(input);

        for (i, (expected_token, expected_literal)) in tests.iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                &token.type_,
                &expected_token.to_string(),
                "tests[{}] - tokentype wrong. expected={:?} , got ={:?}",
                i,
                expected_token,
                token.type_
            );

            assert_eq!(
                &token.literal, *expected_literal,
                "tests[{}] - tokentype wrong. expected={:?} , got ={:?}",
                i, expected_literal, token.literal
            );
        }
    }
}

use std::collections::HashMap;

pub type TokenType = String;

pub struct Token {
    pub type_: String,
    pub literal: String,
}

pub fn keywords() -> HashMap<String, Tokens> {
    let mut keywords = HashMap::new();
    keywords.insert("fn".to_string(), Tokens::Function);
    keywords.insert("let".to_string(), Tokens::Let);
    keywords.insert("true".to_string(), Tokens::True);
    keywords.insert("false".to_string(), Tokens::False);
    keywords.insert("if".to_string(), Tokens::If);
    keywords.insert("else".to_string(), Tokens::Else);
    keywords.insert("return".to_string(), Tokens::Return);
    keywords
}

pub fn lookup_ident(ident: &str) -> Tokens {
    let kw = keywords();
    if let Some(token_type) = kw.get(ident) {
        token_type.clone()
    } else {
        Tokens::Ident(ident.to_string())
    }
}

impl Token {
    pub fn new(type_: &str, literal: &str) -> Self {
        Self {
            type_: type_.to_string(),
            literal: literal.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Tokens {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident(String), // add, foobar, x, y, ...
    Int(String),   // 1343456
    // Operators
    Assign,
    Plus,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
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
    True,
    False,
    If,
    Else,
    Return,
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
            Tokens::MINUS => write!(f, "-"),
            Tokens::BANG => write!(f, "!"),
            Tokens::ASTERISK => write!(f, "*"),
            Tokens::SLASH => write!(f, "/"),
            Tokens::LT => write!(f, "<"),
            Tokens::GT => write!(f, ">"),
            Tokens::True => write!(f, "true"),
            Tokens::False => write!(f, "false"),
            Tokens::If => write!(f, "if"),
            Tokens::Else => write!(f, "else"),
            Tokens::Return => write!(f, "return"),
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
        let input = "let five=5;
            let ten = 10 ;
            let add = fn ( x , y ) {
            x + y ;
            };
            !-/*5;
            5 < 10 >5 ;

            if (5 < 10) {
            return true;
            } else {
            return false;
            }
            ";

        let tests = vec![
            (Tokens::Let, "let"),
            (Tokens::Ident("five".to_string()), "five"),
            (Tokens::Assign, "="),
            (Tokens::Int("5".to_string()), "5"),
            (Tokens::Semicolon, ";"),
            (Tokens::Let, "let"),
            (Tokens::Ident("ten".to_string()), "ten"),
            (Tokens::Assign, "="),
            (Tokens::Int("10".to_string()), "10"),
            (Tokens::Semicolon, ";"),
            (Tokens::Let, "let"),
            (Tokens::Ident("add".to_string()), "add"),
            (Tokens::Assign, "="),
            (Tokens::Function, "fn"),
            (Tokens::LParen, "("),
            (Tokens::Ident("x".to_string()), "x"),
            (Tokens::Comma, ","),
            (Tokens::Ident("y".to_string()), "y"),
            (Tokens::RParen, ")"),
            (Tokens::LBrace, "{"),
            (Tokens::Ident("x".to_string()), "x"),
            (Tokens::Plus, "+"),
            (Tokens::Ident("y".to_string()), "y"),
            (Tokens::Semicolon, ";"),
            (Tokens::RBrace, "}"),
            (Tokens::Semicolon, ";"),
            (Tokens::BANG, "!"),
            (Tokens::MINUS, "-"),
            (Tokens::SLASH, "/"),
            (Tokens::ASTERISK, "*"),
            (Tokens::Int("5".to_string()), "5"),
            (Tokens::Semicolon, ";"),
            (Tokens::Int("5".to_string()), "5"),
            (Tokens::LT, "<"),
            (Tokens::Int("10".to_string()), "10"),
            (Tokens::GT, ">"),
            (Tokens::Int("5".to_string()), "5"),
            (Tokens::Semicolon, ";"),
            (Tokens::If, "if"),
            (Tokens::LParen, "("),
            (Tokens::Int("5".to_string()), "5"),
            (Tokens::LT, "<"),
            (Tokens::Int("10".to_string()), "10"),
            (Tokens::RParen, ")"),
            (Tokens::LBrace, "{"),
            (Tokens::Return, "return"),
            (Tokens::True, "true"),
            (Tokens::Semicolon, ";"),
            (Tokens::RBrace, "}"),
            (Tokens::Else, "else"),
            (Tokens::LBrace, "{"),
            (Tokens::Return, "return"),
            (Tokens::False, "false"),
            (Tokens::Semicolon, ";"),
            (Tokens::RBrace, "}"),
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

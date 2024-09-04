use std::collections::HashMap;

pub type TokenString = String;

#[derive(Debug, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub literal: String,
}

pub fn keywords() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("fn".to_string(), TokenType::Function);
    keywords.insert("let".to_string(), TokenType::Let);
    keywords.insert("true".to_string(), TokenType::True);
    keywords.insert("false".to_string(), TokenType::False);
    keywords.insert("if".to_string(), TokenType::If);
    keywords.insert("else".to_string(), TokenType::Else);
    keywords.insert("return".to_string(), TokenType::Return);
    keywords
}

pub fn lookup_ident(ident: &str) -> TokenType {
    let kw = keywords();
    if let Some(token_type) = kw.get(ident) {
        token_type.clone()
    } else {
        TokenType::Ident
    }
}

impl Token {
    pub fn new(type_: TokenType, literal: &str) -> Self {
        Self {
            type_,
            literal: literal.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum TokenType {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident,
    Int, // 1343456
    // Operators
    Assign,
    Plus,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NotEq,
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

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Illegal => write!(f, "ILLEGAL"),
            TokenType::Eof => write!(f, "EOF"),
            TokenType::Ident => write!(f, "Ident"),
            TokenType::Int => write!(f, "int"),
            TokenType::Assign => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::Function => write!(f, "FUNCTION"),
            TokenType::Let => write!(f, "LET"),
            TokenType::MINUS => write!(f, "-"),
            TokenType::BANG => write!(f, "!"),
            TokenType::ASTERISK => write!(f, "*"),
            TokenType::SLASH => write!(f, "/"),
            TokenType::LT => write!(f, "<"),
            TokenType::GT => write!(f, ">"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::Return => write!(f, "return"),
            TokenType::EQ => write!(f, "=="),
            TokenType::NotEq => write!(f, "!="),
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
        let input = "let five=5 ;
let ten = 10 ;
let add = fn ( x , y ) {
x + y ;
};
!-/*5 ;
5 < 10 >5 ;

if (5 < 10 ) {
return true;
} else {
return false;
}

10 == 10 ;
10 != 9 ;";

        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::BANG, "!"),
            (TokenType::MINUS, "-"),
            (TokenType::SLASH, "/"),
            (TokenType::ASTERISK, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::LT, "<"),
            (TokenType::Int, "10"),
            (TokenType::GT, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Int, "5"),
            (TokenType::LT, "<"),
            (TokenType::Int, "10"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Int, "10"),
            (TokenType::EQ, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "10"),
            (TokenType::NotEq, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input);

        for (i, (expected_token, expected_literal)) in tests.iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                token.type_, *expected_token,
                "tests[{}] - tokentype wrong. expected={:?} , got ={:?}",
                i, expected_token, token.type_
            );

            assert_eq!(
                &token.literal, *expected_literal,
                "tests[{}] - tokentype wrong. expected={:?} , got ={:?}",
                i, expected_literal, token.literal
            );
        }
    }
}

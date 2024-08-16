use std::io::{BufRead, Write};

use crate::lexer::{self, lexer::LexerTrait, token};

const PROMPT: &str = ">>";

pub fn start<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut scanner = input.lines();

    while let Some(Ok(line)) = scanner.next() {
        write!(output, "{}", PROMPT).expect("Failed to write tot put ");
        output.flush().expect("failed to flush output");

        let mut lexer = lexer::lexer::Lexer::new(&line);

        loop {
            let token = lexer.next_token();
            if token.type_ == token::TokenType::Eof {
                break;
            }
            writeln!(output, "{:?}", token).expect("failed to write to outpu");
        }
    }
}

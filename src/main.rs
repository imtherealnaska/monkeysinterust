use std::{env, io, process::exit};

pub mod lexer;
pub mod repl;

fn main() {
    let username = match env::var("USER") {
        Ok(user) => user,
        Err(_) => {
            eprintln!("Failed to get the current user.");
            exit(1);
        }
    };

    println!("Hello {} . This is monkey programming language", username);
    println!("Feel free to type in Commands\n");
    repl::repl::start(io::stdin().lock(), io::stdout());
}

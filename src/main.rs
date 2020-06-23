#![feature(box_syntax)]

use std::io::Write;
use std::{fs, io};

mod ast;
mod lex;
mod util;

use lex::{Lexer, TokenStream, TokenType};
use ast::{Eval, Parse, Stmt};

fn run(input: &str) {
    let lexer = Lexer::new(input);
    let tokens;
    match lexer.tokenize() {
        Ok(t) => tokens = t,
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err);
            }
            return;
        }
    }

    // for token in tokens.iter() {
    //     println!("{}", token);
    // }

    let mut token_stream = TokenStream::from(tokens.as_ref());

    while token_stream.peek().ty != TokenType::EOF {
        let stmt;
        match Stmt::parse(token_stream) {
            Ok((token_stream_, stmt_)) => {
                token_stream = token_stream_;
                stmt = stmt_;
            },
            Err(err) => {
                eprintln!("{}", err);
                return;
            },
        }

        // println!("{}", stmt.eval());

        println!("{:#?}", stmt);
        // println!("{}", stmt);
    }
}

fn repl() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        print!("> ");
        io::stdout().flush()?;

        let bytes = stdin.read_line(&mut input)?;

        if bytes == 0 {
            println!("\nExiting");
            return Ok(());
        }

        run(&input);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(filename) = args.get(1) {
        match fs::read_to_string(filename) {
            Ok(src) => run(&src),
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    } else {
        if let Err(err) = repl() {
            eprintln!("{}", err);
        }
    }
}

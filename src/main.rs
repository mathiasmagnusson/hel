#![feature(box_syntax)]

use std::{fs, io};

mod ast;
mod lex;
mod util;

use ast::{File, Parse, Stmt};
use lex::{Lexer, TokenStream, TokenType};

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

    let mut tokens = TokenStream::from(tokens.as_ref());

    match File::parse(tokens) {
        Ok((_, file)) => return println!("{:#?}", file),
        Err(err) => eprintln!(
            "Received error '{}' when parsing as file. Parsing as statement",
            err
        ),
    }

    while tokens.peek().ty != TokenType::EOF {
        let (new_tokens, stmt) = match Stmt::parse(tokens) {
            Ok(stuff) => stuff,
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        };
        tokens = new_tokens;

        println!("{:#?}", stmt); println!("{}", stmt);
    }
}

fn repl() -> io::Result<()> {
    use io::{BufRead, Write};

    print!("> ");
    io::stdout().flush()?;
    for line in io::stdin().lock().lines() {
        let line = line?;

        run(&line);
        print!("> ");
        io::stdout().flush()?;
    }

    println!("\nExiting");
    Ok(())
}

fn main() -> io::Result<()> {
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

    Ok(())
}

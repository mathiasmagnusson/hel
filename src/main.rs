#![feature(box_syntax, box_patterns)]

use std::{fs, io};

mod ast;
mod bytecode;
mod lex;
mod symbol;
mod types;
mod util;

use ast::{File, Parse};
use lex::{Lexer, TokenStream};

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

    let tokens = TokenStream::from(tokens.as_ref());

    let file = match File::parse(tokens) {
        Ok((_, file)) => file,
        Err(err) => return eprintln!("{}", err),
    };

    println!("{:#?}", file);
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

#![feature(box_syntax)]

use std::io::Write;
use std::{fs, io};

mod ast;
mod lex;
mod util;

use lex::{Lexer, TokenStream, TokenType};
use ast::{Eval, Parse, Stmt};

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    Lex, Parse, Eval
}


fn run(mode: Mode, input: &str) {
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

    if mode == Mode::Lex {
        for token in tokens.iter() {
            println!("{}", token);
        }
        return;
    }

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

        match mode {
            Mode::Parse => println!("{:#?}", stmt),
            Mode::Eval => println!("{}", stmt.eval()),
            _ => unreachable!(),
        }
    }
}

fn repl() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();

    print!("(l)ex, (p)arse, or (e)val hel code? ");
    let mode;

    io::stdout().flush()?;
    stdin.read_line(&mut input)?;
    if input.starts_with("l") {
        mode = Mode::Lex;
    } else if input.starts_with("p") {
        mode = Mode::Parse;
    } else if input.starts_with("e") {
        mode = Mode::Eval;
    } else {
        println!("\nBye then...");
        return Ok(());
    }

    loop {
        input.clear();
        print!("> ");
        io::stdout().flush()?;

        if stdin.read_line(&mut input)? == 0 {
            println!("\nExiting");
            return Ok(());
        }

        run(mode, &input);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(filename) = args.get(1) {
        match fs::read_to_string(filename) {
            Ok(src) => run(Mode::Parse, &src),
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

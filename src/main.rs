#![feature(box_syntax)]

use std::io::{Stdin, Write};
use std::{fs, io};

mod ast;
mod lex;
mod util;

use ast::{Eval, File, Parse, Stmt};
use lex::{Lexer, TokenStream, TokenType};

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    Lex,
    Parse,
    Eval,
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

    let mut tokens = TokenStream::from(tokens.as_ref());

    match File::parse(tokens) {
        Ok((_, file)) => return println!("{:#?}", file),
        Err(err) => eprintln!(
            "Received error '{}' when parsing as file. Parsing as expression",
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

        match mode {
            Mode::Parse => { println!("{:#?}", stmt); println!("{}", stmt); },
            Mode::Eval => println!("{}", stmt.eval()),
            _ => unreachable!(),
        }
    }
}

fn repl(mode: Mode, stdin: Stdin, mut input: String) -> io::Result<()> {
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

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

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

    if let Some(filename) = args.get(1) {
        match fs::read_to_string(filename) {
            Ok(src) => run(Mode::Parse, &src),
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    } else {
        if let Err(err) = repl(mode, stdin, input) {
            eprintln!("{}", err);
        }
    }

    Ok(())
}

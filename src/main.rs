use std::{fs, process, io};

use hel::lex::Lexer;
use hel::cst::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(filename) = args.get(1) {
        let input = match fs::read_to_string(filename) {
            Ok(input) => input,
            Err(err) => {
                eprintln!("{}", err);
                process::exit(-1);
            }
        };
        let mut parser = Parser::new(Lexer::from(&input));
        eprintln!("{:#?}", parser.parse_expr());
    } else {
        let mut line = String::new();
        while line != "exit" {
            match io::stdin().read_line(&mut line) {
                Ok(bytes) => if bytes == 0 { return; }
                Err(err) => {
                    eprintln!("{}", err);
                    process::exit(-1);
                }
            }

            let mut parser = Parser::new(Lexer::from(&line));

            let result = parser.parse_type();

            for diagnostic in parser.diagnostics().iter() {
                eprintln!("{:?}", diagnostic);
            }

            if let Some(result) = result {
                println!("{:#?}", result);
            }

            line.clear();
        }
    }
}

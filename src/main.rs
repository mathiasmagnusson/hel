// use hel::ast::{File, Parse};
// use hel::lex::{Lexer, TokenStream};
// use hel::package::Package;

// fn run(input: &str) {
//     let lexer = Lexer::new(input);
//     let tokens;
//     match lexer.tokenize() {
//         Ok(t) => tokens = t,
//         Err(errors) => {
//             for err in errors.0 {
//                 eprintln!("{}", err);
//             }
//             return;
//         }
//     }

//     let tokens = TokenStream::from(tokens.as_ref());

//     let file = match File::parse(tokens) {
//         Ok((_, file)) => file,
//         Err(err) => return eprintln!("{}", err),
//     };

//     println!("{:#?}", file);
// }

// fn repl() -> anyhow::Result<()> {
//     use std::io::{self, BufRead, Write};

//     print!("> ");
//     io::stdout().flush()?;
//     for line in io::stdin().lock().lines() {
//         let line = line?;

//         run(&line);
//         print!("> ");
//         io::stdout().flush()?;
//     }

//     println!("\nExiting");
//     Ok(())
// }

// fn main() -> anyhow::Result<()> {
//     let args: Vec<String> = std::env::args().collect();

//     if let Some(filename) = args.get(1) {
//         let package = Package::new(filename)?;
//         eprintln!("{:#?}", package);
//     } else {
//         if let Err(err) = repl() {
//             eprintln!("{}", err);
//         }
//     }

//     Ok(())
// }

mod lexer;
mod token;
mod token_stream;

pub use lexer::Lexer;
pub use token::{Token, TokenType};
pub use token_stream::TokenStream;

use std::fmt;

#[derive(Debug)]
pub struct Error {
    message: String,
    line: usize,
}

#[derive(Debug)]
pub struct Errors(pub Vec<Error>);

impl Error {
    pub fn new(message: String, line: usize) -> Self {
        Self {
            message,
            line,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at line {}", self.message, self.line)?;

        Ok(())
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for err in &self.0 {
            write!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

impl std::error::Error for Errors {}

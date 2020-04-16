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

impl std::error::Error for Error {}

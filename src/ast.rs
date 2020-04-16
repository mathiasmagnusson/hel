use std::fmt;

use crate::lex::{Token, TokenStream};

mod expr;
mod op;
mod parse;
mod print;
mod stmt;

pub use expr::Expr;
pub use op::{AssignmentOperator, BinaryOperator, UnaryOperator};
pub use parse::Parse;
pub use stmt::Stmt;

#[derive(Debug)]
pub struct Ident {
    name: String,
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Integer(usize),
    Boolean(bool),
    Null,
    This,
}

#[derive(Debug)]
pub struct Error {
    message: String,
    token: Token,
}

impl Error {
    pub fn new(token: Token, message: String) -> Self {
        Self { message, token }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected {}. {}.", self.token, self.message)?;

        Ok(())
    }
}

impl std::error::Error for Error {}

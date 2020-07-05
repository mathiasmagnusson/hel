use std::fmt;

use crate::lex::{Token, TokenStream};

mod eval;
mod op;
mod parse;
mod print;

pub use eval::{Eval, Value};
pub use op::{AssignmentOperator, BinaryOperator, UnaryOperator};
pub use parse::Parse;

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

#[derive(Debug, Clone)]
pub struct Ident(pub String);

#[derive(Debug, Clone)]
pub struct Path(pub Vec<Ident>);

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Integer(usize),
    Bool(bool),
    Null,
}

#[derive(Debug)]
pub struct File {
    pub imports: Vec<Import>,
    pub functions: Vec<Function>,
    pub structs: Vec<Struct>,
    pub globals: Vec<Global>,
}

#[derive(Debug)]
pub struct Import {
    path: Path,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub ident: Ident,
    args: Vec<Argument>,
    return_type: Type,
    body: Expr,
}

#[derive(Debug, Clone)]
pub struct Argument {
    ident: Ident,
    ty: Type,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub ident: Ident,
    fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    name: Ident,
    ty: Type,
}

#[derive(Debug, Clone)]
pub enum Type {
    Path(Path),
    Reference(Box<Type>),
    Tuple(Vec<Type>),
    List(Box<Type>), // TODO: fixed big size e.g.: [u64; 16]
    Function { args: Vec<Type>, ret: Box<Type> },
}

#[derive(Debug, Clone)]
pub struct Global {
    pub ident: Ident,
    ty: Type,
    value: Expr,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let {
        ident: Ident,
        ty: Option<Type>,
        value: Expr,
    },
    For {
        i: Ident,
        iter: Expr,
        body: Box<Stmt>,
    },
    Return(Expr),
    Assign {
        variable: Expr,
        op: AssignmentOperator,
        value: Expr,
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Path(Path),
    Lit(Literal),
    Binary {
        op: BinaryOperator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOperator,
        right: Box<Expr>,
    },
    Evoc {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Indexing {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    FieldAccess {
        left: Box<Expr>,
        field: Ident,
    },
    TupleOrArray(Vec<Expr>),
    StructConstruct {
        path: Path,
        vals: Vec<(Ident, Box<Expr>)>,
    },
    If {
        cond: Box<Expr>,
        then: Box<Stmt>,
        els: Option<Box<Stmt>>,
    },
    Loop(Box<Expr>),
    Block(Vec<Stmt>),
    Closure {
        args: Vec<Ident>,
        body: Box<Expr>,
    }
}

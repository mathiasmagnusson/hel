use derive_getters::Getters;
use std::ops;

use crate::lex::Token;
use crate::text::{TextSpan, WithSpan};

mod new;
mod operators;
mod parse;
mod span;

pub use parse::Parser;

use operators::{AssignmentOperator, BinaryOperator, UnaryOperator};

#[derive(Debug, Clone, Getters)]
pub struct Package {
    config: Config,
    root: Module,
}

#[derive(Debug, Clone, Getters)]
pub struct Config {}

#[derive(Debug, Clone, Getters)]
pub struct Module {
    //          exported
    items: Vec<(bool, Item)>,
    globals: Vec<(bool, Global)>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Import(Import),
    Function(Function),
    TypeDecls(TypeDecl),
}

#[derive(Debug, Clone, Getters)]
pub struct Global {
    let_token: Token,
    ident: Ident,
    ty: Type,
    value: Expr,
}

#[derive(Debug, Clone, Getters)]
pub struct Import {
    import_token: Token,
    path: Path,
}

#[derive(Debug, Clone, Getters)]
pub struct Function {
    fn_token: Token,
    ident: Ident,
    args: Vec<(Ident, Type)>,
    ret_type: Type,
    body: Expr,
}

#[derive(Debug, Clone, Getters)]
pub struct TypeDecl {
    type_token: Token,
    ident: Ident,
    ty: Type,
}

#[derive(Debug, Clone)]
pub struct Ident(String, TextSpan);

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl ops::Deref for Ident {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Path(Vec<Ident>);

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl ops::Deref for Path {
    type Target = Vec<Ident>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Type = WithSpan<TypeInner>;

#[derive(Debug, Clone)]
pub enum TypeInner {
    Path(Path),                     // path::to::type
    Reference(Box<Type>),           // &type
    Tuple(Vec<Type>),               // (type1, type2)
    InPlaceDynamicArray(Box<Type>), // [type]
    SizedArray(Box<Type>, Expr),    // [type; size]
    DynamicArray(Box<Type>),        // [type..]
    Slice(Box<Type>),               // &[type]
    // fn (arg1, arg2) -> ret
    Function {
        args: Vec<Type>,
        returns: Box<Type>,
    },
    // {yields} or {yields, returns}
    Generator {
        yields: Box<Type>,
        returns: Option<Box<Type>>,
    },
    // The following are only allowed in TypeDecl, Struct, or Enum:
    Struct(Vec<(Ident, Type)>),
    // Enum(Vec<(Ident, Option<Type>)>), TODO!
}

#[derive(Debug, Clone)]
pub enum Expr {
    Path(Path),
    Literal(Literal),
    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expr>,
    },
    Evoc {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Indexing {
        into: Box<Expr>,
        index: Box<Expr>,
    },
    FieldAccess {
        on: Box<Expr>,
        field: Ident,
    },
    Tuple(Vec<Expr>),
    SizedArray(Vec<Expr>, Option<usize>), // [val1, val2] or [val; size]
    DynamicArray(Vec<Expr>),              // [1, 2, 3, ..]
    Struct {
        ty: Path,
        values: Vec<(Ident, Expr)>,
    },
    If {
        condition: Box<Expr>,
        then: Box<Stmt>,
        els: Option<Box<Stmt>>,
    },
    Loop(Box<Expr>),
    Block(Vec<Stmt>),
    Closure {
        args: Vec<Ident>,
        body: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Integer(usize),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let {
        ident: Ident,
        ty: Option<Type>,
        value: Expr,
    },
    Assign {
        var: Expr,
        op: AssignmentOperator,
        value: Expr,
    },
    For {
        i: Ident,
        iter: Expr,
        body: Box<Stmt>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Return(Expr),
    Yield(Expr),
    Break(Expr),
}

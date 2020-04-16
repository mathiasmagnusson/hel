use super::{BinaryOperator, Ident, Literal, Stmt, UnaryOperator};

#[derive(Debug)]
pub enum Expr {
    Ident(Ident),
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
    Tuple(Vec<Expr>),
    Array(Vec<Expr>),
    Indexing {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    Struct {
        ident: Ident,
        vals: Vec<(Ident, Box<Expr>)>,
    },
    If {
        cond: Box<Expr>,
        then: Box<Stmt>,
        els: Option<Box<Stmt>>,
    },
    Loop(Box<Expr>),
    Block(Vec<Stmt>),
}

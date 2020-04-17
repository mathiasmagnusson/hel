use super::{Ident, Expr, AssignmentOperator};
#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Let { ident: Ident, ty: Option<Ident>, value: Expr },
    For { i: Ident, iter: Expr, body: Box<Stmt> },
    Return(Expr),
    Print(Expr),
    Assign { ident: Ident, op: AssignmentOperator, value: Expr },
}

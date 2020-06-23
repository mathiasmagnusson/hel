use super::{BinaryOperator, Expr, Literal, UnaryOperator, Stmt};

pub enum Value {
    String(String),
    Integer(isize),
    Bool(bool),
    Pointer(usize),
}

pub trait Eval {
    fn eval(&self) -> Value;
}

impl Eval for Stmt {
    fn eval(&self) -> Value {
        match self {
            Stmt::Expr(expr) => expr.eval(),
            _ => unimplemented!()
        }
    }
}

impl Eval for Expr {
    fn eval(&self) -> Value {
        match self {
            Expr::Lit(Literal::String(s)) => Value::String(s.clone()),
            Expr::Lit(Literal::Integer(number)) => Value::Integer(*number as isize),
            Expr::Lit(Literal::Bool(b)) => Value::Bool(*b),
            Expr::Lit(Literal::Null) => Value::Pointer(0),
            Expr::Unary {
                op: UnaryOperator::Neg,
                right,
            } => match right.eval() {
                Value::Integer(num) => Value::Integer(-num),
                _ => unimplemented!(),
            },
            Expr::Unary {
                op: UnaryOperator::Abs,
                right,
            } => match right.eval() {
                Value::Integer(num) => Value::Integer(num.abs()),
                _ => unimplemented!(),
            },
            Expr::Unary { .. } => unimplemented!(),
            Expr::Binary {
                op,
                left,
                right,
            } => {
                let left = left.eval();
                let right = right.eval();

                match op {
                    BinaryOperator::Add => {
                        match (left, right) {
                            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left + right),
                            _ => unimplemented!(),
                        }
                    }
                    BinaryOperator::Sub => {
                        match (left, right) {
                            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left - right),
                            _ => unimplemented!(),
                        }
                    }
                    BinaryOperator::Mul => {
                        match (left, right) {
                            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left * right),
                            _ => unimplemented!(),
                        }
                    }
                    BinaryOperator::Div => {
                        match (left, right) {
                            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left / right),
                            _ => unimplemented!(),
                        }
                    }
                    BinaryOperator::Pow => {
                        match (left, right) {
                            (Value::Integer(left), Value::Integer(right)) => {
                                Value::Integer(left.pow(right as _))
                            }
                            _ => unimplemented!(),
                        }
                    }
                    BinaryOperator::Eq => {
                        match (left, right) {
                            (Value::Integer(l), Value::Integer(r)) => Value::Bool(l == r),
                            (Value::Bool(l), Value::Bool(r)) => Value::Bool(l == r),
                            (Value::String(l), Value::String(r)) => Value::Bool(l == r),
                            (Value::Pointer(l), Value::Pointer(r)) => Value::Bool(l == r),
                            _ => unimplemented!()
                        }
                    }
                    BinaryOperator::Ge => {
                        match (left, right) {
                            (Value::Integer(l), Value::Integer(r)) => Value::Bool(l >= r),
                            (Value::Pointer(l), Value::Pointer(r)) => Value::Bool(l >= r),
                            _ => unimplemented!()
                        }
                    }
                    BinaryOperator::Gt => {
                        match (left, right) {
                            (Value::Integer(l), Value::Integer(r)) => Value::Bool(l > r),
                            (Value::Pointer(l), Value::Pointer(r)) => Value::Bool(l > r),
                            _ => unimplemented!()
                        }
                    }
                    BinaryOperator::Le => {
                        match (left, right) {
                            (Value::Integer(l), Value::Integer(r)) => Value::Bool(l <= r),
                            (Value::Pointer(l), Value::Pointer(r)) => Value::Bool(l <= r),
                            _ => unimplemented!()
                        }
                    }
                    BinaryOperator::Lt => {
                        match (left, right) {
                            (Value::Integer(l), Value::Integer(r)) => Value::Bool(l < r),
                            (Value::Pointer(l), Value::Pointer(r)) => Value::Bool(l < r),
                            _ => unimplemented!()
                        }
                    }
                    BinaryOperator::Mod => {
                        match (left, right) {
                            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l % r),
                            _ => unimplemented!()
                        }
                    }
                    _ => unimplemented!()
                }
            }
            _ => unimplemented!()
        }
    }
}

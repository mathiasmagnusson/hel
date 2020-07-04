use std::fmt;

use super::{
    AssignmentOperator, BinaryOperator, Expr, Ident, Literal, Path, Stmt, Type, UnaryOperator,
    Value,
};

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Reference(ty) => write!(f, "&{}", ty),
            Type::Path(path) => write!(f, "{}", path),
            Type::Tuple(types) => {
                write!(f, "[")?;
                fmt_slice(f, ", ", types)?;
                write!(f, "]")
            }
            Type::List(ty) => write!(f, "[{}]", ty),
            Type::Function { args, ret } => {
                write!(f, "fn ")?;
                if args.len() != 1 {
                    write!(f, "(")?;
                }
                fmt_slice(f, ", ", args)?;
                if args.len() != 1 {
                    write!(f, ")")?;
                }
                write!(f, " = {}", ret)
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(num) => write!(f, "{}", num),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Reference(ptr) => write!(f, "<ref 0x{:x}>", ptr),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Path(path) => write!(f, "{}", path)?,
            Expr::Lit(lit) => write!(f, "{}", lit)?,
            Expr::Binary { op, left, right } => write!(f, "({} {} {})", left, op, right)?,
            Expr::Unary { op, right } => write!(f, "{}{}", op, right)?,
            Expr::Evoc { func, args } => {
                if matches!(
                    func.as_ref(),
                    Expr::Path(_)
                        | Expr::FieldAccess { .. }
                        | Expr::Evoc { .. }
                        | Expr::Indexing { .. }
                ) {
                    write!(f, "{}", func)?;
                } else {
                    write!(f, "({})", func)?;
                }
                write!(f, "(")?;
                fmt_slice(f, ", ", args)?;
                write!(f, ")")?;
            }
            Expr::Indexing { array, index } => {
                if matches!(
                    array.as_ref(),
                    Expr::Path(_)
                        | Expr::FieldAccess { .. }
                        | Expr::Evoc { .. }
                        | Expr::Indexing { .. }
                ) {
                    write!(f, "{}", array)?;
                } else {
                    write!(f, "({})", array)?;
                }
                write!(f, "[{}]", index)?;
            }
            Expr::FieldAccess { left, field } => {
                if matches!(
                    left.as_ref(),
                    Expr::Path(_)
                        | Expr::FieldAccess { .. }
                        | Expr::Evoc { .. }
                        | Expr::Indexing { .. }
                ) {
                    write!(f, "{}", left)?;
                } else {
                    write!(f, "({})", left)?;
                }
                write!(f, ".{}", field)?;
            }
            Expr::TupleOrArray(items) => {
                write!(f, "@[")?;
                fmt_slice(f, ", ", items)?;
                write!(f, "]")?;
            }
            Expr::StructConstruct { path, vals } => {
                write!(f, "{} @{{ ", path)?;
                for (i, (k, v)) in vals.iter().enumerate() {
                    write!(f, "{}: {}{}", k, v, if i == vals.len() { "" } else { ", " })?;
                }
                write!(f, " }}")?;
            }
            Expr::If { cond, then, els } => {
                write!(f, "if {} {}", cond, then)?;
                if let Some(els) = els {
                    write!(f, " else {}", els)?;
                }
            }
            Expr::Closure { args, body } => {
                write!(f, "fn ")?;
                if args.len() != 1 {
                    write!(f, "(")?;
                }
                fmt_slice(f, ", ", args)?;
                if args.len() != 1 {
                    write!(f, ")")?;
                }
                if !matches!(body.as_ref(), Expr::Block(_)) {
                    write!(f, " =")?;
                }
                write!(f, "{}", body)?;
            }
            Expr::Loop(body) => write!(f, "loop {}", body)?,
            Expr::Block(block) => {
                write!(f, "{{ ")?;
                fmt_slice(f, " ", block)?;
                write!(f, " }}")?;
            }
        };

        Ok(())
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[rustfmt::skip]
        match self {
            Stmt::Expr(expr) => write!(f, "{}", expr)?,
            Stmt::Let {
                ident,
                ty,
                value
            } => {
                write!(f, "let {}", ident)?;
                if let Some(ty) = ty {
                    write!(f, ": {}", ty)?;
                }
                write!(f, " = {}", value)?;
            },
            Stmt::For {
                i,
                iter,
                body,
            } => write!(f, "for {} in {} {}", i, iter, body)?,
            Stmt::Return(expr) => write!(f, "return {}", expr)?,
            Stmt::Assign {
                variable,
                op,
                value,
            } => write!(f, "{} {} {}", variable, op, value)?,
        };

        Ok(())
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_slice(f, "::", &self.0)
    }
}

fn fmt_slice<T: fmt::Display>(f: &mut fmt::Formatter, sep: &str, slice: &[T]) -> fmt::Result {
    for (i, arg) in slice.iter().enumerate() {
        write!(f, "{}{}", arg, if i < slice.len() - 1 { sep } else { "" })?;
    }

    Ok(())
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{:?}", s)?,
            Literal::Integer(n) => write!(f, "{}", n)?,
            Literal::Bool(true) => write!(f, "true")?,
            Literal::Bool(false) => write!(f, "false")?,
            Literal::Null => write!(f, "null")?,
        }

        Ok(())
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[rustfmt::skip]
        write!(f, "{}", match self {
            UnaryOperator::Ref   => '&',
            UnaryOperator::Deref => '$',
            UnaryOperator::Neg   => '-',
            UnaryOperator::Abs   => '+',
            UnaryOperator::Not   => '!',
        })?;

        Ok(())
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[rustfmt::skip]
        write!(f, "{}", match self {
            BinaryOperator::MethodCall => "|>",
            BinaryOperator::Add        => "+",
            BinaryOperator::Sub        => "-",
            BinaryOperator::Mul        => "*",
            BinaryOperator::Div        => "/",
            BinaryOperator::Mod        => "%",
            BinaryOperator::Pow        => "**",
            BinaryOperator::BitAnd     => "&",
            BinaryOperator::BitOr      => "|",
            BinaryOperator::BitXor     => "^",
            BinaryOperator::And        => "and",
            BinaryOperator::Or         => "or",
            BinaryOperator::Eq         => "==",
            BinaryOperator::Neq        => "!=",
            BinaryOperator::Lt         => "<",
            BinaryOperator::Le         => "<=",
            BinaryOperator::Gt         => ">",
            BinaryOperator::Ge         => ">=",
        })?;

        Ok(())
    }
}

impl fmt::Display for AssignmentOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[rustfmt::skip]
        write!(f, "{}", match self {
            AssignmentOperator::Assign => "=",
            AssignmentOperator::Add => "+=",
            AssignmentOperator::Sub => "-=",
            AssignmentOperator::Mul => "*=",
            AssignmentOperator::Div => "/=",
            AssignmentOperator::Mod => "%=",
            AssignmentOperator::Pow => "**=",
            // AssignmentOperator::BitAnd => "&=",
            // AssignmentOperator::BitOr => "|=",
            // AssignmentOperator::BitXor => "^=",
        })?;

        Ok(())
    }
}

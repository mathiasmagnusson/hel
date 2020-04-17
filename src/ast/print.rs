use std::fmt;

use super::{AssignmentOperator, BinaryOperator, Expr, Ident, Literal, Stmt, UnaryOperator, Value};

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(num) => write!(f, "{}", num)?,
            Value::String(s) => write!(f, "{}", s)?,
            Value::Bool(b) => write!(f, "{}", b)?,
            Value::Pointer(ptr) => write!(f, "<ptr 0x{:x}>", ptr)?,
            _ => unimplemented!()
        };

        Ok(())
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[rustfmt::skip]
        match self {
            Expr::Ident(ident) => write!(f, "{}", ident)?,
            Expr::Lit(lit) => write!(f, "{}", lit)?,
            Expr::Binary {
                op,
                left,
                right,
            } => write!(f, "({} {} {})", left, op, right)?,
            Expr::Unary {
                op,
                right,
            } => write!(f, "{}{}", op, right)?,
            Expr::Evoc {
                func,
                args,
            } => {
                if let Expr::Ident(ident) = func.as_ref() {
                    write!(f, "{}", ident)?;
                } else {
                    write!(f, "({})", func)?;
                }
                write!(f, "(")?;
                fmt_slice(f, ", ", args)?;
                write!(f, ")")?;
            },
            Expr::Tuple(items) => {
                fmt_slice(f, ", ", items)?;
            },
            Expr::Array(items) => {
                write!(f, "[")?;
                fmt_slice(f, ", ", items)?;
                write!(f, "]")?;
            },
            Expr::Indexing {
                array,
                index,
            } => {
                if let Expr::Ident(ident) = array.as_ref() {
                    write!(f, "{}", ident)?;
                } else {
                    write!(f, "({})", array)?;
                }
                write!(f, "[{}]", index)?;
            },
            Expr::Struct {
                ident,
                vals,
            } => {
                write!(f, "{} {{ ", ident)?;
                for (i, (k, v)) in vals.iter().enumerate() {
                    write!(f, "{}: {}{}", k.name, v, if i == vals.len() { "" } else { ", "})?;
                }
                write!(f, " }}")?;
            },
            Expr::If {
                cond,
                then,
                els,
            } => {
                write!(f, "if {} {}", cond, then)?;
                if let Some(els) = els {
                    write!(f, " else {}", els)?;
                }
            },
            Expr::Loop(body) => write!(f, "loop {}", body)?,
            Expr::Block(block) => {
                write!(f, "{{ ")?;
                fmt_slice(f, " ", block)?;
                write!(f, " }}")?;
            },
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
            Stmt::Print(expr) => write!(f, "print {}", expr)?,
            Stmt::Assign {
                ident,
                op,
                value,
            } => write!(f, "{} {} {}", ident, op, value)?,
        };

        Ok(())
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
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
            Literal::This => write!(f, "this")?,
        }

        Ok(())
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[rustfmt::skip]
        write!(f, "{}", match self {
            UnaryOperator::Ref   => '&',
            UnaryOperator::Deref => '*',
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
            BinaryOperator::Add    => "+",
            BinaryOperator::Sub    => "-",
            BinaryOperator::Mul    => "*",
            BinaryOperator::Div    => "/",
            BinaryOperator::Mod    => "%",
            BinaryOperator::Pow    => "**",
            BinaryOperator::BitAnd => "&",
            BinaryOperator::BitOr  => "|",
            BinaryOperator::BitXor => "^",
            BinaryOperator::And    => "and",
            BinaryOperator::Or     => "or",
            BinaryOperator::Eq     => "==",
            BinaryOperator::Neq    => "!=",
            BinaryOperator::Lt     => "<",
            BinaryOperator::Le     => "<=",
            BinaryOperator::Gt     => ">",
            BinaryOperator::Ge     => ">=",
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
            AssignmentOperator::BitAnd => "&=",
            AssignmentOperator::BitOr => "|=",
            AssignmentOperator::BitXor => "^=",
        })?;

        Ok(())
    }
}

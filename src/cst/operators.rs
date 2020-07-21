#![rustfmt::skip]

use crate::lex::{Token, TokenKind};

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Add, Sub, Mul, Div, Mod, Pow,
    BitAnd, BitOr, BitXor,
    And, Or, Eq, Neq,
    Lt, Le, Gt, Ge,
    Pipe,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Ref, Deref, Neg, Abs, Not,
}

#[derive(Debug, Clone)]
pub enum AssignmentOperator {
    Assign,
    Add, Sub, Mul, Div, Mod, Pow,
    BitAnd, BitOr, BitXor,
}

impl BinaryOperator {
    pub fn new(token: &Token) -> Option<Self> {
        use BinaryOperator::*;
        match token.kind() {
            TokenKind::BarGt            => Some(Pipe),
            TokenKind::Plus             => Some(Add),
            TokenKind::Minus            => Some(Sub),
            TokenKind::Percent          => Some(Mod),
            TokenKind::Amp              => Some(BitAnd),
            TokenKind::Caret            => Some(BitXor),
            TokenKind::Bar              => Some(BitOr),
            TokenKind::Asterisk         => Some(Mul),
            TokenKind::AsteriskAsterisk => Some(Pow),
            TokenKind::BangEq           => Some(Neq),
            TokenKind::EqualEqual       => Some(Eq),
            TokenKind::Greater          => Some(Gt),
            TokenKind::GreaterEqual     => Some(Ge),
            TokenKind::Less             => Some(Lt),
            TokenKind::LessEqual        => Some(Le),
            TokenKind::Slash            => Some(Div),
            TokenKind::And              => Some(And),
            TokenKind::Or               => Some(Or),
            _ => None,
        }
    }
    pub fn precedence(&self) -> usize {
        use BinaryOperator::*;
        match self {
            Pipe              => 15,
            Pow               => 14,
            BitAnd            => 12,
            BitXor            => 11,
            BitOr             => 10,
            Mod               => 9,
            Mul | Div         => 8,
            Add | Sub         => 6,
            Lt | Le | Gt | Ge => 5,
            Neq | Eq          => 4,
            And               => 3,
            Or                => 2,
        }
    }
    pub fn left_assoc(&self) -> bool {
        true
    }
    pub fn right_assoc(&self) -> bool {
        !self.left_assoc()
    }
}

impl UnaryOperator {
    pub fn new(token: &Token) -> Option<Self> {
        match token.kind() {
            TokenKind::Amp    => Some(Self::Ref),
            TokenKind::Dollar => Some(Self::Deref),
            TokenKind::Minus  => Some(Self::Neg),
            TokenKind::Plus   => Some(Self::Abs),
            TokenKind::Bang   => Some(Self::Not),
            _ => None,
        }
    }
    pub fn precedence(&self) -> usize {
        use UnaryOperator::*;
        match self {
            Ref | Deref => 14,
            Neg | Abs   => 13,
            Not         => 14,
        }
    }
}

impl AssignmentOperator {
    pub fn new(token: &Token) -> Option<Self> {
        use AssignmentOperator::*;
        match token.kind() {
            TokenKind::Equal              => Some(Assign),
            TokenKind::PlusEq             => Some(Add),
            TokenKind::MinusEq            => Some(Sub),
            TokenKind::AsteriskEq         => Some(Mul),
            TokenKind::SlashEq            => Some(Div),
            TokenKind::PercentEq          => Some(Mod),
            TokenKind::AsteriskAsteriskEq => Some(Pow),
            TokenKind::AmpEq              => Some(BitAnd),
            TokenKind::BarEq              => Some(BitOr),
            TokenKind::CaretEq            => Some(BitXor),
            _ => None,
        }
    }
}

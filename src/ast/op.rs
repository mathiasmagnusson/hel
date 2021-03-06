use crate::lex::{Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    MethodCall,
    Add, Sub, Mul, Div, Mod, Pow,
    BitAnd, BitOr, BitXor,
    And, Or, Eq, Neq,
    Lt, Le, Gt, Ge,
}

impl BinaryOperator {
    pub fn new(token: &Token) -> Option<Self> {
        use BinaryOperator::*;
        match token.ty {
            TokenType::BarGt            => Some(MethodCall),
            TokenType::Plus             => Some(Add),
            TokenType::Minus            => Some(Sub),
            TokenType::Percent          => Some(Mod),
            TokenType::Amp              => Some(BitAnd),
            TokenType::Caret            => Some(BitXor),
            TokenType::Bar              => Some(BitOr),
            TokenType::Asterisk         => Some(Mul),
            TokenType::AsteriskAsterisk => Some(Pow),
            TokenType::BangEq           => Some(Neq),
            TokenType::EqualEqual       => Some(Eq),
            TokenType::Greater          => Some(Gt),
            TokenType::GreaterEqual     => Some(Ge),
            TokenType::Less             => Some(Lt),
            TokenType::LessEqual        => Some(Le),
            TokenType::Slash            => Some(Div),
            TokenType::And              => Some(And),
            TokenType::Or               => Some(Or),
            _ => None,
        }
    }
    pub fn precedence(&self) -> usize {
        use BinaryOperator::*;
        match self {
            MethodCall        => 15,
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

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Ref, Deref, Neg, Abs, Not,
}

impl UnaryOperator {
    pub fn new(token: &Token) -> Option<Self> {
        match token.ty {
            TokenType::Amp    => Some(Self::Ref),
            TokenType::Dollar => Some(Self::Deref),
            TokenType::Minus  => Some(Self::Neg),
            TokenType::Plus   => Some(Self::Abs),
            TokenType::Bang   => Some(Self::Not),
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

#[derive(Debug, Clone)]
pub enum AssignmentOperator {
    Assign,
    Add, Sub, Mul, Div, Mod, Pow,
}

impl AssignmentOperator {
    pub fn new(token: &Token) -> Option<Self> {
        use AssignmentOperator::*;
        match token.ty {
            TokenType::Equal              => Some(Assign),
            TokenType::PlusEq             => Some(Add),
            TokenType::MinusEq            => Some(Sub),
            TokenType::AsteriskEq         => Some(Mul),
            TokenType::SlashEq            => Some(Div),
            TokenType::PercentEq          => Some(Mod),
            TokenType::AsteriskAsteriskEq => Some(Pow),
            // TokenType::AmpEq              => Some(BitAnd),
            // TokenType::BarEq              => Some(BitOr),
            // TokenType::CaretEq            => Some(BitXor),
            _ => None,
        }
    }
}

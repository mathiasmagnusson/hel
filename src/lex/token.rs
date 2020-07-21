use derive_getters::Getters;

use crate::text::TextSpan;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // One char
    LeftParen, RightParen,
    LeftCurly, RightCurly,
    LeftSquare, RightSquare,
    Comma,
    Quest,
    At,
    Dollar,
    Colon,

    // One char, and optional =
    Plus, PlusEq,
    Minus, MinusEq,
    Percent, PercentEq,
    Slash, SlashEq,
    Asterisk, AsteriskEq,
    Bang, BangEq,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    Amp, AmpEq,
    Bar, BarEq,
    Caret, CaretEq,

    // Two char
    RightArrow,
    BarGt,
    ColonColon,
    Dot, DotDot,

    // Two char, and optional =
    AsteriskAsterisk, AsteriskAsteriskEq,

    // Keywords
    Let, Null,
    And, Or, True, False,
    Function, Type, Struct, Import,
    If, Then, Else, For, In, Loop, Return, Defer,
    Copy,

    // Special
    Whitespace, Comment, BadCharacter, EOF,
    Ident(String), String(String), Integer(usize), Float(f64),
}

#[derive(Debug, Clone, Getters)]
pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

impl Token {
    pub const fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self {
            kind,
            span
        }
    }
}

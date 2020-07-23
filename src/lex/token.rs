use derive_getters::Getters;
use std::mem;

use crate::text::TextSpan;

#[derive(Debug, Clone, Getters)]
pub struct Token {
    kind: TokenKind,
    span: TextSpan,
    whitespace_before: bool,
    whitespace_after: bool,
}

impl Token {
    pub const fn new(
        kind: TokenKind,
        span: TextSpan,
        whitespace_before: bool,
        whitespace_after: bool,
    ) -> Self {
        Self {
            kind,
            span,
            whitespace_before,
            whitespace_after,
        }
    }

    /// Gives ownership of the `TokenKind`, replacing it by `TokenKind::Taken`
    pub fn take_kind(&mut self) -> TokenKind {
        mem::replace(&mut self.kind, TokenKind::Taken)
    }
}

impl Into<TextSpan> for &Token {
    fn into(self) -> TextSpan {
        self.span
    }
}

#[rustfmt::skip]
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

    // Special
    EOF, Taken,
    Ident(String), String(String), Integer(usize), Float(f64),
}

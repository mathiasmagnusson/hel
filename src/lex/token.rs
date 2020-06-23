use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    LeftParen, RightParen,
    LeftCurly, RightCurly,
    LeftSquare, RightSquare,
    Comma, Dot,
    Colon, Semicolon,
    Quest,

    Plus, Minus, Percent,
    Amp, Dollar, Bar, Caret, BAmp,
    Asterisk, AsteriskAsterisk,
    Bang, BangEq,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    PlusEq, MinusEq, PercentEq,
    BAmpEq, BarEq, CaretEq,
    AsteriskEq, AsteriskAsteriskEq, SlashEq,

    Slash,

    Ident(String), String(String), Integer(usize),

    Let, Null,
    And, Or, True, False,
    Function, Struct, Impl,
    If, Then, Else, For, In, Loop, Return,
    Alloc, Free, Defer, Import,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            ty,
            lexeme: lexeme.into(),
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} token {} at line {}",
            self.ty, self.lexeme, self.line,
        )?;

        Ok(())
    }
}

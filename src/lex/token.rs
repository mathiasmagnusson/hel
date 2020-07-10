use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    LeftParen, RightParen,
    LeftCurly, RightCurly,
    LeftSquare, RightSquare,
    Comma, Dot,
    Quest,
    At, Amp,

    Plus, Minus, Percent, Slash,
    Dollar, Bar, Caret,
    Asterisk, AsteriskAsterisk,
    Bang, BangEq,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    PlusEq, MinusEq, PercentEq,
    BarEq, CaretEq,
    AsteriskEq, AsteriskAsteriskEq, SlashEq,
    RightArrow, BarGt,
    Colon, ColonColon,

    Ident(String), String(String), Integer(usize),

    Let, Null,
    And, Or, True, False,
    Function, Type, Struct, Import,
    If, Then, Else, For, In, Loop, Return, Defer,
    Copy,

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

use super::{Token, TokenKind};
use crate::text::TextSpan;

#[derive(Clone, Copy)]
pub struct TokenStream<'t> {
    tokens: &'t [Token],
    i: usize,
}

impl TokenStream<'_> {
    fn get(&self, i: usize) -> &Token {
        self
            .tokens
            .get(i)
            .or(self.tokens.last())
            .expect("Empty tokenstream")
    }
    pub fn eat(&mut self) -> &Token {
        self.i += 1;
        self.get(self.i - 1)
    }
    pub fn uneat(&mut self) {
        assert_ne!(self.i, 0);
        self.i -= 1;
    }
    pub fn peek(&mut self) -> &Token {
        self.get(self.i)
    }
    pub fn peek_n(&mut self, n: usize) -> &Token {
        self.get(self.i + n)
    }
}

impl<'t> From<&'t [Token]> for TokenStream<'t> {
    fn from(tokens: &'t [Token]) -> Self {
        Self {
            tokens,
            i: 0,
        }
    }
}

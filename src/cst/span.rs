use super::*;

impl Ident {
    pub fn span(&self) -> TextSpan {
        self.1
    }
}

impl Path {
    pub fn span(&self) -> TextSpan {
        TextSpan::new(self[0].span().start(), self[self.len() - 1].span().end())
    }
}

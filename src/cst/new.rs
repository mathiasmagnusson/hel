use super::*;

impl Ident {
    pub fn new(string: String, span: TextSpan) -> Self {
        Self(string, span)
    }
}

impl Path {
    pub fn new(idents: Vec<Ident>) -> Self {
        Self(idents)
    }
}

use super::Ident;

pub struct File {
    items: Vec<Item>
}

pub enum Item {
    Function(Function)
}

pub struct Function {
    ident: Ident,
    // args: Vec<
}

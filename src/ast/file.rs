use super::Ident;

pub struct File {
    // TODO: pub is temp
    pub items: Vec<Item>
}

pub enum Item {
    Function(Function)
}

pub struct Function {
    ident: Ident,
    // args: Vec<
}

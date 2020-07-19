const POINTER_SIZE: u8 = 8; // std::mem::size_of::<usize>() as u8;

#[derive(Debug, PartialEq)]
pub enum Type {
    Never,
    Named(usize), // reference to named type, to allow recursive types
    Function(Function),
    Struct(Struct),
    Tuple(Vec<Type>), // TODO: just use struct with names '0', '1', ...?
    Reference(Box<Type>),
    List(Box<Type>),
    Integer { size: u8, signed: bool },
    // TODO: add Char? or maybe just use Integer { size: 4, signed: false } (u32)
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub parameters: Vec<Type>,
    pub return_type: Box<Type>,
    // TODO: some context, like captures etc
}

#[derive(Debug, PartialEq)]
pub struct Struct {
    pub fields: Vec<(String, Type)>,
}

impl Type {
    fn size(&self) -> u8 {
        match self {
            Type::Never => 0,
            Type::Named(n) => unimplemented!(),
            Type::Function(_) => POINTER_SIZE, // *2 : ptr, ptr to context ?
            Type::Struct(_) => unimplemented!(),
            Type::Tuple(_) => unimplemented!(),
            Type::Reference(box Type::List(_)) => POINTER_SIZE * 2, // ptr, size
            Type::Reference(_) => POINTER_SIZE,
            Type::List(_) => POINTER_SIZE * 3, // ptr, size, capacity
            Type::Integer { size, .. } => *size,
        }
    }
}

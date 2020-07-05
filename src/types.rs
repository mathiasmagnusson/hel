const POINTER_SIZE: usize = 8;

#[derive(Debug)]
pub enum Type {
    Function(Function),
    Struct(Struct),
    Tuple(Tuple), // TODO: just use struct with names '0', '1', ...?
    Reference(Box<Type>),
    List(Box<Type>),
    Integer, // TODO: replace with Primitive
}

#[derive(Debug)]
pub struct Function {
    parameters: Vec<Type>,
    return_type: Box<Type>,
}

#[derive(Debug)]
pub struct Struct {
    size: usize,
    fields: Vec<(String, Type)>,
}

#[derive(Debug)]
pub struct Tuple {
    size: usize,
    fields: Vec<Type>,
}

impl Type {
    fn size(&self) -> usize {
        match self {
            Type::Function(_) => POINTER_SIZE, // *2 : ptr, ptr to context ?
            Type::Struct(Struct { size, .. }) => *size,
            Type::Tuple(Tuple { size, .. }) => *size,
            Type::Reference(box Type::List(_)) => POINTER_SIZE * 2, // ptr, size
            Type::Reference(_) => POINTER_SIZE,
            Type::List(_) => POINTER_SIZE * 3, // ptr, size, capacity
            Type::Integer => POINTER_SIZE,
        }
    }
}

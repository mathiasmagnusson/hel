use crate::ast::Path;
use crate::File;

const POINTER_SIZE: usize = 8;

#[derive(Debug)]
pub struct Type {
    full_path: Path,
    type_type: TypeType,
}

impl Type {
    fn size(&self) -> usize {
        match self.type_type {
            TypeType::Function { .. } => POINTER_SIZE, // *2 : pointer, pointer to captures ?
            TypeType::Struct { size, .. } => size,
            TypeType::Tuple { size, .. } => size,
            TypeType::Reference(box Type {
                type_type: TypeType::List(_),
                ..
            }) => POINTER_SIZE * 2, // pointer, size
            TypeType::Reference(_) => POINTER_SIZE,
            TypeType::List(_) => POINTER_SIZE * 3, // pointer, size, capacity
            TypeType::Integer => POINTER_SIZE,
        }
    }
}

#[derive(Debug)]
pub enum TypeType {
    Function {
        parameters: Vec<Type>,
    },
    Struct {
        size: usize,
        fields: Vec<(String, Type)>,
    },
    Tuple {
        size: usize,
        fields: Vec<Type>,
    },
    Reference(Box<Type>),
    List(Box<Type>),
    Integer, // TODO: replace with Primitive
}

pub fn get_types(file: &File) -> Vec<Type> {
    vec![]
}

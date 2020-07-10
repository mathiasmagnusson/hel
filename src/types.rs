use std::collections::HashMap;

use crate::ast::Path;
use crate::package::{Package, Symbol, SymbolInner};

const POINTER_SIZE: u8 = 8; // std::mem::size_of::<usize>() as u8;

pub struct NamedTypes {
    types: Vec<Type>,
    paths: HashMap<Path, usize>,
}

#[derive(Debug)]
pub enum Type {
    Function(Function),
    Struct(Struct),
    Tuple(Tuple), // TODO: just use struct with names '0', '1', ...?
    Reference(Box<Type>),
    List(Box<Type>),
    Integer { size: u8, signed: bool },
    // TODO: add Char? or maybe just use Integer { size: 4, signed: false } (u32)
}

#[derive(Debug)]
pub struct Function {
    parameters: Vec<Type>,
    return_type: Box<Type>,
    // TODO: some context, like captures etc
}

#[derive(Debug)]
pub struct Struct {
    size: u8,
    fields: Vec<(String, Type)>,
}

#[derive(Debug)]
pub struct Tuple {
    size: u8,
    fields: Vec<Type>,
}

impl Type {
    fn size(&self) -> u8 {
        match self {
            Type::Function(_) => POINTER_SIZE, // *2 : ptr, ptr to context ?
            Type::Struct(Struct { size, .. }) => *size,
            Type::Tuple(Tuple { size, .. }) => *size,
            Type::Reference(box Type::List(_)) => POINTER_SIZE * 2, // ptr, size
            Type::Reference(_) => POINTER_SIZE,
            Type::List(_) => POINTER_SIZE * 3, // ptr, size, capacity
            Type::Integer { size, .. } => *size,
        }
    }
}

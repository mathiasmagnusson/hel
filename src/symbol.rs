use crate::ast::{File, Function, Global, Path, Struct};

pub struct Symbol {
    path: Path,
    ty: SymbolType,
}

pub enum SymbolType {
    Struct(Struct),
    Global(Global),
    Function(Function),
}

impl Symbol {
    pub fn generate(file: &File) -> Vec<Symbol> {
        let mut symbols =
            Vec::with_capacity(file.structs.len() + file.functions.len() + file.globals.len());

        for struc in &file.structs {
            symbols.push(Symbol {
                path: Path(vec![struc.ident.clone()]),
                ty: SymbolType::Struct(struc.clone()),
            })
        }

        for func in &file.functions {
            symbols.push(Symbol {
                path: Path(vec![func.ident.clone()]),
                ty: SymbolType::Function(func.clone()),
            })
        }

        for global in &file.globals {
            symbols.push(Symbol {
                path: Path(vec![global.ident.clone()]),
                ty: SymbolType::Global(global.clone()),
            })
        }

        // imports

        symbols
    }
}

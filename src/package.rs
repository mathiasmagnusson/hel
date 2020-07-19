use std::borrow::Cow;
use std::collections::{HashSet, VecDeque};
use std::{fs, iter, path};

use crate::ast::{Function, Global, Ident, Path, Struct, Type, TypeDecl};
use crate::types;
use crate::{File, Lexer, Parse, TokenStream};

#[derive(Debug)]
pub struct Package {
    pub symbols: Vec<Symbol>,
    pub named_types: Vec<types::Type>,
}

#[derive(Debug)]
pub struct Symbol {
    pub location: Path,
    pub imports: Vec<Path>,
    pub inner: SymbolInner,
}

#[derive(Debug)]
pub enum SymbolInner {
    Function(Function),
    TypeDecl(TypeDecl, usize),
    Struct(Struct, usize),
    Global(Global),
}

impl Symbol {
    pub fn get_path(&self) -> impl Iterator<Item = &Ident> {
        self.location.0.iter().chain(iter::once(match &self.inner {
            SymbolInner::Function(Function { ident, .. }) => ident,
            SymbolInner::TypeDecl(TypeDecl { ident, .. }, _) => ident,
            SymbolInner::Struct(Struct { ident, .. }, _) => ident,
            SymbolInner::Global(Global { ident, .. }) => ident,
        }))
    }
}

impl Package {
    pub fn new<AsPath: AsRef<path::Path>>(target: &AsPath) -> Result<Self, anyhow::Error> {
        let target = target.as_ref();

        let mut ret = Self {
            symbols: vec![],
            named_types: vec![],
        };

        for &size in &[1, 2, 4, 8] {
            for &signed in &[false, true] {
                ret.symbols.push(Symbol {
                    location: Path(vec![Ident("hel".into())]),
                    imports: vec![],
                    inner: SymbolInner::TypeDecl(
                        TypeDecl {
                            ident: Ident(format!("{}{}", if signed { 's' } else { 'u' }, size * 8)),
                            ty: Type::Path(Path(vec![])),
                        },
                        ret.symbols.len(),
                    ),
                });
                ret.named_types.push(types::Type::Integer { size, signed })
            }
        }

        // TODO: check for project conf file and read that first if present

        let main_file_path = if target.is_dir() {
            Cow::Owned(target.join("main.hel"))
        } else {
            Cow::Borrowed(target)
        };

        let mut files: VecDeque<(Path, Cow<path::Path>)> = VecDeque::new();
        let mut been_at = HashSet::new();
        files.push_back((
            Path(vec![Ident(
                main_file_path
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            )]),
            main_file_path,
        ));

        while let Some((hel_path, fs_path)) = files.pop_front() {
            if been_at.contains(&fs_path) {
                continue;
            }
            been_at.insert(fs_path.clone());

            let tokens = Lexer::new(&fs::read_to_string(&fs_path)?).tokenize()?;
            let file = File::parse(TokenStream::from(tokens.as_ref()))?.1;

            for module in &file.imports {
                let head = &module.path.0[0].0;

                // TODO: check if it's an external library or stuff like 'super',
                // 'package', etc, a known module, or a directory

                let new_fs_path = fs_path.parent().unwrap().join(format!("{}.hel", head));
                if new_fs_path.exists() && new_fs_path.is_file() {
                    let mut new_path = hel_path.0.clone();
                    *new_path.last_mut().unwrap() = Ident(head.clone());
                    files.push_back((Path(new_path), Cow::Owned(new_fs_path)));
                } else {
                    return Err(anyhow::anyhow!(
                        "module {} not found (searched at {:?}, wich didn't exist) in {:?}",
                        &head,
                        new_fs_path,
                        fs_path
                    ));
                }
            }

            for func in &file.functions {
                ret.symbols.push(Symbol {
                    location: hel_path.clone(),
                    imports: file.imports.iter().map(|imp| imp.path.clone()).collect(),
                    inner: SymbolInner::Function(func.clone()),
                });
            }

            for type_decl in &file.type_decls {
                ret.symbols.push(Symbol {
                    location: hel_path.clone(),
                    imports: file.imports.iter().map(|imp| imp.path.clone()).collect(),
                    inner: SymbolInner::TypeDecl(type_decl.clone(), ret.named_types.len()),
                });
                ret.named_types.push(types::Type::Never);
            }

            for struc in &file.structs {
                ret.symbols.push(Symbol {
                    location: hel_path.clone(),
                    imports: file.imports.iter().map(|imp| imp.path.clone()).collect(),
                    inner: SymbolInner::Struct(struc.clone(), ret.named_types.len()),
                });
                ret.named_types.push(types::Type::Never);
            }

            for global in &file.globals {
                ret.symbols.push(Symbol {
                    location: hel_path.clone(),
                    imports: file.imports.iter().map(|imp| imp.path.clone()).collect(),
                    inner: SymbolInner::Global(global.clone()),
                });
            }
        }

        for symbol in &ret.symbols {
            match &symbol.inner {
                SymbolInner::TypeDecl(type_decl, id) => {
                    if ret.named_types[*id] != types::Type::Never {
                        continue;
                    }
                    ret.named_types[*id] = ret.get_type(&type_decl.ty, symbol)?;
                }
                SymbolInner::Struct(struc, id) => {
                    if ret.named_types[*id] != types::Type::Never {
                        continue;
                    }
                    let mut fields = Vec::with_capacity(struc.fields.len());
                    for field in &struc.fields {
                        fields.push((field.name.0.clone(), ret.get_type(&field.ty, symbol)?));
                    }
                    ret.named_types[*id] = types::Type::Struct(types::Struct { fields });
                }
                _ => {}
            }
        }

        Ok(ret)
    }

    pub fn get_symbol(&self, path: &Path, from: &Symbol) -> Option<&Symbol> {
        for symbol in &self.symbols {
            if symbol
                .get_path()
                .eq(&mut from.location.0.iter().chain(&path.0))
            {
                return Some(symbol);
            }
            for imp in &from.imports {
                if path.0[0] == imp.0[imp.0.len() - 1]
                    && symbol
                        .get_path()
                        .eq(&mut from.location.0[0..from.location.0.len() - 1]
                            .iter()
                            .chain(imp.0.iter())
                            .chain(path.0.iter().skip(1)))
                {
                    return Some(symbol);
                }
            }
        }

        None
    }

    pub fn get_type(&self, ty: &Type, from: &Symbol) -> anyhow::Result<types::Type> {
        match ty {
            Type::Path(path) => match self.get_symbol(path, from) {
                Some(Symbol {
                    inner: SymbolInner::TypeDecl(_, id),
                    ..
                }) => Ok(types::Type::Named(*id)),
                Some(Symbol {
                    inner: SymbolInner::Struct(_, id),
                    ..
                }) => Ok(types::Type::Named(*id)),
                Some(_) => Err(anyhow::anyhow!(
                    "Symbol {} is not a type (at {})",
                    path,
                    Path(from.get_path().cloned().collect::<Vec<_>>())
                )),
                None => Err(anyhow::anyhow!(
                    "Unresolvable symbol {} in {}",
                    path,
                    Path(from.get_path().cloned().collect::<Vec<_>>())
                )),
            },
            Type::Reference(inner) => Ok(types::Type::Reference(box self.get_type(inner, from)?)),
            Type::Tuple(inner) => {
                let mut types = Vec::with_capacity(inner.len());
                for ty in inner {
                    types.push(self.get_type(ty, from)?);
                }
                Ok(types::Type::Tuple(types))
            }
            Type::List(inner) => Ok(types::Type::List(box self.get_type(inner, from)?)),
            Type::Function { args, ret } => {
                let mut parameters = Vec::with_capacity(args.len());
                for ty in args {
                    parameters.push(self.get_type(ty, from)?);
                }
                Ok(types::Type::Function(types::Function {
                    parameters,
                    return_type: box self.get_type(ret, from)?,
                }))
            }
        }
    }
}

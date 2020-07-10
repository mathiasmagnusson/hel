use std::borrow::Cow;
use std::collections::{HashSet, VecDeque};
use std::{fs, iter, path};

use crate::ast::{self, Function, Global, Ident, Path, Struct, TypeDecl};
use crate::{types::Type, File, Lexer, Parse, TokenStream};

#[derive(Debug)]
pub struct Package {
    pub symbols: Vec<Symbol>,
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
    TypeDecl(TypeDecl),
    Struct(Struct),
    Global(Global),
}

impl Symbol {
    pub fn get_path(&self) -> impl Iterator<Item = &Ident> {
        self.location.0.iter().chain(iter::once(match &self.inner {
            SymbolInner::Function(Function { ident, .. }) => ident,
            SymbolInner::TypeDecl(TypeDecl { ident, .. }) => ident,
            SymbolInner::Struct(Struct { ident, .. }) => ident,
            SymbolInner::Global(Global { ident, .. }) => ident,
        }))
    }
}

impl Package {
    pub fn new<P: AsRef<path::Path>>(target: &P) -> Result<Self, anyhow::Error> {
        let target = target.as_ref();

        let mut ret = Self { symbols: vec![] };

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
                    inner: SymbolInner::TypeDecl(type_decl.clone()),
                })
            }

            for struc in &file.structs {
                ret.symbols.push(Symbol {
                    location: hel_path.clone(),
                    imports: file.imports.iter().map(|imp| imp.path.clone()).collect(),
                    inner: SymbolInner::Struct(struc.clone()),
                });
            }

            for global in &file.globals {
                ret.symbols.push(Symbol {
                    location: hel_path.clone(),
                    imports: file.imports.iter().map(|imp| imp.path.clone()).collect(),
                    inner: SymbolInner::Global(global.clone()),
                });
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

    pub fn get_type(&self, ty: &ast::Type, from: &Symbol) -> Option<Type> {
        match ty {
            ast::Type::Path(path) => {
                unimplemented!()
            }
            _ => unimplemented!()
        }
    }
}

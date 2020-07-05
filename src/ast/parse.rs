use std::usize;

use super::{
    Argument, AssignmentOperator, BinaryOperator, Error, Expr, Field, File, Function, Ident,
    Import, Literal, Path, Stmt, Struct, TokenStream, Type, UnaryOperator, Global
};

use crate::lex::TokenType;
pub trait Parse: Sized {
    fn parse(tokens: TokenStream) -> Result<(TokenStream, Self), Error> {
        Self::parse_impl(tokens, 0)
    }
    fn parse_prec_lvl(tokens: TokenStream, prec_lvl: usize) -> Result<(TokenStream, Self), Error> {
        Self::parse_impl(tokens, prec_lvl)
    }
    fn parse_impl(tokens: TokenStream, prec_lvl: usize) -> Result<(TokenStream, Self), Error>;
}

impl Parse for Path {
    fn parse_impl(mut tokens: TokenStream, _prec_lvl: usize) -> Result<(TokenStream, Self), Error> {
        let first = tokens.eat();
        let first = match &first.ty {
            TokenType::Ident(ident) => Ident(ident.clone()),
            _ => return Err(Error::new(first.clone(), "Expected identifier".into())),
        };

        let mut idents = vec![first];
        while let TokenType::ColonColon = &tokens.peek().ty {
            tokens.eat(); // ::
            let next = tokens.eat();
            let next = match &next.ty {
                TokenType::Ident(ident) => Ident(ident.clone()),
                _ => return Err(Error::new(next.clone(), "Expected identifier".into())),
            };
            idents.push(next);
        }

        Ok((tokens, Path(idents)))
    }
}

impl Parse for File {
    fn parse_impl(mut tokens: TokenStream, _prec_lvl: usize) -> Result<(TokenStream, Self), Error> {
        let mut file = File {
            imports: vec![],
            structs: vec![],
            functions: vec![],
            globals: vec![],
        };

        loop {
            let token = tokens.eat();

            match &token.ty {
                TokenType::Let => {
                    let ident = tokens.eat();
                    let ident = if let TokenType::Ident(ident) = &ident.ty {
                        Ident(ident.clone())
                    } else {
                        return Err(Error::new(ident.clone(), "Expected identifier".into()));
                    };

                    let colon = tokens.eat();
                    if colon.ty != TokenType::Colon {
                        return Err(Error::new(colon.clone(), "Expected colon".into()));
                    }
                    let (new_tokens, ty) = Type::parse(tokens)?;
                    tokens = new_tokens;

                    let equals = tokens.eat();
                    if equals.ty != TokenType::Equal {
                        return Err(Error::new(equals.clone(), "Expected equals sign".into()));
                    }
                    let (new_tokens, value) = Expr::parse(tokens)?;
                    tokens = new_tokens;

                    file.globals.push(Global {
                        ident,
                        ty,
                        value,
                    })
                }
                TokenType::Import => {
                    let (new_tokens, path) = Path::parse(tokens)?;
                    tokens = new_tokens;

                    file.imports.push(Import {
                        path,
                    });
                }
                TokenType::Function => {
                    let ident = tokens.eat();
                    let ident = if let TokenType::Ident(ident) = &ident.ty {
                        Ident(ident.clone())
                    } else {
                        return Err(Error::new(ident.clone(), "Expected identifier".into()));
                    };

                    let left_paren = tokens.eat();
                    if left_paren.ty != TokenType::LeftParen {
                        return Err(Error::new(
                            left_paren.clone(),
                            "Expected opening parenthesis".into(),
                        ));
                    }

                    let mut args = vec![];
                    while tokens.peek().ty != TokenType::RightParen {
                        let arg_name = tokens.eat();
                        let arg_name = if let TokenType::Ident(arg_name) = &arg_name.ty {
                            Ident(arg_name.clone())
                        } else {
                            return Err(Error::new(arg_name.clone(), "Expected identifier".into()));
                        };

                        let colon = tokens.eat();
                        if colon.ty != TokenType::Colon {
                            return Err(Error::new(colon.clone(), "Expected colon".into()));
                        }

                        let (new_tokens, ty) = Type::parse(tokens)?;
                        tokens = new_tokens;

                        args.push(Argument {
                            ident: arg_name,
                            ty,
                        });

                        let next = tokens.peek();
                        match next.ty {
                            TokenType::Comma => {
                                tokens.eat();
                            }
                            TokenType::RightParen => (),
                            _ => {
                                return Err(Error::new(
                                    next.clone(),
                                    "Expected comma or closing parenthesis".into(),
                                ))
                            }
                        }
                    }

                    let right_paren = tokens.eat();
                    if right_paren.ty != TokenType::RightParen {
                        return Err(Error::new(
                            right_paren.clone(),
                            "Expected closing parenthesis".into(),
                        ));
                    }

                    let (new_tokens, return_type) = if tokens.peek().ty == TokenType::RightArrow {
                        tokens.eat();

                        let (tokens, return_type) = Type::parse(tokens)?;

                        (tokens, return_type)
                    } else {
                        (tokens, Type::Tuple(vec![]))
                    };
                    tokens = new_tokens;

                    let has_equal = tokens.peek().ty == TokenType::Equal;
                    if has_equal {
                        tokens.eat();
                    }

                    let (new_tokens, body) = Expr::parse(tokens)?;
                    if has_equal && matches!(body, Expr::Block(_)) {
                        return Err(Error::new(
                            tokens.peek().clone(),
                            "Function bodies starting with '= {' are hella ugly, remove the '='"
                                .into(),
                        ));
                    }
                    tokens = new_tokens; // NOTE: dont move this before the if-statement above

                    file.functions.push(Function {
                        ident,
                        args,
                        return_type,
                        body,
                    })
                }
                TokenType::Struct => {
                    let ident = tokens.eat();
                    let ident = if let TokenType::Ident(ident) = &ident.ty {
                        Ident(ident.clone())
                    } else {
                        return Err(Error::new(ident.clone(), "Expected identifier".into()));
                    };

                    let left_curly = tokens.eat();
                    if left_curly.ty != TokenType::LeftCurly {
                        return Err(Error::new(
                            left_curly.clone(),
                            "Expected opening curly bracket".into(),
                        ));
                    }

                    let mut fields = vec![];

                    while tokens.peek().ty != TokenType::RightCurly {
                        let name = tokens.eat();
                        let name = if let TokenType::Ident(name) = &name.ty {
                            Ident(name.clone())
                        } else {
                            return Err(Error::new(name.clone(), "Expected identifier".into()));
                        };

                        let colon = tokens.eat();
                        if colon.ty != TokenType::Colon {
                            return Err(Error::new(colon.clone(), "Expected colon".into()));
                        }

                        let (new_tokens, ty) = Type::parse(tokens)?;
                        tokens = new_tokens;

                        fields.push(Field { name, ty });

                        let next = tokens.peek();
                        match next.ty {
                            TokenType::Comma => {
                                tokens.eat();
                            }
                            TokenType::RightCurly => {}
                            _ => {
                                return Err(Error::new(
                                    next.clone(),
                                    "Expected comma or right curly bracket".into(),
                                ))
                            }
                        }
                    }
                    tokens.eat();

                    file.structs.push(Struct { ident, fields });
                }
                TokenType::EOF => break,
                _ => {
                    return Err(Error::new(
                        token.clone(),
                        "Expected import, function or struct definition".into(),
                    ))
                }
            };
        }

        Ok((tokens, file))
    }
}

impl Parse for Type {
    fn parse_impl(mut tokens: TokenStream, _prec_lvl: usize) -> Result<(TokenStream, Self), Error> {
        let token = tokens.peek();
        let (tokens, ty) = match &token.ty {
            TokenType::Ident(_) => {
                let (tokens, path) = Path::parse(tokens)?;
                (tokens, Type::Path(path))
            }
            TokenType::Amp => {
                tokens.eat(); // &
                let (tokens, inner) = Type::parse(tokens)?;
                (tokens, Type::Reference(box inner))
            }
            TokenType::LeftSquare => {
                tokens.eat(); // [
                let mut types = vec![];
                while tokens.peek().ty != TokenType::RightSquare {
                    let (new_tokens, ty) = Type::parse(tokens)?;
                    tokens = new_tokens;

                    types.push(ty);

                    let next = tokens.peek();
                    match next.ty {
                        TokenType::Comma => {
                            tokens.eat();
                        }
                        TokenType::RightSquare => (),
                        _ => {
                            return Err(Error::new(
                                next.clone(),
                                "Expected comma or closing square bracket".into(),
                            ))
                        }
                    }
                }
                tokens.eat(); // ]

                if types.len() == 1 {
                    (tokens, Type::List(box types.into_iter().next().unwrap())) // == types[0]
                } else {
                    (tokens, Type::Tuple(types))
                }
            }
            TokenType::Function => {
                tokens.eat(); // fn
                let (mut tokens, args) = if tokens.peek().ty == TokenType::LeftParen {
                    let (tokens, args) = Type::parse(tokens)?;
                    let args = match args {
                        Type::Tuple(args) => args,
                        _ => unreachable!(),
                    };

                    (tokens, args)
                } else {
                    let (tokens, arg) = Type::parse(tokens)?;
                    (tokens, vec![arg])
                };

                let (new_tokens, ret) = if tokens.peek().ty == TokenType::RightArrow {
                    tokens.eat();

                    let (tokens, return_type) = Type::parse(tokens)?;

                    (tokens, box return_type)
                } else {
                    (tokens, box Type::Tuple(vec![]))
                };
                tokens = new_tokens;

                (tokens, Type::Function { args, ret })
            }
            _ => return Err(Error::new(token.clone(), "Expected type".into())),
        };

        Ok((tokens, ty))
    }
}

impl Parse for Expr {
    fn parse_impl(mut tokens: TokenStream, prec_lvl: usize) -> Result<(TokenStream, Self), Error> {
        let token = tokens.eat();
        let (mut tokens, mut expr) = match &token.ty {
            TokenType::True => (tokens, Expr::Lit(Literal::Bool(true))),
            TokenType::False => (tokens, Expr::Lit(Literal::Bool(false))),
            TokenType::Null => (tokens, Expr::Lit(Literal::Null)),
            TokenType::Function => {
                let has_parens = tokens.peek().ty == TokenType::LeftParen;
                if has_parens {
                    tokens.eat(); // (
                }

                let mut args = vec![];
                while tokens.peek().ty != TokenType::RightParen {
                    let arg = tokens.eat();
                    let arg = match &arg.ty {
                        TokenType::Ident(ident) => ident.clone(),
                        _ => return Err(Error::new(arg.clone(), "Expected identifier".into())),
                    };

                    args.push(Ident(arg));

                    if !has_parens {
                        break;
                    }

                    let next = tokens.peek();
                    match next.ty {
                        TokenType::Comma => {
                            tokens.eat();
                        }
                        TokenType::RightParen => (),
                        _ => {
                            return Err(Error::new(
                                next.clone(),
                                "Expected comma or closing parenthesis".into(),
                            ))
                        }
                    }
                }

                if has_parens {
                    let right_paren = tokens.eat();
                    if right_paren.ty != TokenType::RightParen {
                        return Err(Error::new(
                            right_paren.clone(),
                            "Expected closing parenthesis".into(),
                        ));
                    }
                }

                let equal = tokens.eat();
                if equal.ty != TokenType::Equal {
                    return Err(Error::new(equal.clone(), "Expected equal sign (=)".into()));
                }

                let (tokens, body) = Expr::parse(tokens)?;

                (
                    tokens,
                    Expr::Closure {
                        args,
                        body: box body,
                    },
                )
            }
            TokenType::Ident(_) => {
                tokens.uneat();
                let (mut tokens, path) = Path::parse(tokens)?;
                if tokens.peek().ty == TokenType::At && tokens.peek_n(1).ty == TokenType::LeftCurly
                {
                    tokens.eat(); // @
                    tokens.eat(); // {

                    let mut vals = vec![];

                    while tokens.peek().ty != TokenType::RightCurly {
                        let key = tokens.eat();
                        let key = if let TokenType::Ident(key) = &key.ty {
                            key.clone()
                        } else {
                            return Err(Error::new(key.clone(), "Expected identifier".into()));
                        };

                        let colon = tokens.eat();
                        if colon.ty != TokenType::Colon {
                            return Err(Error::new(colon.clone(), "Expected colon".into()));
                        }

                        let (new_tokens, value) = Expr::parse(tokens)?;
                        tokens = new_tokens;

                        vals.push((Ident(key), box value));

                        let next = tokens.peek();
                        match next.ty {
                            TokenType::Comma => {
                                tokens.eat();
                            }
                            TokenType::RightCurly => {}
                            _ => {
                                return Err(Error::new(
                                    next.clone(),
                                    "Expected comma or right curly bracket".into(),
                                ))
                            }
                        }
                    }
                    tokens.eat(); // }

                    (tokens, Expr::StructConstruct { path, vals })
                } else {
                    let expr = Expr::Path(path);
                    (tokens, expr)
                }
            }
            TokenType::String(val) => {
                let expr = Expr::Lit(Literal::String(val.clone()));
                (tokens, expr)
            }
            TokenType::Integer(val) => {
                let expr = Expr::Lit(Literal::Integer(*val));
                (tokens, expr)
            }
            TokenType::LeftParen => {
                let (mut tokens, expr) = Expr::parse(tokens)?;

                let right_paren = tokens.eat();
                if right_paren.ty != TokenType::RightParen {
                    return Err(Error::new(
                        right_paren.clone(),
                        "Expected closing parenthesis".into(),
                    ));
                }

                (tokens, expr)
            }
            TokenType::At => {
                let left_square = tokens.eat();
                if left_square.ty != TokenType::LeftSquare {
                    return Err(Error::new(
                        left_square.clone(),
                        "Expected opening square bracket".into(),
                    ));
                }

                let mut items = vec![];

                while tokens.peek().ty != TokenType::RightSquare {
                    let (new_tokens, expr) = Expr::parse(tokens)?;
                    tokens = new_tokens;
                    items.push(expr);

                    let next = tokens.peek();
                    match next.ty {
                        TokenType::Comma => {
                            tokens.eat();
                        }
                        TokenType::RightSquare => (),
                        _ => {
                            return Err(Error::new(
                                next.clone(),
                                "Expected comma or closing square bracket".into(),
                            ))
                        }
                    }
                }
                let right_square = tokens.eat();
                if right_square.ty != TokenType::RightSquare {
                    return Err(Error::new(
                        right_square.clone(),
                        "Expected closing square bracket".into(),
                    ));
                }

                (tokens, Expr::TupleOrArray(items))
            }
            TokenType::Loop => {
                let (tokens, body) = Expr::parse(tokens)?;

                (tokens, Expr::Loop(box body))
            }
            TokenType::LeftCurly => {
                let mut stmts = vec![];

                loop {
                    if tokens.peek().ty == TokenType::RightCurly {
                        tokens.eat();
                        break;
                    }

                    let (new_tokens, stmt) = Stmt::parse(tokens)?;
                    tokens = new_tokens;
                    stmts.push(stmt);
                }

                (tokens, Expr::Block(stmts))
            }
            TokenType::If => {
                let (mut tokens, cond) = Expr::parse(tokens)?;
                if tokens.peek().ty == TokenType::Then {
                    tokens.eat();
                }
                let (mut tokens, then) = Stmt::parse(tokens)?;
                let (tokens, els) = if tokens.peek().ty == TokenType::Else {
                    tokens.eat();
                    let (tokens, els) = Stmt::parse(tokens)?;
                    (tokens, Some(els))
                } else {
                    (tokens, None)
                };

                (
                    tokens,
                    Expr::If {
                        cond: box cond,
                        then: box then,
                        els: els.map(Box::new),
                    },
                )
            }
            _ => {
                if let Some(unary_op) = UnaryOperator::new(&token) {
                    let (tokens, expr) = Expr::parse_prec_lvl(tokens, unary_op.precedence())?;

                    (
                        tokens,
                        Expr::Unary {
                            op: unary_op,
                            right: box expr,
                        },
                    )
                } else {
                    return Err(Error::new(token.clone(), "Expected expression".into()));
                }
            }
        };

        loop {
            match tokens.peek().ty {
                TokenType::Dot => {
                    tokens.eat();

                    let next = tokens.eat();
                    let field = match &next.ty {
                        TokenType::Ident(ident) => ident.clone(),
                        _ => return Err(Error::new(next.clone(), "Expected identifier".into())),
                    };

                    expr = Expr::FieldAccess {
                        left: box expr,
                        field: Ident(field),
                    }
                }
                TokenType::LeftSquare => {
                    tokens.eat();

                    let (new_tokens, index) = Expr::parse(tokens)?;
                    tokens = new_tokens;

                    let right_square = tokens.eat();
                    if right_square.ty != TokenType::RightSquare {
                        return Err(Error::new(
                            right_square.clone(),
                            "Expected closing square bracket to end indexing".into(),
                        ));
                    }

                    expr = Expr::Indexing {
                        array: box expr,
                        index: box index,
                    };
                }
                TokenType::LeftParen => {
                    tokens.eat();

                    let mut args = vec![];

                    while tokens.peek().ty != TokenType::RightParen {
                        let (new_tokens, arg) = Expr::parse(tokens)?;
                        tokens = new_tokens;
                        args.push(arg);

                        let next = tokens.peek();
                        match next.ty {
                            TokenType::Comma => {
                                tokens.eat();
                            }
                            TokenType::RightParen => {}
                            _ => {
                                return Err(Error::new(
                                    next.clone(),
                                    "Expected comma or right parenthesis".into(),
                                ))
                            }
                        };
                    }
                    let right_paren = tokens.eat();
                    if right_paren.ty != TokenType::RightParen {
                        return Err(Error::new(
                            right_paren.clone(),
                            "Expected right parenthesis".into(),
                        ));
                    }

                    expr = Expr::Evoc {
                        func: box expr,
                        args,
                    }
                }
                _ => break,
            }
        }

        while let Some(bin_op) = BinaryOperator::new(tokens.peek()) {
            let op_prec = bin_op.precedence();
            if op_prec > prec_lvl || op_prec == prec_lvl && bin_op.right_assoc() {
                tokens.eat(); // the binary operator
                let right_side_first_token = tokens.peek().clone();
                let (new_tokens, right_expr) = Expr::parse_prec_lvl(tokens, op_prec)?;
                tokens = new_tokens;

                if bin_op == BinaryOperator::MethodCall {
                    if let Expr::Evoc { func, mut args } = right_expr {
                        args.insert(0, expr);

                        expr = Expr::Evoc { func, args };
                    } else {
                        return Err(Error::new(
                            right_side_first_token,
                            "Right side of |> operator must be a function invocation".into(),
                        ));
                    }
                } else {
                    expr = Expr::Binary {
                        op: bin_op,
                        left: box expr,
                        right: box right_expr,
                    };
                }
            } else {
                break;
            }
        }

        Ok((tokens, expr))
    }
}

impl Parse for Stmt {
    fn parse_impl(mut tokens: TokenStream, _prec_lvl: usize) -> Result<(TokenStream, Self), Error> {
        let token = tokens.peek();
        let (tokens, stmt) = match &token.ty {
            TokenType::Let => {
                tokens.eat();

                let name = tokens.eat();
                let name = if let TokenType::Ident(name) = &name.ty {
                    name.clone()
                } else {
                    return Err(Error::new(name.clone(), "Expected identifier".into()));
                };
                let (mut tokens, ty) = if tokens.peek().ty == TokenType::Colon {
                    tokens.eat();

                    let (tokens, ty) = Type::parse(tokens)?;

                    (tokens, Some(ty))
                } else {
                    (tokens, None)
                };
                let equals = tokens.eat();
                if equals.ty != TokenType::Equal {
                    return Err(Error::new(equals.clone(), "Expected equals sign".into()));
                }
                let (tokens, value) = Expr::parse(tokens)?;

                (
                    tokens,
                    Stmt::Let {
                        ident: Ident(name),
                        ty,
                        value,
                    },
                )
            }
            TokenType::Return => {
                tokens.eat();

                let (tokens, expr) = Expr::parse(tokens)?;

                (tokens, Stmt::Return(expr))
            }
            TokenType::For => {
                tokens.eat();

                let i = tokens.eat();
                let i = if let TokenType::Ident(i) = &i.ty {
                    Ident(i.clone())
                } else {
                    return Err(Error::new(i.clone(), "Expected identifier".into()));
                };

                let in_token = tokens.eat();
                if in_token.ty != TokenType::In {
                    return Err(Error::new(in_token.clone(), "Expected 'in'".into()));
                }

                let (tokens, iter) = Expr::parse(tokens)?;

                let (tokens, body) = Stmt::parse(tokens)?;

                (
                    tokens,
                    Stmt::For {
                        i,
                        iter,
                        body: box body,
                    },
                )
            }
            _ => {
                let (mut tokens, expr) = Expr::parse(tokens)?;

                if let Some(op) = AssignmentOperator::new(tokens.peek()) {
                    tokens.eat(); // op
                    let (tokens, value) = Expr::parse(tokens)?;

                    (tokens, Stmt::Assign { variable: expr, op, value })
                } else {
                    (tokens, Stmt::Expr(expr))
                }
            }
        };

        Ok((tokens, stmt))
    }
}

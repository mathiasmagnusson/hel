use std::usize;

use super::{BinaryOperator, Error, Expr, Ident, Literal, Stmt, TokenStream, UnaryOperator, AssignmentOperator};

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

impl Parse for Expr {
    fn parse_impl(mut tokens: TokenStream, prec_lvl: usize) -> Result<(TokenStream, Self), Error> {
        let token = tokens.eat();
        let (mut tokens, mut expr) = match &token.ty {
            TokenType::True => {
                (tokens, Expr::Lit(Literal::Boolean(true)))
            },
            TokenType::False => {
                (tokens, Expr::Lit(Literal::Boolean(false)))
            },
            TokenType::Null => {
                (tokens, Expr::Lit(Literal::Null))
            },
            TokenType::This => {
                (tokens, Expr::Lit(Literal::This))
            },
            TokenType::Ident(val)  => {
                let ident = Ident { name: val.clone() };
                if tokens.peek().ty == TokenType::LeftCurly {
                    tokens.eat();

                    let mut vals = vec![];

                    while tokens.peek().ty != TokenType::RightCurly {
                        let key = tokens.eat();
                        let key = if let TokenType::Ident(key) = &key.ty {
                            key.clone()
                        } else {
                            return Err(Error::new(
                                key.clone(), 
                                "Expected identifier".into(),
                            ));
                        };

                        let colon = tokens.eat();
                        if colon.ty != TokenType::Colon {
                            return Err(Error::new(
                                colon.clone(),
                                "Expected colon".into(),
                            ));
                        }

                        let (new_tokens, value) = Expr::parse(tokens)?;
                        tokens = new_tokens;

                        vals.push((
                            Ident { name: key },
                            box value
                        ));

                        let next = tokens.peek();
                        match next.ty {
                            TokenType::Comma => {
                                tokens.eat();
                            },
                            TokenType::RightCurly => {},
                            _ => return Err(Error::new(
                                next.clone(),
                                "Expected comma or right curly bracket".into()
                            ))
                        }
                    }

                    tokens.eat();

                    (tokens, Expr::Struct {
                        ident,
                        vals,
                    })
                } else {
                    let expr = Expr::Ident(ident);
                    (tokens, expr)
                }
            },
            TokenType::String(val) => {
                let expr = Expr::Lit(Literal::String(val.clone()));
                (tokens, expr)
            },
            TokenType::Integer(val) => {
                let expr = Expr::Lit(Literal::Integer(*val));
                (tokens, expr)
            },
            TokenType::LeftParen => {
                if tokens.peek().ty == TokenType::RightParen {
                    tokens.eat();

                    (tokens, Expr::Tuple(vec![]))
                } else {
                    let (mut tokens, expr) = Expr::parse(tokens)?;

                    let right_paren_or_comma = tokens.eat();
                    match right_paren_or_comma.ty {
                        TokenType::RightParen => (tokens, expr),
                        TokenType::Comma => {
                            let mut items = vec![expr];
                            while tokens.peek().ty != TokenType::RightParen {
                                let (new_tokens, item) = Expr::parse(tokens)?;
                                tokens = new_tokens;
                                items.push(item);

                                let next = tokens.peek();
                                match next.ty {
                                    TokenType::Comma => {
                                        tokens.eat();
                                    },
                                    TokenType::RightParen => {},
                                    _ => return Err(Error::new(
                                        next.clone(), 
                                        "Exptected comma or right parenthesis".into()
                                    )),
                                }
                            }

                            tokens.eat();

                            (tokens, Expr::Tuple(items))
                        },
                        _ => {
                            return Err(Error::new(
                                right_paren_or_comma.clone(), 
                                "Expected right parenthesis".into()
                            ));
                        }
                    }
                }
            },
            TokenType::LeftSquare => {
                let mut items = vec![];

                while tokens.peek().ty != TokenType::RightSquare {
                    let (new_tokens, expr) = Expr::parse(tokens)?;
                    tokens = new_tokens;
                    items.push(expr);

                    let next = tokens.peek();
                    match next.ty {
                        TokenType::Comma =>  {
                            tokens.eat();
                        },
                        TokenType::RightSquare => (),
                        _ => return Err(Error::new(
                            next.clone(), 
                            "Expected comma or closing square bracket".into()
                        ))
                    }
                }
                let right_square = tokens.eat();
                if right_square.ty != TokenType::RightSquare {
                    return Err(Error::new(
                        right_square.clone(),
                        "Expected closing square bracket".into()
                    ))
                }

                (tokens, Expr::Array(items))
            },
            TokenType::Loop => {
                let (tokens, body) = Expr::parse(tokens)?;

                (tokens, Expr::Loop(box body))
            },
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
            },
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

                (tokens, Expr::If {
                    cond: box cond,
                    then: box then,
                    els: els.map(Box::new),
                })
            }
            _ => {
                if let Some(unary_op) = UnaryOperator::new(&token) {
                    let (tokens, expr) = Expr::parse_prec_lvl(
                        tokens, 
                        unary_op.precedence()
                    )?;

                    (tokens, Expr::Unary {
                        op: unary_op,
                        right: box expr,
                    })
                } else {
                    return Err(Error::new(token.clone(), "Expected expression".into()))
                }
            },
        };

        loop {
            match tokens.peek().ty {
                TokenType::LeftSquare => {
                    tokens.eat();

                    let (new_tokens, index) = Expr::parse(tokens)?;
                    tokens = new_tokens;

                    let right_square = tokens.eat();
                    if right_square.ty != TokenType::RightSquare {
                        return Err(Error::new(
                            right_square.clone(), 
                            "Expected closing square bracket to end indexing".into()
                        ))
                    }

                    expr = Expr::Indexing {
                        array: box expr,
                        index: box index,
                    };
                },
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
                            },
                            TokenType::RightParen => {},
                            _ => return Err(Error::new(
                                next.clone(), 
                                "Expected comma or right parenthesis".into()
                            ))
                        };
                    }
                    let right_paren = tokens.eat();
                    if right_paren.ty != TokenType::RightParen {
                        return Err(Error::new(
                            right_paren.clone(), 
                            "Expected right parenthesis".into()
                        ))
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
                tokens.eat(); // eat the binary operator
                let (new_tokens, right_expr) = Expr::parse_prec_lvl(tokens, op_prec)?;
                tokens = new_tokens;
                expr = Expr::Binary {
                    op: bin_op,
                    left: box expr,
                    right: box right_expr,
                };
            } else {
                break
            }
        }

        Ok((tokens, expr))
    }
}

impl Parse for Stmt {
    fn parse_impl(mut tokens: TokenStream, _prec_lvl: usize) -> Result<(TokenStream, Self), Error> {
        let token = tokens.peek();
        let (mut tokens, stmt) = match &token.ty {
            TokenType::Let => {
                tokens.eat();

                let name = tokens.eat();
                let name = if let TokenType::Ident(name) = &name.ty {
                    name.clone()
                } else {
                    return Err(Error::new(
                        name.clone(), 
                        "Expected identifier".into(),
                    ));
                };
                let ty = if tokens.peek().ty == TokenType::Colon {
                    tokens.eat();

                    let ty = tokens.eat();
                    let ty = if let TokenType::Ident(ty) = &ty.ty {
                        ty.clone()
                    } else {
                        return Err(Error::new(
                            ty.clone(),
                            "Expected type identifier".into()
                        ));
                    };

                    Some(Ident {
                        name: ty,
                    })
                } else {
                    None
                };
                let equals = tokens.eat();
                if equals.ty != TokenType::Equal {
                    return Err(Error::new(
                        equals.clone(),
                        "Expected equals sign".into(),
                    ));
                }
                let (tokens, value) = Expr::parse(tokens)?;

                (tokens, Stmt::Let {
                    ident: Ident {
                        name
                    },
                    ty,
                    value
                })
            },
            TokenType::Return => {
                tokens.eat();

                let (tokens, expr) = Expr::parse(tokens)?;

                (tokens, Stmt::Return(expr))
            },
            TokenType::For => {
                tokens.eat();

                let i = tokens.eat();
                let i = if let TokenType::Ident(i) = &i.ty {
                    Ident {
                        name: i.clone()
                    }
                } else {
                    return Err(Error::new(
                        i.clone(),
                        "Expected identifier".into()
                    ));
                };

                let in_token = tokens.eat();
                if in_token.ty != TokenType::In {
                    return Err(Error::new(
                        in_token.clone(),
                        "Expected 'in'".into(),
                    ));
                }

                let (tokens, iter) = Expr::parse(tokens)?;

                let (tokens, body) = Stmt::parse(tokens)?;

                (tokens, Stmt::For {
                    i,
                    iter,
                    body: box body,
                })
            },
            TokenType::Ident(ident) => {
                let ident = ident.clone();
                if let Some(op) = AssignmentOperator::new(tokens.peek_n(1)) {
                    tokens.eat(); // ident
                    tokens.eat();

                    let (tokens, value) = Expr::parse(tokens)?;

                    (tokens, Stmt::Assign {
                        ident: Ident { name: ident.clone() },
                        op,
                        value
                    })
                } else {
                    let (tokens, expr) = Expr::parse(tokens)?;

                    (tokens, Stmt::Expr(expr))
                }
            },
            _ => {
                let (tokens, expr) = Expr::parse(tokens)?;

                (tokens, Stmt::Expr(expr))
            }
        };

        if tokens.peek().ty == TokenType::Semicolon {
            tokens.eat();
        }

        Ok((tokens, stmt))
    }
}
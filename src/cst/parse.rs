use super::*;
use crate::diagnostics::Diagnostics;
use crate::lex::{Lexer, TokenKind};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    diagnostics: Diagnostics,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            diagnostics: Diagnostics::default(),
        }
    }

    pub fn diagnostics(&self) -> &Diagnostics {
        &self.diagnostics
    }

    fn eat_whitespace(&mut self) {
        while *self.lexer.peek().kind() == TokenKind::Whitespace {
            self.lexer.eat();
        }
    }

    pub fn parse_path(&mut self) -> Option<Path> {
        let mut path = vec![];
        loop {
            let ident_token = self.lexer.eat();
            let ident = match ident_token.kind() {
                TokenKind::Ident(ident) => ident.clone(),
                _ => {
                    self.diagnostics
                        .unexpected_token(ident_token, Some("identifier"));
                    return None;
                }
            };
            path.push(Ident::new(ident, ident_token.span().clone()));

            let colon_colon = self.lexer.peek();
            if *colon_colon.kind() == TokenKind::ColonColon {
                self.lexer.eat(); // ::
            } else {
                break;
            }
        }

        Some(Path::new(path))
    }

    pub fn parse_type(&mut self) -> Option<Type> {
        let token = self.lexer.peek();
        let ty = match token.kind() {
            TokenKind::Ident(_) => {
                let path = self.parse_path()?;
                let span = path.span();
                Type::new(TypeInner::Path(path), span)
            }
            TokenKind::Amp => {
                let start = self.lexer.eat().span().start(); // &
                let inner = self.parse_type()?;
                if let Type {
                    inner: TypeInner::InPlaceDynamicArray(ty),
                    span,
                } = inner
                {
                    Type::new(TypeInner::Slice(ty), TextSpan::new(start, span.end()))
                } else {
                    let span = TextSpan::new(start, inner.span().end());
                    Type::new(TypeInner::Reference(box inner), span)
                }
            }
            TokenKind::LeftParen => {
                let start = self.lexer.eat().span().start(); // (
                let mut types = vec![];
                while *self.lexer.peek().kind() != TokenKind::RightParen {
                    self.eat_whitespace();
                    types.push(self.parse_type()?);
                    self.eat_whitespace();
                    let next = self.lexer.peek();
                    match next.kind() {
                        TokenKind::Comma => {
                            self.lexer.eat();
                        }
                        TokenKind::RightParen => (),
                        _ => {
                            self.diagnostics.unexpected_token(
                                self.lexer.eat(),
                                Some("comma or right parenthesis"),
                            );
                        }
                    }
                }
                let right_paren = self.lexer.eat();
                if *right_paren.kind() != TokenKind::RightParen {
                    self.diagnostics
                        .unexpected_token(right_paren, Some("right parenthesis"));
                    return None;
                }
                Type::new(
                    TypeInner::Tuple(types),
                    TextSpan::new(start, right_paren.span().end()),
                )
            }
            TokenKind::LeftSquare => {
                let left_square = self.lexer.eat();
                self.eat_whitespace();
                let inner = box self.parse_type()?;

                self.eat_whitespace();
                let next = self.lexer.peek();
                let ty = match next.kind() {
                    TokenKind::DotDot => {
                        self.lexer.eat(); // ..
                        TypeInner::DynamicArray(inner)
                    }
                    TokenKind::Asterisk => {
                        self.lexer.eat(); // *
                        self.eat_whitespace();
                        let size = self.parse_expr()?;
                        TypeInner::SizedArray(inner, size)
                    }
                    TokenKind::RightSquare => TypeInner::InPlaceDynamicArray(inner),
                    _ => {
                        self.diagnostics.unexpected_token(
                            self.lexer.eat(),
                            Some("right square bracket, two dots, or asterisk"),
                        );
                        return None;
                    }
                };
                self.eat_whitespace();
                let right_square = self.lexer.eat();
                if *right_square.kind() != TokenKind::RightSquare {
                    self.diagnostics
                        .unexpected_token(right_square, Some("right square bracket"));
                    return None;
                }

                Type::new(ty, TextSpan::combine(left_square.span(), right_square.span()))
            }
            TokenKind::Function => {
                let _start = self.lexer.eat().span().start(); // fn
                unimplemented!()
            }
            TokenKind::LeftCurly => {
                let left_curly = self.lexer.eat();
                self.eat_whitespace();
                let yields = box self.parse_type()?;
                self.eat_whitespace();
                let next = self.lexer.peek();
                let returns = match next.kind() {
                    TokenKind::Comma => {
                        self.lexer.eat();
                        self.eat_whitespace();
                        Some(box self.parse_type()?)
                    }
                    TokenKind::RightCurly => None,
                    _ => {
                        self.diagnostics.unexpected_token(
                            self.lexer.eat(),
                            Some("comma or right curly bracket"),
                        );
                        return None;
                    }
                };
                self.eat_whitespace();
                let right_curly = self.lexer.eat();
                if *right_curly.kind() != TokenKind::RightCurly {
                    self.diagnostics
                        .unexpected_token(right_curly, Some("right curly bracket"));
                    return None;
                }

                Type::new(
                    TypeInner::Generator {
                        yields,
                        returns,
                    },
                    TextSpan::combine(left_curly.span(), right_curly.span()),
                )
            }
            TokenKind::Struct => {
                let _start = self.lexer.eat().span().start(); // struct
                unimplemented!()
            }
            _ => {
                self.diagnostics
                    .unexpected_token(self.lexer.eat(), Some("type"));
                return None;
            }
        };

        Some(ty)
    }

    pub fn parse_expr(&mut self) -> Option<Expr> {
        unimplemented!()
    }
}

use super::*;
use crate::diagnostics::Diagnostics;
use crate::lex::{Lexer, TokenKind};

macro_rules! match_token {
    ($self:expr, $kind:ident) => {
        match_token!($self, TokenKind::$kind, TokenKind::$kind)
    };
    ($self:expr, $kind:pat, $instance:expr) => {{
        let token = $self.lexer.eat();
        if !matches!(token.kind(), $kind) {
            $self
                .diagnostics
                .unexpected_token(token)
                .expected_token(&$instance);
            return None;
        }
        token
    }};
}

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

    pub fn parse_path(&mut self) -> Option<Path> {
        let mut path = vec![];
        loop {
            let mut ident_token = self.lexer.eat();
            let ident = match ident_token.take_kind() {
                TokenKind::Ident(ident) => ident,
                _ => {
                    self.diagnostics
                        .unexpected_token(ident_token)
                        .expected_token(&TokenKind::Ident(String::new()));
                    return None;
                }
            };
            path.push(Ident::new(ident, *ident_token.span()));

            if *ident_token.whitespace_after() {
                break;
            }

            let colon_colon = self.lexer.peek();
            if *colon_colon.kind() == TokenKind::ColonColon && !colon_colon.whitespace_after() {
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
                let amp_token = self.lexer.eat(); // &
                let inner = self.parse_type()?;
                if let Type {
                    inner: TypeInner::InPlaceDynamicArray(ty),
                    span,
                } = inner
                {
                    Type::new(TypeInner::Slice(ty), (amp_token.span(), &span))
                } else {
                    let span: TextSpan = (amp_token.span(), inner.span()).into();
                    Type::new(TypeInner::Reference(box inner), span)
                }
            }
            TokenKind::LeftParen => {
                let left_paren = self.lexer.eat();

                let (types, right_paren_span) = self.parse_many(
                    Self::parse_type,
                    TokenKind::RightParen,
                    Some(TokenKind::Comma),
                )?;

                Type::new(
                    TypeInner::Tuple(types),
                    (left_paren.span(), &right_paren_span),
                )
            }
            TokenKind::LeftSquare => {
                let left_square = self.lexer.eat();
                let inner = box self.parse_type()?;

                let next = self.lexer.peek();
                let ty = match next.kind() {
                    TokenKind::DotDot => {
                        self.lexer.eat(); // ..
                        TypeInner::DynamicArray(inner)
                    }
                    TokenKind::Asterisk => {
                        self.lexer.eat(); // *
                        let size = self.parse_expr()?;
                        TypeInner::SizedArray(inner, size)
                    }
                    TokenKind::RightSquare => TypeInner::InPlaceDynamicArray(inner),
                    _ => {
                        self.diagnostics
                            .unexpected_token(self.lexer.eat())
                            .expected_tokens(&[
                                TokenKind::RightSquare,
                                TokenKind::DotDot,
                                TokenKind::Asterisk,
                            ]);
                        return None;
                    }
                };
                let right_square = match_token!(self, RightSquare);

                Type::new(ty, (left_square.span(), right_square.span()))
            }
            TokenKind::Function => {
                let fn_token = self.lexer.eat();
                let args = match self.parse_type()? {
                    Type {
                        inner: TypeInner::Tuple(args),
                        ..
                    } => args,
                    arg => vec![arg],
                };
                match_token!(self, RightArrow);

                let returns = box self.parse_type()?;

                let span: TextSpan = (fn_token.span(), returns.span()).into();
                Type::new(TypeInner::Function { args, returns }, span)
            }
            TokenKind::LeftCurly => {
                let left_curly = self.lexer.eat();
                let yields = box self.parse_type()?;
                let next = self.lexer.peek();
                let returns = match next.kind() {
                    TokenKind::Comma => {
                        self.lexer.eat();
                        Some(box self.parse_type()?)
                    }
                    TokenKind::RightCurly => None,
                    _ => {
                        self.diagnostics
                            .unexpected_token(self.lexer.eat())
                            .expected_tokens(&[TokenKind::Comma, TokenKind::RightCurly]);
                        return None;
                    }
                };
                let right_curly = match_token!(self, RightCurly);

                Type::new(
                    TypeInner::Generator { yields, returns },
                    (left_curly.span(), right_curly.span()),
                )
            }
            TokenKind::Struct => {
                let struct_token = self.lexer.eat(); // struct
                match_token!(self, LeftCurly);

                fn parse_field(this: &mut Parser) -> Option<(Ident, Type)> {
                    // TODO: clean this up if possible
                    let mut ident_token = this.lexer.eat();
                    let name = match ident_token.take_kind() {
                        TokenKind::Ident(name) => name,
                        _ => {
                            this.diagnostics
                                .unexpected_token(ident_token)
                                .expected_token(&TokenKind::Ident(String::new()));
                            return None;
                        }
                    };
                    let ident = Ident::new(name, *ident_token.span());

                    match_token!(this, Colon);

                    let ty = this.parse_type()?;

                    Some((ident, ty))
                }

                let (fields, right_curly_span) =
                    self.parse_many(parse_field, TokenKind::RightCurly, Some(TokenKind::Comma))?;

                Type::new(
                    TypeInner::Struct(fields),
                    (struct_token.span(), &right_curly_span),
                )
            }
            _ => {
                self.diagnostics
                    .unexpected_token(self.lexer.eat())
                    .expected("type");
                return None;
            }
        };

        Some(ty)
    }

    pub fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_expr_(0)
    }

    fn parse_expr_(&mut self, prec_lvl: usize) -> Option<Expr> {
        let mut token = self.lexer.eat();
        let expr = match token.take_kind() {
            TokenKind::True => Expr::new(ExprInner::Literal(Literal::Bool(true)), *token.span()),
            TokenKind::False => Expr::new(ExprInner::Literal(Literal::Bool(false)), *token.span()),
            TokenKind::Null => Expr::new(ExprInner::Literal(Literal::Null), *token.span()),
            TokenKind::String(s) => {
                Expr::new(ExprInner::Literal(Literal::String(s)), *token.span())
            }
            TokenKind::Integer(val) => {
                Expr::new(ExprInner::Literal(Literal::Integer(val)), *token.span())
            }
            TokenKind::Float(val) => {
                Expr::new(ExprInner::Literal(Literal::Float(val)), *token.span())
            }
            TokenKind::LeftParen => {
                let (exprs, right_paren_span) = self.parse_many(
                    Self::parse_expr,
                    TokenKind::RightParen,
                    Some(TokenKind::Comma),
                )?;

                if exprs.len() == 1 {
                    let expr = exprs.into_iter().next().unwrap();
                    Expr::new(expr.inner, (token.span(), &right_paren_span))
                } else {
                    Expr::new(ExprInner::Tuple(exprs), (token.span(), &right_paren_span))
                }
            }
            TokenKind::Loop => {
                let expr = self.parse_expr()?;
                let expr_span = *expr.span();
                Expr::new(expr.inner, (token.span(), &expr_span))
            }
            TokenKind::LeftCurly => {
                let (stmts, right_curly_span) =
                    self.parse_many(Self::parse_stmt, TokenKind::RightParen, None)?;

                Expr::new(ExprInner::Block(stmts), (token.span(), &right_curly_span))
            }
            TokenKind::If => {
                let condition = box self.parse_expr()?;
                let then = box self.parse_stmt()?;
                let els = if *self.lexer.peek().kind() == TokenKind::Else {
                    self.lexer.eat(); // else
                    Some(box self.parse_stmt()?)
                } else {
                    None
                };

                let last = els.as_ref().unwrap_or(&then);
                let span: TextSpan = (token.span(), last.span()).into();
                Expr::new(
                    ExprInner::If {
                        condition,
                        then,
                        els,
                    },
                    span,
                )
            }
            _ => {
                self.diagnostics
                    .unexpected_token(token)
                    .expected("expression");

                return None;
            }
        };

        Some(expr)
    }

    pub fn parse_stmt(&mut self) -> Option<Stmt> {
        unimplemented!()
    }

    fn parse_many<T, P: FnMut(&mut Self) -> Option<T>>(
        &mut self,
        mut parser: P,
        finisher: TokenKind,
        mut separator: Option<TokenKind>,
    ) -> Option<(Vec<T>, TextSpan)> {
        let mut stuff = vec![];

        while *self.lexer.peek().kind() != finisher {
            stuff.push(parser(self)?);
            let next = self.lexer.peek();
            if separator
                .as_ref()
                .map(|separator| *separator == *next.kind())
                .unwrap_or(false)
            {
                self.lexer.eat();
                continue;
            }
            if *next.kind() != finisher {
                let d = self.diagnostics.unexpected_token(self.lexer.eat());
                if let Some(separator) = separator.take() {
                    d.expected_tokens(&[finisher, separator]);
                } else {
                    d.expected_token(&finisher);
                }
                return None;
            }
        }
        let finisher_token = self.lexer.eat();
        if *finisher_token.kind() != finisher {
            self.diagnostics
                .unexpected_token(finisher_token)
                .expected_token(&finisher);
            return None;
        }

        Some((stuff, *finisher_token.span()))
    }
}

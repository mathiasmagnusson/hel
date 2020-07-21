use std::borrow::Cow;

use crate::lex::Token;
use crate::text::TextSpan;

#[derive(Default, Debug)]
pub struct Diagnostics(Vec<Diagnostic>);

impl std::ops::Deref for Diagnostics {
    type Target = Vec<Diagnostic>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Diagnostics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Diagnostics {
    pub fn unterminated_string_literal(&mut self, position: usize) {
        self.push(Diagnostic {
            message: Cow::Borrowed("Unterminated string literal"),
            span: TextSpan::single(position),
        });
    }

    pub fn invalid_escape_character(&mut self, position: usize, c: char) {
        self.push(Diagnostic {
            message: Cow::Owned(format!("Invalid escape character: {}", c)),
            span: TextSpan::single(position),
        });
    }

    pub fn unterminated_multiline_comment(&mut self, position: usize) {
        self.push(Diagnostic {
            message: Cow::Borrowed("Unterminated multiline comment"),
            span: TextSpan::single(position),
        })
    }

    pub fn unexpected_character(&mut self, position: usize, c: char) {
        self.push(Diagnostic {
            message: Cow::Owned(format!("Unexpected character {}", c)),
            span: TextSpan::single(position),
        })
    }

    pub fn invalid_float_literal(&mut self, span: TextSpan) {
        self.push(Diagnostic {
            message: Cow::Owned(format!("Invalid float literal")),
            span,
        })
    }

    pub fn unexpected_token(&mut self, token: Token, expected: Option<&str>) {
        self.push(Diagnostic {
            message: Cow::Owned(if let Some(expected) = expected {
                format!("Unexpected {:?} token, expected {}", token.kind(), expected)
            } else {
                format!("Unexpected {:?} token", token.kind())
            }),
            span: token.span().clone(),
        })
    }
}

#[derive(Debug)]
pub struct Diagnostic {
    message: Cow<'static, str>,
    span: TextSpan,
}

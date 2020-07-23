use std::borrow::Cow;

use crate::lex::{Token, TokenKind};
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

#[derive(Debug)]
pub struct Diagnostic {
    message: Cow<'static, str>,
    span: TextSpan,
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

    pub fn unexpected_token(&mut self, token: Token) -> UnexpectedToken {
        UnexpectedToken { diagnostics: self, token }
    }
}

#[must_use]
pub struct UnexpectedToken<'d> {
    diagnostics: &'d mut Diagnostics,
    token: Token,
}

impl<'d> UnexpectedToken<'d> {
    pub fn expected_token(self, expected: &TokenKind) {
        self.diagnostics.push(Diagnostic {
            message: Cow::Owned(format!(
                "Unexpected {:?} token, expected {:?}",
                self.token, expected
            )),
            span: self.token.span().clone(),
        })
    }

    pub fn expected_tokens(self, expected: &[TokenKind]) {
        let mut message: Cow<'_, str> = Cow::Owned(format!("Unexpected {:?} token", self.token));
        if !expected.is_empty() {
            message.to_mut().push_str(", expected ");
            for (i, ex) in expected.iter().enumerate() {
                message.to_mut().push_str(&format!("{:?}", ex));
                if i + 2 < expected.len() {
                    message.to_mut().push_str(", ");
                } else if i + 2 == expected.len() {
                    message.to_mut().push_str(", or ");
                }
            }
        }

        self.diagnostics.push(Diagnostic {
            message,
            span: self.token.span().clone(),
        })
    }

    pub fn expected(self, expected: &str) {
        self.diagnostics.push(Diagnostic {
            message: Cow::Owned(format!(
                "Unexpected {:?} token, expected {}",
                self.token, expected
            )),
            span: self.token.span().clone(),
        })
    }
}

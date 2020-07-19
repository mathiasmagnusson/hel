use std::collections::HashMap;
use std::iter::Peekable;

use super::{Token, TokenKind};
use crate::diagnostics::Diagnostics;
use crate::text::TextSpan;

#[derive(Debug)]
pub struct Lexer<Input: Iterator<Item = char>> {
    input: Peekable<Input>,
    position: usize,
    diagnostics: Diagnostics,
    keywords: HashMap<&'static str, TokenKind>,
}

impl<Input: Iterator<Item = char>> Lexer<Input> {
    pub fn new(input: Input) -> Self {
        Self {
            input: input.peekable(),
            position: 0,
            diagnostics: Diagnostics::default(),
            #[rustfmt::skip]
            keywords: [
                ("let",    TokenKind::Let),
                ("null",   TokenKind::Null),
                ("and",    TokenKind::And),
                ("or",     TokenKind::Or),
                ("true",   TokenKind::True),
                ("false",  TokenKind::False),
                ("fn",     TokenKind::Function),
                ("type",   TokenKind::Type),
                ("struct", TokenKind::Struct),
                ("if",     TokenKind::If),
                ("then",   TokenKind::Then),
                ("else",   TokenKind::Else),
                ("for",    TokenKind::For),
                ("in",     TokenKind::In),
                ("loop",   TokenKind::Loop),
                ("return", TokenKind::Return),
                ("defer",  TokenKind::Defer),
                ("copy",   TokenKind::Copy),
                ("import", TokenKind::Import),
            ].iter().cloned().collect(),
        }
    }

    pub fn diagnostics(&self) -> &Diagnostics {
        &self.diagnostics
    }

    fn is_eof(&mut self) -> bool {
        self.input.peek().is_none()
    }

    fn eat(&mut self) -> char {
        self.position += 1;
        self.input.next().unwrap_or('\0')
    }

    fn peek(&mut self) -> char {
        self.input.peek().copied().unwrap_or('\0')
    }

    fn lex_string(&mut self) -> TokenKind {
        let mut value = String::new();
        loop {
            if self.is_eof() {
                self.diagnostics.unterminated_string_literal(self.position);
                break;
            }
            match self.eat() {
                '"' => {
                    break;
                }
                '\\' => match self.eat() {
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    c => {
                        self.diagnostics.invalid_escape_character(self.position, c);
                    }
                },
                c => {
                    value.push(c);
                }
            }
        }
        TokenKind::String(value)
    }
}

impl<Input: Iterator<Item = char>> Iterator for Lexer<Input> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof() {
            return None;
        }

        let start = self.position;

        let kind = match self.eat() {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftSquare,
            ']' => TokenKind::RightSquare,
            '{' => TokenKind::LeftCurly,
            '}' => TokenKind::RightCurly,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '?' => TokenKind::Quest,
            '@' => TokenKind::At,
            '$' => TokenKind::Dollar,
            ':' => match self.peek() {
                ':' => {
                    self.eat();
                    TokenKind::ColonColon
                }
                _ => TokenKind::Colon,
            },
            '&' => match self.peek() {
                '=' => {
                    self.eat();
                    TokenKind::AmpEq
                }
                _ => TokenKind::Amp,
            },
            '+' => match self.peek() {
                '=' => {
                    self.eat();
                    TokenKind::PlusEq
                }
                _ => TokenKind::Plus,
            },
            '-' => match self.peek() {
                '=' => {
                    self.eat();
                    TokenKind::MinusEq
                }
                '>' => {
                    self.eat();
                    TokenKind::RightArrow
                }
                _ => TokenKind::Minus,
            },
            '%' => match self.peek() {
                '=' => {
                    self.eat();
                    TokenKind::PercentEq
                }
                _ => TokenKind::Percent,
            },
            '|' => match self.peek() {
                '=' => {
                    self.eat();
                    TokenKind::BarEq
                }
                '>' => {
                    self.eat();
                    TokenKind::BarGt
                }
                _ => TokenKind::Bar,
            },
            '^' => match self.peek() {
                '=' => {
                    self.eat();
                    TokenKind::CaretEq
                }
                _ => TokenKind::Caret,
            },
            '*' => match self.peek() {
                '*' => {
                    self.eat();
                    match self.peek() {
                        '=' => {
                            self.eat();
                            TokenKind::AsteriskAsteriskEq
                        }
                        _ => TokenKind::AsteriskAsterisk,
                    }
                }
                '=' => {
                    self.eat();
                    TokenKind::AsteriskEq
                }
                _ => TokenKind::Asterisk,
            },
            '/' => match self.peek() {
                '=' => {
                    self.eat();
                    TokenKind::SlashEq
                }
                _ => TokenKind::Slash,
            },
            '!' => match self.peek() {
                '=' => {
                    self.eat();
                    TokenKind::BangEq
                }
                _ => TokenKind::Bang,
            },
            '=' => match self.peek() {
                '=' => {
                    self.eat();
                    TokenKind::EqualEqual
                }
                _ => TokenKind::Equal,
            },
            '<' => match self.peek() {
                '=' => {
                    self.eat();
                    TokenKind::LessEqual
                }
                _ => TokenKind::Less,
            },
            '>' => match self.peek() {
                '=' => {
                    self.eat();
                    TokenKind::GreaterEqual
                }
                _ => TokenKind::Greater,
            },
            '#' => match self.peek() {
                '-' => {
                    let mut depth = 1;
                    loop {
                        if self.is_eof() {
                            self.diagnostics
                                .unterminated_multiline_comment(self.position);
                            break;
                        }
                        let a = self.eat();
                        let b = self.peek();

                        if (a, b) == ('#', '-') {
                            depth += 1;
                        } else if (a, b) == ('-', '#') {
                            depth -= 1;
                        }
                        if depth == 0 {
                            self.eat();
                            break;
                        }
                    }
                    TokenKind::Comment
                }
                _ => {
                    while self.peek() != '\n' {
                        self.eat();
                    }
                    TokenKind::Comment
                }
            },
            '"' => self.lex_string(),
            c @ '0'..='9' => {
                let base = match self.peek() {
                    'b' => {
                        self.eat();
                        2
                    }
                    'x' => {
                        self.eat();
                        16
                    }
                    _ => 10,
                };
                let get_digit_value = |d| {
                    "0123456789abcdef"
                        .bytes()
                        .take(base)
                        .enumerate()
                        .find(|(_, digit)| *digit as char == d)
                        .map(|(i, _)| i)
                };
                let mut value = get_digit_value(c).unwrap();
                while let Some(val) = get_digit_value(self.peek()) {
                    self.eat();
                    value *= base;
                    value += val;
                }

                if self.peek() == '.' || self.peek() == 'e' {
                    let mut s = value.to_string();
                    if self.peek() == '.' {
                        s.push(self.eat());
                        while self.peek().is_ascii_digit() {
                            s.push(self.eat());
                        }
                    }
                    if self.peek() == 'e' {
                        s.push(self.eat());
                        while self.peek().is_ascii_digit() {
                            s.push(self.eat());
                        }
                    }
                    match s.parse() {
                        Ok(value) => TokenKind::Float(value),
                        Err(_err) => {
                            self.diagnostics
                                .invalid_float_literal(TextSpan::new(start, self.position));
                            TokenKind::BadCharacter
                        }
                    }
                } else {
                    TokenKind::Integer(value)
                }
            }
            w if w == '_' || w.is_alphabetic() => {
                let mut value = String::new();
                value.push(w);
                while self.peek() == '_' || self.peek().is_alphanumeric() {
                    value.push(self.eat());
                }

                if let Some(keyword) = self.keywords.get(value.as_str()) {
                    keyword.clone()
                } else {
                    TokenKind::Ident(value)
                }
            }
            s if s.is_whitespace() => {
                while self.peek().is_whitespace() {
                    self.eat();
                }
                TokenKind::Whitespace
            }
            c => {
                self.diagnostics.unexpected_character(self.position, c);
                TokenKind::BadCharacter
            }
        };
        return Some(Token::new(kind, TextSpan::new(start, self.position)));
    }
}

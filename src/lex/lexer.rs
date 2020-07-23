use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use super::{Token, TokenKind};
use crate::diagnostics::Diagnostics;
use crate::text::TextSpan;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
    diagnostics: Diagnostics,
    keywords: HashMap<&'static str, TokenKind>,
    peeked: Option<Token>,
    just_saw_whitespace: bool,
}

impl<'a> From<&'a String> for Lexer<'a> {
    fn from(input: &'a String) -> Self {
        Self {
            input: input.chars().peekable(),
            position: 0,
            diagnostics: Diagnostics::default(),
            keywords: Self::get_keywords(),
            peeked: None,
            just_saw_whitespace: false,
        }
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            position: 0,
            diagnostics: Diagnostics::default(),
            keywords: Self::get_keywords(),
            peeked: None,
            just_saw_whitespace: false,
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn diagnostics(&self) -> &Diagnostics {
        &self.diagnostics
    }

    #[rustfmt::skip]
    fn get_keywords() -> HashMap<&'static str, TokenKind> {
        [
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
            ("import", TokenKind::Import),
        ].iter().cloned().collect()
    }

    fn is_eof(&mut self) -> bool {
        self.input.peek().is_none()
    }

    fn eat_char(&mut self) -> char {
        self.position += 1;
        self.input.next().unwrap_or('\0')
    }

    fn peek_char(&mut self) -> char {
        self.input.peek().copied().unwrap_or('\0')
    }

    fn lex_string(&mut self) -> TokenKind {
        let mut value = String::new();
        loop {
            if self.is_eof() {
                self.diagnostics.unterminated_string_literal(self.position);
                break;
            }
            match self.eat_char() {
                '"' => {
                    break;
                }
                '\\' => match self.eat_char() {
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

    fn process(&mut self) -> Option<Token> {
        if self.is_eof() {
            return Some(Token::new(
                TokenKind::EOF,
                TextSpan::new(self.position, self.position),
                self.just_saw_whitespace,
                false,
            ));
        }

        let start = self.position;

        let kind = match self.eat_char() {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftSquare,
            ']' => TokenKind::RightSquare,
            '{' => TokenKind::LeftCurly,
            '}' => TokenKind::RightCurly,
            ',' => TokenKind::Comma,
            '?' => TokenKind::Quest,
            '@' => TokenKind::At,
            '$' => TokenKind::Dollar,
            '.' => match self.peek_char() {
                '.' => {
                    self.eat_char();
                    TokenKind::DotDot
                }
                _ => TokenKind::Dot,
            },
            ':' => match self.peek_char() {
                ':' => {
                    self.eat_char();
                    TokenKind::ColonColon
                }
                _ => TokenKind::Colon,
            },
            '&' => match self.peek_char() {
                '=' => {
                    self.eat_char();
                    TokenKind::AmpEq
                }
                _ => TokenKind::Amp,
            },
            '+' => match self.peek_char() {
                '=' => {
                    self.eat_char();
                    TokenKind::PlusEq
                }
                _ => TokenKind::Plus,
            },
            '-' => match self.peek_char() {
                '=' => {
                    self.eat_char();
                    TokenKind::MinusEq
                }
                '>' => {
                    self.eat_char();
                    TokenKind::RightArrow
                }
                _ => TokenKind::Minus,
            },
            '%' => match self.peek_char() {
                '=' => {
                    self.eat_char();
                    TokenKind::PercentEq
                }
                _ => TokenKind::Percent,
            },
            '|' => match self.peek_char() {
                '=' => {
                    self.eat_char();
                    TokenKind::BarEq
                }
                '>' => {
                    self.eat_char();
                    TokenKind::BarGt
                }
                _ => TokenKind::Bar,
            },
            '^' => match self.peek_char() {
                '=' => {
                    self.eat_char();
                    TokenKind::CaretEq
                }
                _ => TokenKind::Caret,
            },
            '*' => match self.peek_char() {
                '*' => {
                    self.eat_char();
                    match self.peek_char() {
                        '=' => {
                            self.eat_char();
                            TokenKind::AsteriskAsteriskEq
                        }
                        _ => TokenKind::AsteriskAsterisk,
                    }
                }
                '=' => {
                    self.eat_char();
                    TokenKind::AsteriskEq
                }
                _ => TokenKind::Asterisk,
            },
            '/' => match self.peek_char() {
                '=' => {
                    self.eat_char();
                    TokenKind::SlashEq
                }
                _ => TokenKind::Slash,
            },
            '!' => match self.peek_char() {
                '=' => {
                    self.eat_char();
                    TokenKind::BangEq
                }
                _ => TokenKind::Bang,
            },
            '=' => match self.peek_char() {
                '=' => {
                    self.eat_char();
                    TokenKind::EqualEqual
                }
                _ => TokenKind::Equal,
            },
            '<' => match self.peek_char() {
                '=' => {
                    self.eat_char();
                    TokenKind::LessEqual
                }
                _ => TokenKind::Less,
            },
            '>' => match self.peek_char() {
                '=' => {
                    self.eat_char();
                    TokenKind::GreaterEqual
                }
                _ => TokenKind::Greater,
            },
            '#' => match self.peek_char() {
                '-' => {
                    let mut depth = 1;
                    loop {
                        if self.is_eof() {
                            self.diagnostics
                                .unterminated_multiline_comment(self.position);
                            break;
                        }
                        let a = self.eat_char();
                        let b = self.peek_char();

                        if (a, b) == ('#', '-') {
                            depth += 1;
                        } else if (a, b) == ('-', '#') {
                            depth -= 1;
                        }
                        if depth == 0 {
                            self.eat_char();
                            break;
                        }
                    }
                    return None;
                }
                _ => {
                    while self.peek_char() != '\n' {
                        self.eat_char();
                    }
                    return None;
                }
            },
            '"' => self.lex_string(),
            c @ '0'..='9' => {
                let base = match self.peek_char() {
                    'b' => {
                        self.eat_char();
                        2
                    }
                    'x' => {
                        self.eat_char();
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
                while let Some(val) = get_digit_value(self.peek_char()) {
                    self.eat_char();
                    value *= base;
                    value += val;
                }

                if self.peek_char() == '.' || self.peek_char() == 'e' {
                    let mut s = value.to_string();
                    if self.peek_char() == '.' {
                        s.push(self.eat_char());
                        while self.peek_char().is_ascii_digit() {
                            s.push(self.eat_char());
                        }
                    }
                    if self.peek_char() == 'e' {
                        s.push(self.eat_char());
                        while self.peek_char().is_ascii_digit() {
                            s.push(self.eat_char());
                        }
                    }
                    match s.parse() {
                        Ok(value) => TokenKind::Float(value),
                        Err(_err) => {
                            self.diagnostics
                                .invalid_float_literal(TextSpan::new(start, self.position));
                            self.just_saw_whitespace = false;
                            return None;
                        }
                    }
                } else {
                    TokenKind::Integer(value)
                }
            }
            w if w == '_' || w.is_alphabetic() => {
                let mut value = String::new();
                value.push(w);
                while self.peek_char() == '_' || self.peek_char().is_alphanumeric() {
                    value.push(self.eat_char());
                }

                if let Some(keyword) = self.keywords.get(value.as_str()) {
                    keyword.clone()
                } else {
                    TokenKind::Ident(value)
                }
            }
            s if s.is_whitespace() => {
                while self.peek_char().is_whitespace() {
                    self.eat_char();
                }

                self.just_saw_whitespace = true;
                return None;
            }
            c => {
                self.diagnostics.unexpected_character(self.position, c);

                self.just_saw_whitespace = false;
                return None;
            }
        };

        let whitespace_before = self.just_saw_whitespace;
        let whitespace_after = self.peek_char().is_whitespace();

        self.just_saw_whitespace = false;

        return Some(Token::new(
            kind,
            TextSpan::new(start, self.position),
            whitespace_before,
            whitespace_after,
        ));
    }

    pub fn peek(&mut self) -> &Token {
        if self.peeked.is_none() {
            self.peeked = Some(self.eat());
        }

        self.peeked.as_ref().unwrap()
    }

    pub fn eat(&mut self) -> Token {
        if let Some(token) = self.peeked.take() {
            return token;
        }

        loop {
            if let Some(token) = self.process() {
                break token;
            }
        }
    }
}

use std::collections::HashMap;
use std::num::Wrapping;

use super::{Error, Errors, Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    start: usize,
    curr: usize,
    line: usize,
    tokens: Vec<Token>,
    errors: Vec<Error>,
    keywords: HashMap<&'static str, TokenType>,
}

impl Lexer {
    #[rustfmt::skip]
    pub fn new(input: &str) -> Self {
        let mut keywords = HashMap::new();
        {
            keywords.insert("let",    TokenType::Let);
            keywords.insert("null",   TokenType::Null);
            keywords.insert("and",    TokenType::And);
            keywords.insert("or",     TokenType::Or);
            keywords.insert("true",   TokenType::True);
            keywords.insert("false",  TokenType::False);
            keywords.insert("fn",     TokenType::Function);
            keywords.insert("type",   TokenType::Type);
            keywords.insert("struct", TokenType::Struct);
            keywords.insert("if",     TokenType::If);
            keywords.insert("then",   TokenType::Then);
            keywords.insert("else",   TokenType::Else);
            keywords.insert("for",    TokenType::For);
            keywords.insert("in",     TokenType::In);
            keywords.insert("loop",   TokenType::Loop);
            keywords.insert("return", TokenType::Return);
            keywords.insert("defer",  TokenType::Defer);
            keywords.insert("copy",   TokenType::Copy);
            keywords.insert("import", TokenType::Import);
        }

        Self {
            input: input.chars().collect(),
            start: 0,
            curr: 0,
            line: 1,
            tokens: vec![],
            errors: vec![],
            keywords,
        }
    }
    pub fn tokenize(mut self) -> Result<Vec<Token>, Errors> {
        while !self.is_eof() {
            self.start = self.curr;
            self.next_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, String::new(), self.line));

        if self.errors.len() > 0 {
            Err(Errors(self.errors))
        } else {
            Ok(self.tokens)
        }
    }
    fn next_token(&mut self) {
        match self.eat() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '[' => self.add_token(TokenType::LeftSquare),
            ']' => self.add_token(TokenType::RightSquare),
            '{' => self.add_token(TokenType::LeftCurly),
            '}' => self.add_token(TokenType::RightCurly),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '?' => self.add_token(TokenType::Quest),
            '@' => self.add_token(TokenType::At),
            '&' => self.add_token(TokenType::Amp),
            '$' => self.add_token(TokenType::Dollar),
            ':' => match self.peek() {
                ':' => {
                    self.eat();
                    self.add_token(TokenType::ColonColon);
                }
                _ => self.add_token(TokenType::Colon),
            },
            '+' => match self.peek() {
                '=' => {
                    self.eat();
                    self.add_token(TokenType::PlusEq);
                }
                _ => self.add_token(TokenType::Plus),
            },
            '-' => match self.peek() {
                '=' => {
                    self.eat();
                    self.add_token(TokenType::MinusEq);
                }
                '>' => {
                    self.eat();
                    self.add_token(TokenType::RightArrow);
                }
                _ => self.add_token(TokenType::Minus),
            },
            '%' => match self.peek() {
                '=' => {
                    self.eat();
                    self.add_token(TokenType::PercentEq);
                }
                _ => self.add_token(TokenType::Percent),
            },
            '|' => match self.peek() {
                '=' => {
                    self.eat();
                    self.add_token(TokenType::BarEq);
                }
                '>' => {
                    self.eat();
                    self.add_token(TokenType::BarGt);
                }
                _ => self.add_token(TokenType::Bar),
            },
            '^' => match self.peek() {
                '=' => {
                    self.eat();
                    self.add_token(TokenType::CaretEq);
                }
                _ => self.add_token(TokenType::Caret),
            },
            '*' => match self.peek() {
                '*' => {
                    self.eat();
                    match self.peek() {
                        '=' => {
                            self.eat();
                            self.add_token(TokenType::AsteriskAsteriskEq)
                        }
                        _ => self.add_token(TokenType::AsteriskAsterisk),
                    }
                }
                '=' => {
                    self.eat();
                    self.add_token(TokenType::AsteriskEq)
                }
                _ => self.add_token(TokenType::Asterisk),
            },
            '/' => match self.peek() {
                '=' => {
                    self.eat();
                    self.add_token(TokenType::SlashEq)
                }
                _ => self.add_token(TokenType::Slash),
            },
            '!' => {
                match self.peek() {
                    '=' => {
                        self.eat();
                        self.add_token(TokenType::BangEq)
                    }
                    _ => self.add_token(TokenType::Bang),
                };
            }
            '=' => {
                match self.peek() {
                    '=' => {
                        self.eat();
                        self.add_token(TokenType::EqualEqual)
                    }
                    _ => self.add_token(TokenType::Equal),
                };
            }
            '<' => {
                match self.peek() {
                    '=' => {
                        self.eat();
                        self.add_token(TokenType::LessEqual)
                    }
                    _ => self.add_token(TokenType::Less),
                };
            }
            '>' => {
                match self.peek() {
                    '=' => {
                        self.eat();
                        self.add_token(TokenType::GreaterEqual)
                    }
                    _ => self.add_token(TokenType::Greater),
                };
            }
            '#' => match self.peek() {
                '(' => {
                    let mut depth = 1;
                    loop {
                        if self.is_eof() {
                            break self.error("unterminated multi-line comment");
                        }
                        let a = self.eat();
                        let b = self.peek();

                        if (a, b) == ('#', '(') {
                            depth += 1;
                        } else if (a, b) == (')', '#') {
                            depth -= 1;
                        }
                        if depth == 0 {
                            self.eat();
                            break;
                        }
                    }
                }
                _ => {
                    while self.peek() != '\n' {
                        self.eat();
                    }
                }
            },
            '"' => {
                self.next_string();
            }
            c @ '0'..='9' => {
                let p = |c| (c as u8 - b'0') as usize;
                let mut literal: Wrapping<usize> = Wrapping::<usize>(p(c));
                while matches!(self.peek(), '0'..='9') {
                    literal *= Wrapping(10);
                    literal += Wrapping(p(self.eat()));
                }
                self.add_token(TokenType::Integer(literal.0));
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                while matches!(self.peek(), 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '0' ..= '9') {
                    self.eat();
                }
                let literal: String = self.input[self.start..self.curr].iter().collect();

                if let Some(keyword) = self.keywords.get(literal.as_str()) {
                    let keyword = keyword.clone();
                    self.add_token(keyword);
                } else {
                    self.add_token(TokenType::Ident(literal));
                }
            }
            c => self.error(format!("Unexpected character {}", c)),
        };
    }
    fn next_string(&mut self) {
        let mut literal = String::new();
        loop {
            if self.is_eof() {
                self.error("Unterminated string");
                return;
            }
            match self.eat() {
                '"' => {
                    break;
                }
                '\\' => match self.eat() {
                    '"' => literal.push('"'),
                    '\\' => literal.push('\\'),
                    'n' => literal.push('\n'),
                    't' => literal.push('\t'),
                    'r' => literal.push('\r'),
                    c => {
                        self.error(format!("Invalid escape character {}", c));
                        literal.push('�');
                    }
                },
                c => {
                    literal.push(c);
                }
            }
        }
        self.add_token(TokenType::String(literal));
    }
    fn error<S: Into<String>>(&mut self, message: S) {
        self.errors.push(Error::new(message.into(), self.line));
    }
    fn is_eof(&self) -> bool {
        self.curr >= self.input.len()
    }
    fn eat(&mut self) -> char {
        if self.is_eof() {
            '\0'
        } else {
            self.curr += 1;
            self.input[self.curr - 1]
        }
    }
    fn peek(&self) -> char {
        if self.is_eof() {
            '\0'
        } else {
            self.input[self.curr]
        }
    }
    #[allow(dead_code)]
    fn peek_n(&self, n: usize) -> char {
        if self.curr + n >= self.input.len() {
            '\0'
        } else {
            self.input[self.curr + n]
        }
    }
    fn add_token(&mut self, ty: TokenType) {
        self.tokens.push(Token::new(
            ty,
            self.input[self.start..self.curr].iter().collect::<String>(),
            self.line,
        ))
    }
}

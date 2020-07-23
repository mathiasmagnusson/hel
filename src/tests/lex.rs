use crate::lex::{Lexer, TokenKind};

#[test]
fn single_tokens() {
    for (text, kind) in basic_tokens().iter().chain(&tokens_with_values()) {
        let mut lexer = Lexer::from(*text);
        assert_eq!(lexer.eat().kind(), kind, "Lexing: {}", text);
        assert_eq!(*lexer.eat().kind(), TokenKind::EOF);
        assert!(lexer.diagnostics().is_empty());
    }
}

#[test]
fn token_pairs() {
    for (text1, kind1) in basic_tokens().iter().chain(&tokens_with_values()) {
        for (text2, kind2) in basic_tokens().iter().chain(&tokens_with_values()) {
            if require_separation(kind1, kind2) {
                let input = &format!("{} {}", text1, text2);
                let mut lexer = Lexer::from(input);
                let [first, second] = [lexer.eat(), lexer.eat()];

                assert_eq!(first.kind(), kind1);
                assert!(!first.whitespace_before());
                assert!(first.whitespace_after());

                assert_eq!(second.kind(), kind2);
                assert!(second.whitespace_before());
                assert!(!second.whitespace_after());

                assert_eq!(*lexer.eat().kind(), TokenKind::EOF);
                assert!(lexer.diagnostics().is_empty());
            } else {
                let input = &format!("{}{}", text1, text2);
                let mut lexer = Lexer::from(input);
                let [first, second] = [lexer.eat(), lexer.eat()];
                assert_eq!(first.kind(), kind1);
                assert_eq!(second.kind(), kind2);
                assert_eq!(*lexer.eat().kind(), TokenKind::EOF);
                assert!(lexer.diagnostics().is_empty());
            }
        }
    }
}

#[test]
fn whitespace() {
    let whitespace = &['\n', ' ', '\t', '\r', '\u{A0}'];
    for &c1 in whitespace {
        for &c2 in whitespace {
            for &c3 in whitespace {
                for &c4 in whitespace {
                    for &c5 in whitespace {
                        let whitespace = &format!("{}{}{}{}{}", c1, c2, c3, c4, c5);
                        let mut lexer = Lexer::from(whitespace);
                        let eof_token = lexer.eat();
                        assert_eq!(*eof_token.kind(), TokenKind::EOF);
                        assert!(eof_token.whitespace_before());
                        assert!(lexer.diagnostics().is_empty());
                    }
                }
            }
        }
    }
}

fn require_separation(kind1: &TokenKind, kind2: &TokenKind) -> bool {
    use TokenKind::*;
    match (kind1, kind2) {
        (Colon, Colon)
        | (Colon, ColonColon)
        | (Plus, Equal)
        | (Plus, EqualEqual)
        | (Minus, Equal)
        | (Minus, EqualEqual)
        | (Asterisk, Equal)
        | (Asterisk, EqualEqual)
        | (Minus, Greater)
        | (Minus, GreaterEqual)
        | (Percent, Equal)
        | (Percent, EqualEqual)
        | (Slash, Equal)
        | (Slash, EqualEqual)
        | (Asterisk, Asterisk)
        | (AsteriskAsterisk, Equal)
        | (AsteriskAsterisk, EqualEqual)
        | (Asterisk, AsteriskEq)
        | (Asterisk, AsteriskAsteriskEq)
        | (Asterisk, AsteriskAsterisk)
        | (Bang, Equal)
        | (Bang, EqualEqual)
        | (Equal, Equal)
        | (Equal, EqualEqual)
        | (Greater, Equal)
        | (Greater, EqualEqual)
        | (Less, Equal)
        | (Less, EqualEqual)
        | (Amp, Equal)
        | (Amp, EqualEqual)
        | (Bar, Equal)
        | (Bar, EqualEqual)
        | (Bar, Greater)
        | (Bar, GreaterEqual)
        | (Caret, Equal)
        | (Caret, EqualEqual)
        | (Dot, Dot)
        | (Dot, DotDot) => true,
        (kw1, kw2) if is_keyword(kw1) && is_keyword(kw2) => true,
        (kw, Integer(_)) if is_keyword(kw) => true,
        (kw, Ident(_)) if is_keyword(kw) => true,
        (kw, Float(_)) if is_keyword(kw) => true,
        (Integer(_), Dot) => true,
        (Integer(_), DotDot) => true,
        (Integer(_), kw) if is_keyword(kw) => true,
        (Integer(_), Integer(_)) => true,
        (Integer(_), Float(_)) => true,
        (Ident(_), kw) if is_keyword(kw) => true,
        (Ident(_), Integer(_)) => true,
        (Ident(_), Ident(_)) => true,
        (Ident(_), Float(_)) => true,
        (Float(_), kw) if is_keyword(kw) => true,
        (Float(_), Integer(_)) => true,
        (Float(_), Float(_)) => true,
        _ => false,
    }
}

fn is_keyword(kind: &TokenKind) -> bool {
    use TokenKind::*;
    return *kind == Let
        || *kind == Let
        || *kind == Null
        || *kind == And
        || *kind == Or
        || *kind == True
        || *kind == False
        || *kind == Function
        || *kind == Type
        || *kind == Struct
        || *kind == Import
        || *kind == If
        || *kind == Then
        || *kind == Else
        || *kind == For
        || *kind == In
        || *kind == Loop
        || *kind == Return
        || *kind == Defer;
}

fn tokens_with_values() -> [(&'static str, TokenKind); 7] {
    [
        ("498035872", TokenKind::Integer(498035872)),
        ("some_identifier", TokenKind::Ident("some_identifier".into())),
        ("0xdeadbeef", TokenKind::Integer(0xdeadbeef)),
        ("1.234", TokenKind::Float(1.234)),
        ("1e9", TokenKind::Float(1e9)),
        ("0b101010", TokenKind::Integer(42)),
        ("my_1st_variable", TokenKind::Ident("my_1st_variable".into())),
    ]
}

fn basic_tokens() -> [(&'static str, TokenKind); 60] {
    [
        ("(", TokenKind::LeftParen),
        (")", TokenKind::RightParen),
        ("{", TokenKind::LeftCurly),
        ("}", TokenKind::RightCurly),
        ("[", TokenKind::LeftSquare),
        ("]", TokenKind::RightSquare),
        (",", TokenKind::Comma),
        ("?", TokenKind::Quest),
        ("@", TokenKind::At),
        ("$", TokenKind::Dollar),
        (":", TokenKind::Colon),
        (".", TokenKind::Dot),
        //
        ("+", TokenKind::Plus),
        ("+=", TokenKind::PlusEq),
        ("-", TokenKind::Minus),
        ("-=", TokenKind::MinusEq),
        ("%", TokenKind::Percent),
        ("%=", TokenKind::PercentEq),
        ("/", TokenKind::Slash),
        ("/=", TokenKind::SlashEq),
        ("*", TokenKind::Asterisk),
        ("*=", TokenKind::AsteriskEq),
        ("!", TokenKind::Bang),
        ("!=", TokenKind::BangEq),
        ("=", TokenKind::Equal),
        ("==", TokenKind::EqualEqual),
        (">", TokenKind::Greater),
        (">=", TokenKind::GreaterEqual),
        ("<", TokenKind::Less),
        ("<=", TokenKind::LessEqual),
        ("&", TokenKind::Amp),
        ("&=", TokenKind::AmpEq),
        ("|", TokenKind::Bar),
        ("|=", TokenKind::BarEq),
        ("^", TokenKind::Caret),
        ("^=", TokenKind::CaretEq),
        //
        ("..", TokenKind::DotDot),
        ("->", TokenKind::RightArrow),
        ("|>", TokenKind::BarGt),
        ("::", TokenKind::ColonColon),
        //
        ("**", TokenKind::AsteriskAsterisk),
        ("**=", TokenKind::AsteriskAsteriskEq),
        //
        ("let", TokenKind::Let),
        ("null", TokenKind::Null),
        ("and", TokenKind::And),
        ("or", TokenKind::Or),
        ("true", TokenKind::True),
        ("false", TokenKind::False),
        ("fn", TokenKind::Function),
        ("type", TokenKind::Type),
        ("struct", TokenKind::Struct),
        ("import", TokenKind::Import),
        ("if", TokenKind::If),
        ("then", TokenKind::Then),
        ("else", TokenKind::Else),
        ("for", TokenKind::For),
        ("in", TokenKind::In),
        ("loop", TokenKind::Loop),
        ("return", TokenKind::Return),
        ("defer", TokenKind::Defer),
    ]
}

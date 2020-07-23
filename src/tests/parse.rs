use crate::cst::{Ident, Parser, Path, Type, TypeInner};
use crate::text::TextSpan;

#[test]
fn parse_path() {
    let paths = vec![
        vec!["sprutt", "i", "bang", "bang"],
        vec!["a", "b", "c", "d", "e", "f"],
    ];

    for path in paths {
        let joined = path.join("::");
        let mut parser = Parser::new(joined.as_str().into());

        let parsed_path = parser
            .parse_path()
            .expect(&format!("{:?}", parser.diagnostics()));

        assert_eq!(
            parsed_path
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
            path
        );

        assert!(
            parser.diagnostics().is_empty(),
            "{:?}",
            parser.diagnostics()
        )
    }
}

const TS: TextSpan = TextSpan::new(0, 0);

#[test]
fn parse_type() {
    let types = vec![
        (
            "path::to::a_type",
            Type::new(
                TypeInner::Path(Path::new(vec![
                    Ident::new("path".into(), TS),
                    Ident::new("to".into(), TS),
                    Ident::new("a_type".into(), TS),
                ])),
                TS,
            ),
        ),
        (
            "&stuff",
            Type::new(
                TypeInner::Reference(box Type::new(
                    TypeInner::Path(Path::new(vec![Ident::new("stuff".into(), TS)])),
                    TS,
                )),
                TS,
            ),
        ),
        (
            "(&a, b)",
            Type::new(
                TypeInner::Tuple(vec![
                    Type::new(
                        TypeInner::Reference(box Type::new(
                            TypeInner::Path(Path::new(vec![Ident::new("a".into(), TS)])),
                            TS,
                        )),
                        TS,
                    ),
                    Type::new(
                        TypeInner::Path(Path::new(vec![Ident::new("b".into(), TS)])),
                        TS,
                    ),
                ]),
                TS,
            ),
        ),
        (
            "[a..]",
            Type::new(
                TypeInner::DynamicArray(box Type::new(
                    TypeInner::Path(Path::new(vec![Ident::new("a".into(), TS)])),
                    TS,
                )),
                TS,
            ),
        ),
        (
            "&[a]",
            Type::new(
                TypeInner::Slice(box Type::new(
                    TypeInner::Path(Path::new(vec![Ident::new("a".into(), TS)])),
                    TS,
                )),
                TS,
            ),
        ),
        (
            "[a]",
            Type::new(
                TypeInner::InPlaceDynamicArray(box Type::new(
                    TypeInner::Path(Path::new(vec![Ident::new("a".into(), TS)])),
                    TS,
                )),
                TS,
            ),
        ),
        (
            "{a::b}",
            Type::new(
                TypeInner::Generator {
                    yields: box Type::new(
                        TypeInner::Path(Path::new(vec![
                            Ident::new("a".into(), TS),
                            Ident::new("b".into(), TS),
                        ])),
                        TS,
                    ),
                    returns: None,
                },
                TS,
            ),
        ),
        (
            "{a, [a..]}",
            Type::new(
                TypeInner::Generator {
                    yields: box Type::new(
                        TypeInner::Path(Path::new(vec![Ident::new("a".into(), TS)])),
                        TS,
                    ),
                    returns: Some(box Type::new(
                        TypeInner::DynamicArray(box Type::new(
                            TypeInner::Path(Path::new(vec![Ident::new("a".into(), TS)])),
                            TS,
                        )),
                        TS,
                    )),
                },
                TS,
            ),
        ),
        (
            "fn a -> b",
            Type::new(
                TypeInner::Function {
                    args: vec![Type::new(
                        TypeInner::Path(Path::new(vec![Ident::new("a".into(), TS)])),
                        TS,
                    )],
                    returns: box Type::new(
                        TypeInner::Path(Path::new(vec![Ident::new("b".into(), TS)])),
                        TS,
                    ),
                },
                TS,
            ),
        ),
        (
            "struct {call: fn a -> b,}",
            Type::new(
                TypeInner::Struct(vec![(
                    Ident::new("call".into(), TS),
                    Type::new(
                        TypeInner::Function {
                            args: vec![Type::new(
                                TypeInner::Path(Path::new(vec![Ident::new("a".into(), TS)])),
                                TS,
                            )],
                            returns: box Type::new(
                                TypeInner::Path(Path::new(vec![Ident::new("b".into(), TS)])),
                                TS,
                            ),
                        },
                        TS,
                    ),
                )]),
                TS,
            ),
        ),
        (
            "struct {
                prev: &LinkedList,
                next: &LinkedList,
                value: a
            }",
            Type::new(
                TypeInner::Struct(vec![
                    (
                        Ident::new("prev".into(), TS),
                        Type::new(
                            TypeInner::Reference(box Type::new(
                                TypeInner::Path(Path::new(vec![Ident::new(
                                    "LinkedList".into(),
                                    TS,
                                )])),
                                TS,
                            )),
                            TS,
                        ),
                    ),
                    (
                        Ident::new("next".into(), TS),
                        Type::new(
                            TypeInner::Reference(box Type::new(
                                TypeInner::Path(Path::new(vec![Ident::new(
                                    "LinkedList".into(),
                                    TS,
                                )])),
                                TS,
                            )),
                            TS,
                        ),
                    ),
                    (
                        Ident::new("value".into(), TS),
                        Type::new(
                            TypeInner::Path(Path::new(vec![Ident::new("a".into(), TS)])),
                            TS,
                        ),
                    ),
                ]),
                TS,
            ),
        ),
    ];

    for (input, ty) in types.into_iter() {
        let mut parser = Parser::new(input.into());
        let parsed =
            parser
                .parse_type()
                .expect(&format!("Input: {}\n{:#?}", input, parser.diagnostics()));
        assert!(parser.diagnostics().is_empty());
        assert_types_eq(parsed, ty);
    }
}

fn assert_types_eq(t1: Type, t2: Type) {
    let error_message = format!("\ngot\n{:?}\nexpected\n{:?}\n", &t1, t2);
    let fail = || panic!(error_message);
    match t1.inner {
        TypeInner::Path(p1) => {
            if let TypeInner::Path(p2) = t2.inner {
                assert_eq!(p1, p2)
            } else {
                fail();
            }
        }
        TypeInner::Reference(i1) => {
            if let TypeInner::Reference(i2) = t2.inner {
                assert_types_eq(*i1, *i2)
            } else {
                fail();
            }
        }
        TypeInner::Tuple(t1s) => {
            if let TypeInner::Tuple(t2s) = t2.inner {
                for (t1, t2) in t1s.into_iter().zip(t2s.into_iter()) {
                    assert_types_eq(t1, t2);
                }
            } else {
                fail();
            }
        }
        TypeInner::InPlaceDynamicArray(i1) => {
            if let TypeInner::InPlaceDynamicArray(i2) = t2.inner {
                assert_types_eq(*i1, *i2)
            } else {
                fail();
            }
        }
        TypeInner::SizedArray(_, _) => unimplemented!(),
        TypeInner::DynamicArray(i1) => {
            if let TypeInner::DynamicArray(i2) = t2.inner {
                assert_types_eq(*i1, *i2)
            } else {
                fail();
            }
        }
        TypeInner::Slice(i1) => {
            if let TypeInner::Slice(i2) = t2.inner {
                assert_types_eq(*i1, *i2)
            } else {
                fail();
            }
        }
        TypeInner::Function {
            args: a1s,
            returns: r1,
        } => {
            if let TypeInner::Function {
                args: a2s,
                returns: r2,
            } = t2.inner
            {
                assert_eq!(a1s.len(), a2s.len());
                for (a1, a2) in Iterator::zip(a1s.into_iter(), a2s.into_iter()) {
                    assert_types_eq(a1, a2);
                }
                assert_types_eq(*r1, *r2);
            } else {
                fail();
            }
        }
        TypeInner::Generator {
            yields: y1,
            returns: r1,
        } => {
            if let TypeInner::Generator {
                yields: y2,
                returns: r2,
            } = t2.inner
            {
                assert_types_eq(*y1, *y2);
                assert_types_eq(
                    *r1.unwrap_or(box Type::new(TypeInner::Tuple(vec![]), TS)),
                    *r2.unwrap_or(box Type::new(TypeInner::Tuple(vec![]), TS)),
                );
            } else {
                fail();
            }
        }
        TypeInner::Struct(i1s) => {
            if let TypeInner::Struct(i2s) = t2.inner {
                assert_eq!(i1s.len(), i2s.len());
                for ((n1, i1), (n2, i2)) in Iterator::zip(i1s.into_iter(), i2s.into_iter()) {
                    assert_eq!(n1, n2);
                    assert_types_eq(i1, i2);
                }
            } else {
                fail();
            }
        }
    }
}

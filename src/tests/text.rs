use crate::text::SourceCode;

#[test]
fn line_col() {
    let source = "3pic story:\nth3 forc3\nis strong with\nthis on3\n";
    for &source in &[source, source.trim()] {
        let source_code = SourceCode::new(source.into());

        let mut threes = source
            .char_indices()
            .filter_map(|(i, c)| if c == '3' { Some(i) } else { None })
            .map(|i| source_code.line_col(i));

        assert_eq!(threes.next(), Some((1, 1)));
        assert_eq!(threes.next(), Some((2, 3)));
        assert_eq!(threes.next(), Some((2, 9)));
        assert_eq!(threes.next(), Some((4, 8)));
    }
}

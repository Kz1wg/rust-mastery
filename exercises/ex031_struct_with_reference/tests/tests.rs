use ex031_struct_with_reference::Parser;

#[test]
fn yields_words_in_order() {
    let mut p = Parser::new("hello big world");
    assert_eq!(p.next_word(), Some("hello"));
    assert_eq!(p.next_word(), Some("big"));
    assert_eq!(p.next_word(), Some("world"));
    assert_eq!(p.next_word(), None);
}

#[test]
fn skips_repeated_whitespace() {
    let mut p = Parser::new("  a   b  ");
    assert_eq!(p.next_word(), Some("a"));
    assert_eq!(p.next_word(), Some("b"));
    assert_eq!(p.next_word(), None);
}

#[test]
fn empty_input_yields_nothing() {
    let mut p = Parser::new("");
    assert_eq!(p.next_word(), None);
}

/// 取り出した単語を持ったまま、次の単語を取り出せる。
/// next_word の戻り値が &mut self に結びついていたら、この関数はコンパイルできない（E0499）。
#[test]
fn words_can_be_held_while_parsing_continues() {
    let mut p = Parser::new("one two three");
    let w1 = p.next_word();
    let w2 = p.next_word();
    let w3 = p.next_word();
    assert_eq!((w1, w2, w3), (Some("one"), Some("two"), Some("three")));
}

/// 単語はパーサではなく元の文字列を借りているので、パーサを捨てた後も使える。
#[test]
fn words_outlive_the_parser() {
    let text = String::from("alpha beta");
    let words: Vec<&str> = {
        let mut p = Parser::new(&text);
        let mut out = Vec::new();
        while let Some(w) = p.next_word() {
            out.push(w);
        }
        out
    };
    assert_eq!(words, vec!["alpha", "beta"]);
}

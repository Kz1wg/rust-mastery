use ex059_api_review::{analyze, Count, Options, Unit};

fn count(item: &str, occurrences: usize) -> Count {
    Count {
        item: item.to_string(),
        occurrences,
    }
}

#[test]
fn words_with_default_options() {
    let result = analyze("b a b", &Options::default());
    assert_eq!(result, vec![count("b", 2), count("a", 1)]);
}

#[test]
fn words_ignoring_case() {
    let options = Options::default().ignore_case(true);
    let result = analyze("Hello hello HELLO world", &options);
    assert_eq!(result, vec![count("hello", 3), count("world", 1)]);
}

#[test]
fn words_are_case_sensitive_by_default() {
    let result = analyze("Hello hello", &Options::default());
    assert_eq!(result, vec![count("Hello", 1), count("hello", 1)]);
}

#[test]
fn chars_skip_whitespace() {
    let options = Options::default().unit(Unit::Chars);
    let result = analyze("a b a", &options);
    assert_eq!(result, vec![count("a", 2), count("b", 1)]);
}

/// 同じ回数のときは辞書順（結果が実行ごとに変わらないこと）。
#[test]
fn ties_are_ordered_alphabetically() {
    let result = analyze("c b a", &Options::default());
    assert_eq!(result, vec![count("a", 1), count("b", 1), count("c", 1)]);
}

#[test]
fn empty_text_has_no_counts() {
    assert!(analyze("", &Options::default()).is_empty());
}

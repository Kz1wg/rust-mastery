use ex003_ownership_design::{count_words, Config};

#[test]
fn count_words_basic() {
    assert_eq!(count_words("hello world"), 2);
    assert_eq!(count_words(""), 0);
    assert_eq!(count_words("one"), 1);
}

#[test]
fn count_words_handles_extra_whitespace() {
    assert_eq!(count_words("  hello   world  "), 2);
    assert_eq!(count_words("a\nb\tc"), 3);
}

/// count_words が &str リテラル・&String・&Stringの再借用のいずれからも
/// 呼び出せることを確認する（deref coercion）。
#[test]
fn count_words_callable_with_string_and_literal() {
    let owned = String::from("alice bob carol");
    assert_eq!(count_words(&owned), 3); // &String -> &str
    assert_eq!(count_words("x y"), 2); // &'static str
    assert_eq!(count_words(owned.as_str()), 3);
}

#[test]
fn config_new_from_literal() {
    let c = Config::new("alice");
    assert_eq!(c.name(), "alice");
}

#[test]
fn config_new_from_owned_string() {
    let owned = String::from("bob");
    let c = Config::new(owned);
    assert_eq!(c.name(), "bob");
}

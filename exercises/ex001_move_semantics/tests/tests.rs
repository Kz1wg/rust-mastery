use ex001_move_semantics::{longest_owned, longest_ref};

#[test]
fn longest_owned_picks_the_longer_one() {
    assert_eq!(
        longest_owned("hello".to_string(), "hi".to_string()),
        "hello"
    );
    assert_eq!(
        longest_owned("hi".to_string(), "hello".to_string()),
        "hello"
    );
}

#[test]
fn longest_owned_prefers_a_on_tie() {
    assert_eq!(longest_owned("aa".to_string(), "bb".to_string()), "aa");
}

#[test]
fn longest_ref_picks_the_longer_one() {
    assert_eq!(longest_ref("hello", "hi"), "hello");
    assert_eq!(longest_ref("hi", "hello"), "hello");
}

#[test]
fn longest_ref_prefers_a_on_tie() {
    assert_eq!(longest_ref("aa", "bb"), "aa");
}

/// 借用版は、呼び出し後も呼び出し側の変数が使えることを確認する。
/// もし longest_ref が誤って所有権を要求するシグネチャ（例: fn(String, String) -> String）
/// になっていたら、この関数自体がコンパイルできない。
#[test]
fn longest_ref_does_not_consume_its_arguments() {
    let a = String::from("hello");
    let b = String::from("hi");

    let result = longest_ref(&a, &b);
    assert_eq!(result, "hello");

    // a と b がまだ使えることを確認する
    assert_eq!(a, "hello");
    assert_eq!(b, "hi");
}

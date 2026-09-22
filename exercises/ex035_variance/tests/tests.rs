use ex035_variance::{collect_short_words, pick_longer, push_word};

#[test]
fn pick_longer_mixes_static_and_local() {
    let local = String::from("a longer local string");
    let s: &'static str = "short";
    // &'static str を、local と同じ短い 'a として渡せる（共変）
    assert_eq!(pick_longer(&local, s), "a longer local string");
    assert_eq!(pick_longer(s, "tiny"), "short");
}

#[test]
fn pick_longer_prefers_a_on_tie() {
    assert_eq!(pick_longer("ab", "cd"), "ab");
}

#[test]
fn push_word_with_matching_lifetimes() {
    let local = String::from("local");
    let mut v: Vec<&str> = vec!["static"]; // 型を 'static に固定しない
    push_word(&mut v, &local);
    assert_eq!(v, vec!["static", "local"]);
}

#[test]
fn collect_short_words_filters_by_length() {
    assert_eq!(
        collect_short_words("a bb ccc dddd", 2),
        vec!["a".to_string(), "bb".to_string()]
    );
}

/// 所有型で集めたので、元の文字列を捨てた後も使える。
#[test]
fn collected_words_outlive_the_source() {
    let words = {
        let text = String::from("x yy zzz");
        collect_short_words(&text, 2)
    };
    assert_eq!(words, vec!["x".to_string(), "yy".to_string()]);
}

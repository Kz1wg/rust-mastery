//! Lesson 01-1: moveは何を守っているのか
//!
//! `longest_owned` は所有権を受け取り、`longest_ref` は借用する。
//! 呼び出し側から見て何が違うかは、`tests/tests.rs` を読んで確認すること。

/// 長い方の `String` を返す。同じ長さなら `a` を返す。
/// 引数の所有権を受け取る（呼び出し側は以降 `a` / `b` を使えない）。
pub fn longest_owned(a: String, b: String) -> String {
    todo!("a と b の長さを比較し、長い方を返してください（同じ長さなら a）")
}

/// 長い方の `&str` を返す。同じ長さなら `a` を返す。
/// 引数を借用する（呼び出し側は呼び出し後も a / b を使える）。
pub fn longest_ref<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!("a と b の長さを比較し、長い方を返してください（同じ長さなら a）")
}

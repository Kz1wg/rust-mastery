//! Lesson 08-1: lifetime annotationは何を主張しているのか
//!
//! 2つの関数は、本体の違いより**シグネチャの違い**が重要。
//! `longest` は「戻り値は a と b の両方を借りうる」、
//! `first` は「戻り値は a だけを借りる」と宣言している。

/// 長い方を返す（同じ長さなら a）。
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!("a と b の長さを比べ、長い方を返してください（同じなら a）")
}

/// 常に a を返す。戻り値は b とは無関係。
/// `_b: &str` には、省略規則により a とは別の lifetime が割り当てられる
/// （`fn first<'a, 'b>(a: &'a str, _b: &'b str)` と同じ意味）。
pub fn first<'a>(a: &'a str, _b: &str) -> &'a str {
    todo!("a をそのまま返してください（b は使わない）")
}

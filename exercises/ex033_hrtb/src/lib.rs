//! Lesson 08-4: HRTB入門
//!
//! どちらの関数も「f / pred は、どんな lifetime の参照でも受け取れる」ことを要求している。
//! apply_to_local は for<'a> を明示し、count_matching は省略形 Fn(&str) で書いている。

/// 関数の中で String を作り、その参照を f に渡した結果を返す。
/// （中で作った値の参照を渡せるのは、f が「どんな 'a でも」受け取れるから。）
pub fn apply_to_local<F>(f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    todo!("\"hello world\" という String を作り、その参照を f に渡した結果を返してください")
}

/// pred が true を返す要素の数を数える。
/// `Fn(&str)` は暗黙に `for<'a> Fn(&'a str)` として扱われる。
pub fn count_matching<F: Fn(&str) -> bool>(items: &[String], pred: F) -> usize {
    todo!("items の各要素を &str として pred に渡し、true の数を数えてください")
}

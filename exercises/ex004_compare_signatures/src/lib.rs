//! Lesson 01-4: シグネチャを比較する
//!
//! 「文字列のリストをソートし、重複を除く」処理を3通りのシグネチャで実装する。
//! 3つとも同じ結果を返すが、所有権・確保・柔軟性が異なる。
//! 実装後、`NOTES.md`（任意）に (A) が有利になる場面を書いてみること。

/// (A) 所有権を受け取る。入力の `Vec` を並べ替えて重複を除き、そのまま返す
/// （新しい `Vec` を確保しない。各要素の `String` 自体も再利用する）。
pub fn dedup_sorted_owned(mut v: Vec<String>) -> Vec<String> {
    todo!("v をソートし、連続する重複を取り除いて返してください（新しい Vec を作らない）")
}

/// (B) スライスを借りる。元の `v` は変更せず、新しい `Vec` を返す。
pub fn dedup_sorted_ref(v: &[String]) -> Vec<String> {
    todo!("v の内容をコピーしてソートし、重複を除いた新しい Vec を返してください")
}

/// (C) `IntoIterator` を受け、`&str` として読めるものなら何でも受け付ける。
pub fn dedup_sorted_iter<I>(v: I) -> Vec<String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    todo!("v の要素を String にして集め、ソートし、重複を除いて返してください")
}

//! Lesson 01-2: 借用のエラーを読む
//!
//! 以下のコメントは、最初に書いたら借用エラーになるコードです。
//! そのまま使わず、**設計を変えて**エラーを起こさずに同じ結果を得てください。
//!
//! ```text
//! // これはコンパイルできない（本文 Bad Example 1 と同じ形）:
//! pub fn first_then_push_BAD(v: &mut Vec<i32>, extra: i32) -> i32 {
//!     let first = &v[0];
//!     v.push(extra);
//!     *first
//! }
//! ```

/// `v` の先頭要素を返しつつ、`extra` を末尾に追加する。
///
/// Bad Example のように参照を持ち続けてはいけない。
/// `i32` は `Copy` であることを利用し、値を先に取り出す設計にすること。
pub fn first_then_push(v: &mut Vec<i32>, extra: i32) -> i32 {
    todo!("v の先頭要素の「値」を先に取り出してから push してください")
}

/// `v[i]` と `v[j]` の両方に `1` を加える（`i == j` の場合は `2` 加わる）。
///
/// `i == j` のときは `v[i] += 2;` の1回の操作で済む。
/// `i != j` のときは、2つの可変参照を同時に得るために
/// `slice::split_at_mut` を使うこと（本文 Bad Example 2 を参照）。
pub fn bump_two(v: &mut [i32], i: usize, j: usize) {
    todo!("i == j の場合と i != j の場合を分けて実装してください")
}

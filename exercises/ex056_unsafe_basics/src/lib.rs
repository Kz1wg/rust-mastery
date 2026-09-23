//! Lesson 15-1: unsafe は何を宣言しているのか
//!
//! 生ポインタを「作る」のは安全、「たどる」のは unsafe:
//!
//! ```compile_fail
//! let x = 5;
//! let p = &x as *const i32;
//! let _ = *p; // unsafe ブロックの外では参照外しできない
//! ```

/// スライスの先頭を、生ポインタ経由で読んで返す。空なら None。
///
/// （本当は slice.first().copied() で済む。これは練習のための書き方。）
pub fn first_via_ptr(slice: &[i32]) -> Option<i32> {
    if slice.is_empty() {
        return None;
    }
    let ptr = slice.as_ptr();
    todo!(
        "SAFETY コメントを書いたうえで、unsafe ブロックで ptr をたどって値を Some で返してください"
    )
}

/// ポインタが指す値を読む。
///
/// # Safety
///
/// - `p` は null であってはならない
/// - `p` は、有効な（初期化済みで、まだ生きている）`i32` を指していなければならない
pub unsafe fn read_value(p: *const i32) -> i32 {
    todo!("SAFETY コメントを書いたうえで、unsafe ブロックで p をたどって返してください")
}

/// 2つの値を入れ替える。unsafe を使わずに書けるので、使わないこと。
pub fn swap_values(a: &mut i32, b: &mut i32) {
    todo!("std::mem::swap を使ってください")
}

//! Lesson 07-1: adapterの組み合わせ
//!
//! even_squares は単純な変換なので adapter の連鎖で。
//! first_index_over は複数の状態を追跡する処理なので for ループで実装する。

/// 偶数だけを選び、二乗したものを集める。
pub fn even_squares(nums: &[i32]) -> Vec<i32> {
    todo!("filter と map を使って実装してください")
}

/// 先頭から累積和を取り、初めて threshold を超えた位置（インデックス）を返す。
/// 超える要素が無ければ None。
pub fn first_index_over(nums: &[i32], threshold: i32) -> Option<usize> {
    todo!("forループで、累積和を持ちながら実装してください")
}

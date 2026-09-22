//! Lesson 09-4: zero-cost abstraction
//!
//! iterator 版と for ループ版は、release ビルドでは同等の機械語になることが多い（保証ではない）。
//! 速さが重要なら、Lesson の Challenge のように計測・機械語で確かめること。

/// 偶数の二乗の和（iterator の連鎖で実装する）。
pub fn sum_even_squares_iter(v: &[u64]) -> u64 {
    todo!("filter と map と sum で実装してください")
}

/// 偶数の二乗の和（for ループで実装する）。
pub fn sum_even_squares_loop(v: &[u64]) -> u64 {
    todo!("for ループと可変の合計で実装してください")
}

/// repr(transparent) の newtype。実行時には u64 そのもの。
#[repr(transparent)]
pub struct UserId(u64);

impl UserId {
    pub fn new(value: u64) -> Self {
        todo!("value を包んだ UserId を返してください")
    }

    pub fn get(&self) -> u64 {
        todo!("中の値を返してください")
    }
}

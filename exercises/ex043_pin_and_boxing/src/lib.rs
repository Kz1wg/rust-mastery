//! Lesson 11-2: Pin / Unpin
//!
//! async ブロックが作る Future は Unpin ではない（自己参照しうるため）。
//! Box::pin で固定すれば、Pin<Box<dyn Future>> としてどこへでも持ち運べる。

pub mod runtime;
pub use runtime::block_on;

use std::future::Future;
use std::pin::Pin;

/// 固定済みの Future を受け取って実行する。
///
/// async ブロックの Future は Unpin ではない:
///
/// ```compile_fail
/// fn assert_unpin<T: Unpin>(_: T) {}
///
/// let f = async { 1u32 };
/// assert_unpin(f); // 動かせない（自己参照しうる状態機械）
/// ```
pub fn run_boxed(future: Pin<Box<dyn Future<Output = u32>>>) -> u32 {
    todo!("block_on で実行した結果を返してください")
}

/// 条件によって異なる async ブロックを返す。
/// 異なる具象型を返すので、impl Future ではなく Box<dyn Future> が必要（05-3 と同じ理由）。
pub fn make_boxed_future(double: bool, value: u32) -> Pin<Box<dyn Future<Output = u32>>> {
    if double {
        Box::pin(async move { value * 2 })
    } else {
        todo!("value をそのまま返す async ブロックを Box::pin して返してください")
    }
}

/// Pin<Box<F>> を持つ型は Unpin になる（中身が動かないので、外側は動いてよい）。
pub fn boxed_future_is_unpin() -> bool {
    fn assert_unpin<T: Unpin>(_value: &T) {}

    // async ブロック自体は Unpin ではないが、Box::pin で固定した後の
    // Pin<Box<dyn Future>> は Unpin（外側は動かしてよい）。
    let boxed: Pin<Box<dyn Future<Output = u32>>> = Box::pin(async { 1u32 });
    assert_unpin(&boxed);
    true
}

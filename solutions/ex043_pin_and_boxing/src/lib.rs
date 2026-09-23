pub mod runtime;
pub use runtime::block_on;

use std::future::Future;
use std::pin::Pin;

/// ```compile_fail
/// fn assert_unpin<T: Unpin>(_: T) {}
///
/// let f = async { 1u32 };
/// assert_unpin(f); // 動かせない（自己参照しうる状態機械）
/// ```
pub fn run_boxed(future: Pin<Box<dyn Future<Output = u32>>>) -> u32 {
    block_on(future)
}

pub fn make_boxed_future(double: bool, value: u32) -> Pin<Box<dyn Future<Output = u32>>> {
    if double {
        Box::pin(async move { value * 2 })
    } else {
        Box::pin(async move { value })
    }
}

pub fn boxed_future_is_unpin() -> bool {
    fn assert_unpin<T: Unpin>(_value: &T) {}

    // async ブロック自体は Unpin ではないが、Box::pin で固定した後の
    // Pin<Box<dyn Future>> は Unpin（外側は動かしてよい）。
    let boxed: Pin<Box<dyn Future<Output = u32>>> = Box::pin(async { 1u32 });
    assert_unpin(&boxed);
    true
}

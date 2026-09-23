pub mod runtime;
pub use runtime::block_on;

use std::future::Future;
use std::sync::Arc;

pub async fn yield_once() {}

/// ```compile_fail
/// use std::rc::Rc;
/// use ex044_async_ownership::{assert_send_future, yield_once};
///
/// let f = async {
///     let data = Rc::new(vec![1u64, 2, 3]);
///     yield_once().await;
///     data.iter().sum::<u64>()
/// };
/// assert_send_future(&f);
/// ```
pub async fn sum_shared(data: Arc<Vec<u64>>) -> u64 {
    yield_once().await;
    data.iter().sum()
}

pub fn assert_send_future<F: Future + Send>(_f: &F) {}

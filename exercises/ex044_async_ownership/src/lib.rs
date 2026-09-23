//! Lesson 11-3: asyncと所有権
//!
//! Future が Send かどうかは、.await をまたいで何を持っているかで決まる。
//! Arc なら Send のまま、Rc だと Send でなくなる。

pub mod runtime;
pub use runtime::block_on;

use std::future::Future;
use std::sync::Arc;

/// 何もしないが .await できる Future（状態機械に切れ目を作るため）。
pub async fn yield_once() {}

/// Arc で共有したデータの合計を返す。.await をまたいで Arc を保持しても Send のまま。
///
/// Rc を .await をまたいで持つと Send でなくなる:
///
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
    todo!("yield_once().await を挟んでから、data の合計を返してください")
}

/// Future が Send であることをコンパイル時に確かめるヘルパー。
pub fn assert_send_future<F: Future + Send>(_f: &F) {}

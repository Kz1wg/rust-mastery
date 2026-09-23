//! Lesson 11-1: Future とは何か
//!
//! Future は poll されて初めて進む。Pending を返すときは、
//! 「後で起こす」責任（waker）を果たさないと永久に止まる。

pub mod runtime;
pub use runtime::block_on;

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

/// n 回 Pending を返してから Ready になる Future。
pub struct Countdown {
    pub n: u32,
    /// poll が呼ばれた回数を記録する（テスト用）。
    pub polls: Arc<AtomicU32>,
}

impl Future for Countdown {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.polls.fetch_add(1, Ordering::SeqCst);
        let result: Poll<&'static str> = todo!(
            "n が 0 なら Ready を返し、そうでなければ n を1減らし、waker を起こして Pending を返してください"
        );
        result
    }
}

pub async fn add(a: u32, b: u32) -> u32 {
    a + b
}

/// async fn を block_on で実行する。
pub fn run_add(a: u32, b: u32) -> u32 {
    todo!("add(a, b) を block_on で実行した結果を返してください")
}

/// Countdown を完了まで実行し、poll が呼ばれた回数を返す。
pub fn poll_count(n: u32) -> u32 {
    let polls = Arc::new(AtomicU32::new(0));
    let countdown = Countdown {
        n,
        polls: Arc::clone(&polls),
    };
    block_on(countdown);
    polls.load(Ordering::SeqCst)
}

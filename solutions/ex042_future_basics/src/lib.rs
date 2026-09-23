pub mod runtime;
pub use runtime::block_on;

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct Countdown {
    pub n: u32,
    pub polls: Arc<AtomicU32>,
}

impl Future for Countdown {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.polls.fetch_add(1, Ordering::SeqCst);
        if self.n == 0 {
            Poll::Ready("finished")
        } else {
            self.n -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

pub async fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn run_add(a: u32, b: u32) -> u32 {
    block_on(add(a, b))
}

pub fn poll_count(n: u32) -> u32 {
    let polls = Arc::new(AtomicU32::new(0));
    let countdown = Countdown {
        n,
        polls: Arc::clone(&polls),
    };
    block_on(countdown);
    polls.load(Ordering::SeqCst)
}

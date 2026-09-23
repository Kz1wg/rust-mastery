use ex042_future_basics::{block_on, poll_count, run_add, Countdown};
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

#[test]
fn run_add_executes_the_async_fn() {
    assert_eq!(run_add(2, 3), 5);
}

#[test]
fn countdown_finishes() {
    let c = Countdown {
        n: 3,
        polls: Arc::new(AtomicU32::new(0)),
    };
    assert_eq!(block_on(c), "finished");
}

#[test]
fn countdown_with_zero_is_ready_immediately() {
    assert_eq!(poll_count(0), 1);
}

/// n 回 Pending を返した後に Ready になるので、poll は n + 1 回呼ばれる。
#[test]
fn poll_is_called_once_per_pending_plus_final_ready() {
    assert_eq!(poll_count(3), 4);
    assert_eq!(poll_count(10), 11);
}

#[test]
fn block_on_works_with_async_blocks() {
    let result = block_on(async {
        let x = ex042_future_basics::add(1, 2).await;
        x * 10
    });
    assert_eq!(result, 30);
}

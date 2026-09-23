use ex045_join2::{block_on, join2, Steps};
use std::sync::{Arc, Mutex};

#[test]
fn join2_returns_both_results() {
    let result = block_on(join2(async { 1u32 }, async { "two" }));
    assert_eq!(result, (1, "two"));
}

/// スレッドを1つも使わずに、2つの処理が交互に進む（並行 ≠ 並列）。
#[test]
fn both_futures_make_progress_alternately() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let a = Steps {
        n: 2,
        label: "a",
        log: Arc::clone(&log),
    };
    let b = Steps {
        n: 2,
        label: "b",
        log: Arc::clone(&log),
    };

    block_on(join2(a, b));

    assert_eq!(*log.lock().unwrap(), vec!["a", "b", "a", "b", "a", "b"]);
}

/// 片方が先に終わっても、もう片方は最後まで進む。
#[test]
fn shorter_future_finishes_first_but_both_complete() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let a = Steps {
        n: 0,
        label: "a",
        log: Arc::clone(&log),
    };
    let b = Steps {
        n: 2,
        label: "b",
        log: Arc::clone(&log),
    };

    block_on(join2(a, b));

    let entries = log.lock().unwrap().clone();
    assert_eq!(entries.iter().filter(|l| **l == "a").count(), 1);
    assert_eq!(entries.iter().filter(|l| **l == "b").count(), 3);
}

#[test]
fn join2_with_immediate_futures() {
    let result = block_on(join2(async { 10u32 }, async { 20u32 }));
    assert_eq!(result.0 + result.1, 30);
}

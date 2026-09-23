use ex043_pin_and_boxing::{boxed_future_is_unpin, make_boxed_future, run_boxed};

#[test]
fn run_boxed_executes_the_future() {
    assert_eq!(run_boxed(Box::pin(async { 7 })), 7);
}

#[test]
fn make_boxed_future_doubles_when_asked() {
    assert_eq!(run_boxed(make_boxed_future(true, 21)), 42);
}

#[test]
fn make_boxed_future_passes_through_otherwise() {
    assert_eq!(run_boxed(make_boxed_future(false, 21)), 21);
}

#[test]
fn both_branches_have_the_same_type() {
    // 条件で分岐しても、Pin<Box<dyn Future>> なら同じ型として扱える
    let futures = vec![make_boxed_future(true, 1), make_boxed_future(false, 1)];
    let results: Vec<u32> = futures.into_iter().map(run_boxed).collect();
    assert_eq!(results, vec![2, 1]);
}

#[test]
fn pinned_box_is_unpin() {
    assert!(boxed_future_is_unpin());
}

use ex044_async_ownership::{assert_send_future, block_on, sum_shared, yield_once};
use std::sync::Arc;

#[test]
fn sum_shared_adds_all_elements() {
    let data = Arc::new(vec![1, 2, 3, 4]);
    assert_eq!(block_on(sum_shared(data)), 10);
}

#[test]
fn sum_shared_with_empty_data() {
    assert_eq!(block_on(sum_shared(Arc::new(vec![]))), 0);
}

/// Arc を .await をまたいで持っても、Future は Send のまま。
#[test]
fn future_is_send_with_arc() {
    let f = sum_shared(Arc::new(vec![1, 2]));
    assert_send_future(&f);
    assert_eq!(block_on(f), 3);
}

/// .await をまたがなければ、Send でない値を使っても Future は Send。
#[test]
fn value_dropped_before_await_keeps_future_send() {
    let f = async {
        {
            let local = std::rc::Rc::new(1u64);
            let _ = *local;
        } // ここで drop されるので、.await の地点では存在しない
        yield_once().await;
        42u64
    };
    assert_send_future(&f);
    assert_eq!(block_on(f), 42);
}

use ex038_send_sync::{assert_send, assert_sync, spawn_sum};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

#[test]
fn spawn_sum_adds_all_elements() {
    assert_eq!(spawn_sum(vec![1, 2, 3, 4]), 10);
}

#[test]
fn spawn_sum_with_odd_length() {
    assert_eq!(spawn_sum(vec![1, 2, 3, 4, 5]), 15);
}

#[test]
fn spawn_sum_with_empty_input() {
    assert_eq!(spawn_sum(vec![]), 0);
}

#[test]
fn owned_types_are_send_and_sync() {
    assert_send::<Vec<u64>>();
    assert_sync::<Vec<u64>>();
    assert_send::<Arc<Mutex<u64>>>();
    assert_sync::<Arc<Mutex<u64>>>();
}

/// RefCell は Send だが Sync ではない（まるごと渡せるが、共有はできない）。
#[test]
fn refcell_is_send_but_not_sync() {
    assert_send::<RefCell<u8>>();
    // assert_sync::<RefCell<u8>>(); // これはコンパイルできない
}

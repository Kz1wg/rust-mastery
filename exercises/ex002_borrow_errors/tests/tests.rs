use ex002_borrow_errors::{bump_two, first_then_push};

#[test]
fn first_then_push_returns_original_first_element() {
    let mut v = vec![1, 2, 3];
    let first = first_then_push(&mut v, 99);
    assert_eq!(first, 1);
    assert_eq!(v, vec![1, 2, 3, 99]);
}

#[test]
fn first_then_push_on_single_element() {
    let mut v = vec![42];
    let first = first_then_push(&mut v, 7);
    assert_eq!(first, 42);
    assert_eq!(v, vec![42, 7]);
}

#[test]
fn bump_two_with_distinct_indices() {
    let mut v = vec![10, 20, 30, 40];
    bump_two(&mut v, 0, 3);
    assert_eq!(v, vec![11, 20, 30, 41]);
}

#[test]
fn bump_two_with_indices_reversed() {
    // i > j でも正しく動くこと（split_at_mut は i < j を前提にしがちなので注意）
    let mut v = vec![10, 20, 30, 40];
    bump_two(&mut v, 3, 0);
    assert_eq!(v, vec![11, 20, 30, 41]);
}

#[test]
fn bump_two_with_same_index() {
    let mut v = vec![10, 20, 30];
    bump_two(&mut v, 1, 1);
    assert_eq!(v, vec![10, 22, 30]);
}

use ex015_associated_type::{drain_all, IntStack, Stack};

#[test]
fn push_then_pop_returns_last_pushed() {
    let mut s = IntStack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}

#[test]
fn drain_all_returns_items_in_pop_order() {
    let mut s = IntStack::new();
    s.push(10);
    s.push(20);
    s.push(30);
    let drained = drain_all(&mut s);
    assert_eq!(drained, vec![30, 20, 10]);
}

#[test]
fn drain_all_on_empty_stack_returns_empty_vec() {
    let mut s = IntStack::new();
    let drained = drain_all(&mut s);
    assert_eq!(drained, Vec::<i32>::new());
}

#[test]
fn drain_all_leaves_stack_empty() {
    let mut s = IntStack::new();
    s.push(1);
    s.push(2);
    let _ = drain_all(&mut s);
    assert_eq!(s.pop(), None);
}

use ex020_impl_trait::{evens_up_to, shout};

#[test]
fn shout_with_integer() {
    assert_eq!(shout(42), "42");
}

#[test]
fn shout_with_str() {
    assert_eq!(shout("hi"), "HI");
}

#[test]
fn shout_with_string() {
    assert_eq!(shout("hello".to_string()), "HELLO");
}

#[test]
fn evens_up_to_collects_correctly() {
    let v: Vec<i32> = evens_up_to(10).collect();
    assert_eq!(v, vec![0, 2, 4, 6, 8]);
}

#[test]
fn evens_up_to_zero_is_empty() {
    let v: Vec<i32> = evens_up_to(0).collect();
    assert_eq!(v, Vec::<i32>::new());
}

#[test]
fn evens_up_to_is_a_real_iterator() {
    // Iterator のメソッド（sum など）がそのまま使えることを確認する
    let total: i32 = evens_up_to(6).sum();
    assert_eq!(total, 0 + 2 + 4);
}

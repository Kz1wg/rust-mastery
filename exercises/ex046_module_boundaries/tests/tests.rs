use ex046_module_boundaries::order::Order;

#[test]
fn total_with_tax_adds_ten_percent() {
    let o = Order::new(1, 1000);
    assert_eq!(o.total_with_tax(), 1100);
}

#[test]
fn id_is_preserved() {
    assert_eq!(Order::new(42, 100).id(), 42);
}

#[test]
fn zero_total_has_no_tax() {
    assert_eq!(Order::new(1, 0).total_with_tax(), 0);
}

#[test]
fn tax_is_truncated_toward_zero() {
    // 5 / 10 = 0（整数除算）
    assert_eq!(Order::new(1, 5).total_with_tax(), 5);
}

use ex005_invalid_states::{all_statuses, label, Light, OrderStatus};
use std::collections::HashSet;

#[test]
fn light_cycles_green_yellow_red() {
    assert_eq!(Light::Green.next(), Light::Yellow);
    assert_eq!(Light::Yellow.next(), Light::Red);
    assert_eq!(Light::Red.next(), Light::Green);
}

#[test]
fn light_full_cycle_returns_to_start() {
    let start = Light::Red;
    let after_three = start.next().next().next();
    assert_eq!(after_three, start);
}

#[test]
fn order_status_has_exactly_four_distinct_states() {
    let statuses = all_statuses();
    assert_eq!(statuses.len(), 4);

    let unique_labels: HashSet<&'static str> = statuses.iter().map(|s| label(*s)).collect();
    assert_eq!(unique_labels.len(), 4);
}

#[test]
fn order_status_labels_are_distinct_and_named() {
    assert_eq!(label(OrderStatus::Pending), "Pending");
    assert_eq!(label(OrderStatus::Paid), "Paid");
    assert_eq!(label(OrderStatus::Shipped), "Shipped");
    assert_eq!(label(OrderStatus::Delivered), "Delivered");
}

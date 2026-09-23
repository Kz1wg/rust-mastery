use ex054_macro_rules::{hashmap, Grams, Meters, Seconds};
use std::collections::HashMap;

#[test]
fn hashmap_builds_entries() {
    let m: HashMap<&str, i32> = hashmap! { "a" => 1, "b" => 2 };
    assert_eq!(m.len(), 2);
    assert_eq!(m["a"], 1);
    assert_eq!(m["b"], 2);
}

#[test]
fn hashmap_allows_trailing_comma() {
    let m: HashMap<&str, i32> = hashmap! {
        "x" => 10,
        "y" => 20,
    };
    assert_eq!(m.len(), 2);
}

#[test]
fn hashmap_with_no_entries() {
    let m: HashMap<&str, i32> = hashmap! {};
    assert!(m.is_empty());
}

#[test]
fn impl_unit_generates_types_with_display() {
    assert_eq!(Meters(3.0).to_string(), "3m");
    assert_eq!(Seconds(1.5).to_string(), "1.5s");
    assert_eq!(Grams(250.0).to_string(), "250g");
}

#[test]
fn generated_types_are_distinct() {
    // 同じ f64 を包んでいても、Meters と Seconds は別の型（02-2 の newtype）
    let m = Meters(1.0);
    let s = Seconds(1.0);
    assert_eq!(m.0, s.0);
}

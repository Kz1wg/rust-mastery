use ex008_visibility::{Inventory, InventoryError, Temperature, TemperatureError};

#[test]
fn temperature_accepts_absolute_zero() {
    assert_eq!(Temperature::new(-273.15).map(|t| t.celsius()), Ok(-273.15));
}

#[test]
fn temperature_rejects_below_absolute_zero() {
    assert_eq!(Temperature::new(-274.0), Err(TemperatureError));
}

#[test]
fn temperature_accepts_normal_values() {
    assert_eq!(Temperature::new(20.0).map(|t| t.celsius()), Ok(20.0));
}

#[test]
fn set_celsius_rejects_and_keeps_old_value() {
    let mut t = Temperature::new(20.0).unwrap();
    assert_eq!(t.set_celsius(-300.0), Err(TemperatureError));
    assert_eq!(t.celsius(), 20.0); // 変更されていない
}

#[test]
fn set_celsius_accepts_valid_value() {
    let mut t = Temperature::new(20.0).unwrap();
    assert_eq!(t.set_celsius(-100.0), Ok(()));
    assert_eq!(t.celsius(), -100.0);
}

#[test]
fn inventory_starts_with_no_reservation() {
    let inv = Inventory::new(10);
    assert_eq!(inv.quantity(), 10);
    assert_eq!(inv.reserved(), 0);
    assert_eq!(inv.available(), 10);
}

#[test]
fn inventory_reserve_reduces_available() {
    let mut inv = Inventory::new(10);
    assert_eq!(inv.reserve(4), Ok(()));
    assert_eq!(inv.available(), 6);
}

#[test]
fn inventory_reserve_over_quantity_fails_and_keeps_state() {
    let mut inv = Inventory::new(10);
    inv.reserve(8).unwrap();
    assert_eq!(inv.reserve(3), Err(InventoryError)); // 8 + 3 > 10
    assert_eq!(inv.reserved(), 8); // 変更されていない
}

#[test]
fn inventory_release_reduces_reserved() {
    let mut inv = Inventory::new(10);
    inv.reserve(5).unwrap();
    assert_eq!(inv.release(2), Ok(()));
    assert_eq!(inv.reserved(), 3);
    assert_eq!(inv.available(), 7);
}

#[test]
fn inventory_release_more_than_reserved_fails_and_keeps_state() {
    let mut inv = Inventory::new(10);
    inv.reserve(3).unwrap();
    assert_eq!(inv.release(5), Err(InventoryError));
    assert_eq!(inv.reserved(), 3); // 変更されていない
}

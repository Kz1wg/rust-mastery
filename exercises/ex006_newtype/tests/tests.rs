use ex006_newtype::{add_meters, Feet, Meters};

#[test]
fn add_meters_sums_values() {
    let a = Meters::new(1.5);
    let b = Meters::new(2.5);
    assert_eq!(add_meters(a, b).value(), 4.0);
}

#[test]
fn add_meters_with_zero() {
    let a = Meters::new(0.0);
    let b = Meters::new(3.0);
    assert_eq!(add_meters(a, b).value(), 3.0);
}

#[test]
fn feet_to_meters_conversion() {
    let one_foot = Feet::new(1.0);
    let meters: Meters = one_foot.into();
    assert!((meters.value() - 0.3048).abs() < 1e-9);
}

#[test]
fn ten_feet_to_meters() {
    let ten_feet = Feet::new(10.0);
    let meters = Meters::from(ten_feet);
    assert!((meters.value() - 3.048).abs() < 1e-9);
}

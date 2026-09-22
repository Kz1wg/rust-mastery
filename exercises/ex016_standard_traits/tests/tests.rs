use ex016_standard_traits::{Password, Percentage, PercentageError};

#[test]
fn percentage_try_from_valid_string() {
    let p = Percentage::try_from("50").unwrap();
    assert_eq!(p.value(), 50);
}

#[test]
fn percentage_try_from_boundary_values() {
    assert_eq!(Percentage::try_from("0").unwrap().value(), 0);
    assert_eq!(Percentage::try_from("100").unwrap().value(), 100);
}

#[test]
fn percentage_try_from_out_of_range() {
    assert_eq!(Percentage::try_from("101"), Err(PercentageError));
}

#[test]
fn percentage_try_from_non_numeric_string() {
    assert_eq!(Percentage::try_from("abc"), Err(PercentageError));
}

#[test]
fn percentage_try_into_also_works() {
    let p: Result<Percentage, _> = "75".try_into();
    assert_eq!(p.unwrap().value(), 75);
}

#[test]
fn password_can_be_verified_without_exposing_it() {
    let p = Password::new("hunter2");
    assert!(p.verify("hunter2"));
    assert!(!p.verify("wrong"));
}

#[test]
fn password_debug_does_not_leak_the_value() {
    let p = Password::new("hunter2");
    let debug_output = format!("{p:?}");
    assert!(!debug_output.contains("hunter2"));
}

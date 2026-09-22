use ex007_parse_dont_validate::{describe, NonEmptyString, Percentage, PercentageError};

#[test]
fn percentage_accepts_boundary_values() {
    assert_eq!(Percentage::new(0).map(|p| p.value()), Ok(0));
    assert_eq!(Percentage::new(100).map(|p| p.value()), Ok(100));
}

#[test]
fn percentage_rejects_out_of_range() {
    assert_eq!(Percentage::new(101), Err(PercentageError));
    assert_eq!(Percentage::new(255), Err(PercentageError));
}

#[test]
fn describe_formats_as_percentage() {
    let p = Percentage::new(50).unwrap();
    assert_eq!(describe(p), "50%");

    let zero = Percentage::new(0).unwrap();
    assert_eq!(describe(zero), "0%");
}

#[test]
fn non_empty_string_accepts_non_empty() {
    let s = NonEmptyString::new("hello".to_string()).unwrap();
    assert_eq!(s.as_str(), "hello");
}

#[test]
fn non_empty_string_rejects_empty() {
    assert!(NonEmptyString::new(String::new()).is_none());
}

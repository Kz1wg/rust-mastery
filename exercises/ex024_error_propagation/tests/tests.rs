use ex024_error_propagation::{parse_two_numbers, MyError};

#[test]
fn sums_two_valid_numbers() {
    assert_eq!(parse_two_numbers("3", "4"), Ok(7));
}

#[test]
fn negative_numbers_work() {
    assert_eq!(parse_two_numbers("-5", "10"), Ok(5));
}

#[test]
fn first_invalid_produces_error_via_from() {
    let expected_source = "abc".parse::<i32>().unwrap_err();
    assert_eq!(
        parse_two_numbers("abc", "4"),
        Err(MyError {
            source: expected_source
        })
    );
}

#[test]
fn second_invalid_produces_error_via_from() {
    let expected_source = "xyz".parse::<i32>().unwrap_err();
    assert_eq!(
        parse_two_numbers("3", "xyz"),
        Err(MyError {
            source: expected_source
        })
    );
}

/// From の実装自体を直接テストする（? を介さずに）。
#[test]
fn from_impl_wraps_the_parse_error() {
    let parse_err = "notanumber".parse::<i32>().unwrap_err();
    let my_err = MyError::from(parse_err.clone());
    assert_eq!(my_err, MyError { source: parse_err });
}

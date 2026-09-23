use ex052_testing_failures::{get_item, parse_age, AgeError};

#[test]
fn valid_ages() {
    assert_eq!(parse_age("42"), Ok(42));
    assert_eq!(parse_age("  7 "), Ok(7));
}

#[test]
fn boundary_values() {
    assert_eq!(parse_age("0"), Ok(0));
    assert_eq!(parse_age("150"), Ok(150));
    assert_eq!(parse_age("151"), Err(AgeError::TooOld));
    assert_eq!(parse_age("255"), Err(AgeError::TooOld));
}

/// どの種類の Err か、まで確かめる。is_err() だけでは区別できない。
#[test]
fn each_kind_of_error() {
    assert_eq!(parse_age(""), Err(AgeError::Empty));
    assert_eq!(parse_age("   "), Err(AgeError::Empty));
    assert_eq!(parse_age("abc"), Err(AgeError::NotANumber));
    assert_eq!(parse_age("-1"), Err(AgeError::NotANumber));
}

/// u8 に入らない数は、範囲チェックの前に parse の時点で失敗する。
#[test]
fn numbers_too_large_for_u8_are_not_a_number() {
    assert_eq!(parse_age("256"), Err(AgeError::NotANumber));
    assert_eq!(parse_age("300"), Err(AgeError::NotANumber));
}

#[test]
fn get_item_in_range() {
    assert_eq!(get_item(&[10, 20, 30], 1), 20);
}

/// expected を付けているので、「別の理由」の panic では通らない。
#[test]
#[should_panic(expected = "index out of range")]
fn get_item_out_of_range_panics_with_message() {
    get_item(&[1, 2, 3], 10);
}

#[test]
#[should_panic(expected = "index out of range")]
fn get_item_on_empty_slice_panics_with_message() {
    get_item(&[], 0);
}

//! 主なテストはドキュメントのコード例（doctest）にある。ここでは補足の確認だけをする。

use ex061_documentation::{parse_duration, DurationError};

#[test]
fn single_units() {
    assert_eq!(parse_duration("1h"), Ok(3600));
    assert_eq!(parse_duration("1m"), Ok(60));
    assert_eq!(parse_duration("1s"), Ok(1));
}

#[test]
fn all_units_combined() {
    assert_eq!(parse_duration("1h1m1s"), Ok(3661));
}

#[test]
fn trailing_number_is_missing_unit() {
    assert_eq!(parse_duration("1h30"), Err(DurationError::MissingUnit));
}

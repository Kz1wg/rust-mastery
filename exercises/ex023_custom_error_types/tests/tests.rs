use ex023_custom_error_types::{read_number_from_str, ReadNumberError};

#[test]
fn parses_valid_number() {
    assert_eq!(read_number_from_str("42").unwrap(), 42);
}

#[test]
fn parse_error_produces_parse_variant() {
    let err = read_number_from_str("abc").unwrap_err();
    assert!(matches!(err, ReadNumberError::Parse(_)));
}

#[test]
fn display_for_parse_error_mentions_invalid_number() {
    let err = read_number_from_str("abc").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("invalid number"));
}

#[test]
fn display_for_io_error_mentions_input_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
    let err = ReadNumberError::from(io_err);
    let msg = err.to_string();
    assert!(msg.contains("input error"));
}

#[test]
fn implements_std_error_error() {
    fn assert_is_error<E: std::error::Error>() {}
    assert_is_error::<ReadNumberError>();
}

#[test]
fn converts_into_boxed_error() {
    let err = read_number_from_str("abc").unwrap_err();
    let _boxed: Box<dyn std::error::Error> = Box::new(err);
}

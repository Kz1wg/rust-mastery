use ex022_result_design::{parse_config, ConfigError};

#[test]
fn parses_valid_number() {
    assert_eq!(parse_config("42"), Ok(42));
}

#[test]
fn parses_number_with_whitespace() {
    assert_eq!(parse_config("  42  "), Ok(42));
}

#[test]
fn empty_string_is_empty_error() {
    assert_eq!(parse_config(""), Err(ConfigError::Empty));
}

#[test]
fn whitespace_only_is_empty_error() {
    assert_eq!(parse_config("   "), Err(ConfigError::Empty));
}

#[test]
fn non_numeric_string_is_not_a_number_error() {
    assert_eq!(
        parse_config("abc"),
        Err(ConfigError::NotANumber("abc".to_string()))
    );
}

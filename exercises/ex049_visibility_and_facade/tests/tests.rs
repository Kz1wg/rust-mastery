use ex049_visibility_and_facade::{parse_config, Config};

#[test]
fn parses_simple_entries() {
    let config = parse_config("host=localhost\nport=8080");
    assert_eq!(config.get("host"), Some("localhost"));
    assert_eq!(config.get("port"), Some("8080"));
    assert_eq!(config.len(), 2);
}

#[test]
fn keys_are_normalized() {
    let config = parse_config("  HOST  = localhost ");
    assert_eq!(config.get("host"), Some("localhost"));
    assert_eq!(config.get("  HoSt "), Some("localhost"));
}

#[test]
fn blank_lines_are_ignored() {
    let config = parse_config("a=1\n\n\nb=2\n");
    assert_eq!(config.len(), 2);
}

#[test]
fn missing_keys_return_none() {
    let config = parse_config("a=1");
    assert_eq!(config.get("zzz"), None);
}

#[test]
fn empty_input_produces_empty_config() {
    let config: Config = parse_config("");
    assert!(config.is_empty());
}

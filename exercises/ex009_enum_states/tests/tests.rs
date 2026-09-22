use ex009_enum_states::Connection;

#[test]
fn new_connection_is_disconnected_with_zero_retries() {
    let c = Connection::new();
    assert!(!c.is_connected());
    assert_eq!(c.retry_count(), 0);
}

#[test]
fn connect_makes_it_connected() {
    let c = Connection::new().connect();
    assert!(c.is_connected());
    assert_eq!(c.retry_count(), 0);
}

#[test]
fn fail_increments_retry_count_and_disconnects() {
    let c = Connection::new().fail("timeout".to_string());
    assert!(!c.is_connected());
    assert_eq!(c.retry_count(), 1);
}

#[test]
fn fail_after_connect_still_increments_retry_count() {
    let c = Connection::new().connect().fail("dropped".to_string());
    assert!(!c.is_connected());
    assert_eq!(c.retry_count(), 1);
}

#[test]
fn multiple_failures_accumulate_retry_count() {
    let c = Connection::new()
        .fail("a".to_string())
        .connect()
        .fail("b".to_string())
        .fail("c".to_string());
    assert_eq!(c.retry_count(), 3);
    assert!(!c.is_connected());
}

#[test]
fn connect_after_failure_resets_connected_flag_but_keeps_retry_count() {
    let c = Connection::new().fail("x".to_string()).connect();
    assert!(c.is_connected());
    assert_eq!(c.retry_count(), 1);
}

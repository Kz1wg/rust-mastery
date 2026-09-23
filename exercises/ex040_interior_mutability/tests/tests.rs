use ex040_interior_mutability::{Counter, Logger};

#[test]
fn counter_increments_through_shared_reference() {
    let c = Counter::new();
    let shared: &Counter = &c; // &mut ではない
    shared.increment();
    shared.increment();
    assert_eq!(shared.get(), 2);
}

#[test]
fn counter_starts_at_zero() {
    assert_eq!(Counter::new().get(), 0);
}

#[test]
fn logger_records_messages() {
    let l = Logger::new();
    l.log("first");
    l.log("second");
    assert_eq!(l.count(), 2);
    assert_eq!(l.entries(), vec!["first".to_string(), "second".to_string()]);
}

#[test]
fn logger_starts_empty() {
    let l = Logger::new();
    assert_eq!(l.count(), 0);
    assert!(l.entries().is_empty());
}

#[test]
fn try_log_succeeds_when_not_borrowed() {
    let l = Logger::new();
    assert!(l.try_log("ok"));
    assert_eq!(l.count(), 1);
}

/// 複数の不変参照から同時に使える（&mut を要求しない設計の利点）。
#[test]
fn multiple_shared_references_can_log() {
    let l = Logger::new();
    let a: &Logger = &l;
    let b: &Logger = &l;
    a.log("from a");
    b.log("from b");
    assert_eq!(l.count(), 2);
}

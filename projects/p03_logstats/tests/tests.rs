use p03_logstats::{parse_line, summarize, Level, LogEntry, ModuleErrors, ParseError};

#[test]
fn level_from_str() {
    assert_eq!("INFO".parse::<Level>(), Ok(Level::Info));
    assert_eq!("ERROR".parse::<Level>(), Ok(Level::Error));
    assert_eq!("info".parse::<Level>(), Err("info".to_string()));
}

#[test]
fn levels_are_ordered_by_severity() {
    assert!(Level::Debug < Level::Info);
    assert!(Level::Warn < Level::Error);
}

#[test]
fn parse_a_normal_line() {
    let e = parse_line("2026-09-22T10:00:00 INFO [auth] user logged in", 1).unwrap();
    assert_eq!(
        e,
        LogEntry {
            timestamp: "2026-09-22T10:00:00".into(),
            level: Level::Info,
            module: "auth".into(),
            message: "user logged in".into(),
        }
    );
}

#[test]
fn message_may_be_empty() {
    let e = parse_line("t WARN [cache]", 1).unwrap();
    assert_eq!(e.module, "cache");
    assert_eq!(e.message, "");
}

#[test]
fn parse_errors_carry_the_line_number() {
    assert_eq!(
        parse_line("t FATAL [x] boom", 4),
        Err(ParseError::UnknownLevel {
            line: 4,
            level: "FATAL".into()
        })
    );
    assert_eq!(
        parse_line("t INFO auth msg", 5),
        Err(ParseError::BadModule { line: 5 })
    );
    assert_eq!(
        parse_line("t INFO [auth msg", 6),
        Err(ParseError::BadModule { line: 6 })
    );
    assert_eq!(
        parse_line("t", 7),
        Err(ParseError::MissingField {
            line: 7,
            field: "level"
        })
    );
    assert_eq!(
        parse_line("t INFO", 8),
        Err(ParseError::MissingField {
            line: 8,
            field: "module"
        })
    );
}

const LOG: &str = "\
2026-09-22T10:00:00 INFO [auth] login
2026-09-22T10:00:01 ERROR [db] timeout

2026-09-22T10:00:02 ERROR [db] refused
this line is broken
2026-09-22T10:00:03 ERROR [auth] bad token
2026-09-22T10:00:04 DEBUG [db] retry
";

#[test]
fn summarize_counts_and_keeps_going_after_bad_lines() {
    let s = summarize(LOG.as_bytes()).unwrap();
    assert_eq!(s.total, 5);
    assert_eq!(s.by_level.get(&Level::Error), Some(&3));
    assert_eq!(s.by_level.get(&Level::Info), Some(&1));
    assert_eq!(s.by_level.get(&Level::Warn), None);
    // 解析できなかった行は、止めずに記録される（空行も数えた行番号）
    assert_eq!(
        s.invalid,
        vec![ParseError::UnknownLevel {
            line: 5,
            level: "line".into()
        }]
    );
}

#[test]
fn errors_by_module_are_sorted() {
    let s = summarize(LOG.as_bytes()).unwrap();
    assert_eq!(
        s.errors_by_module,
        vec![
            ModuleErrors {
                module: "db".into(),
                errors: 2
            },
            ModuleErrors {
                module: "auth".into(),
                errors: 1
            },
        ]
    );
}

#[test]
fn error_rate() {
    let s = summarize(LOG.as_bytes()).unwrap();
    assert!((s.error_rate() - 0.6).abs() < 1e-9);
    let empty = summarize("".as_bytes()).unwrap();
    assert_eq!(empty.error_rate(), 0.0);
}

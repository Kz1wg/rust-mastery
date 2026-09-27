use p07_iniconf::{Document, GetError, ParseError, ParseErrorKind, Section};

const SAMPLE: &str = "\
; 先頭のコメント
name = demo
debug = true

[server]
host = 127.0.0.1
port = 8080

# セクションの間のコメント
[database]
url = postgres://localhost/app?sslmode=disable
pool = 4
";

fn doc() -> Document {
    Document::parse(SAMPLE).expect("SAMPLE は正しい INI")
}

fn parse_err(input: &str) -> ParseError {
    Document::parse(input).expect_err("解析に失敗するはず")
}

// ---------------------------------------------------------------------------
// 解析: 正しい入力
// ---------------------------------------------------------------------------

#[test]
fn get_values_from_sections() {
    let doc = doc();
    assert_eq!(doc.get("server", "host"), Some("127.0.0.1"));
    assert_eq!(doc.get("server", "port"), Some("8080"));
    assert_eq!(doc.get("database", "pool"), Some("4"));
}

#[test]
fn keys_before_any_section_go_to_root() {
    let doc = doc();
    assert_eq!(doc.get("", "name"), Some("demo"));
    assert_eq!(doc.get("", "debug"), Some("true"));
}

#[test]
fn missing_section_or_key_is_none() {
    let doc = doc();
    assert_eq!(doc.get("server", "nope"), None);
    assert_eq!(doc.get("nope", "host"), None);
    // キーはセクションごとに別。server に url はない
    assert_eq!(doc.get("server", "url"), None);
}

#[test]
fn value_may_contain_equals_sign() {
    // 最初の = だけで分ける
    let doc = doc();
    assert_eq!(
        doc.get("database", "url"),
        Some("postgres://localhost/app?sslmode=disable")
    );
}

#[test]
fn whitespace_is_trimmed() {
    let doc = Document::parse("  [  web  ]  \n   key   =   some value   \n").unwrap();
    assert_eq!(doc.get("web", "key"), Some("some value"));
}

#[test]
fn empty_value_is_allowed() {
    let doc = Document::parse("[a]\nkey =\n").unwrap();
    assert_eq!(doc.get("a", "key"), Some(""));
}

#[test]
fn empty_input_is_an_empty_document() {
    let doc = Document::parse("").unwrap();
    assert_eq!(doc.sections().count(), 0);
    assert!(doc.section("").unwrap().is_empty());
}

#[test]
fn crlf_line_endings_are_accepted() {
    let doc = Document::parse("[a]\r\nkey = v\r\n").unwrap();
    assert_eq!(doc.get("a", "key"), Some("v"));
}

#[test]
fn from_str_is_the_same_as_parse() {
    let a: Document = SAMPLE.parse().unwrap();
    assert_eq!(a, doc());
}

// ---------------------------------------------------------------------------
// 解析: 失敗
// ---------------------------------------------------------------------------

#[test]
fn unclosed_section() {
    let e = parse_err("a = 1\n[server\n");
    assert_eq!(e.line(), 2);
    assert_eq!(e.kind(), &ParseErrorKind::UnclosedSection);
}

#[test]
fn empty_section_name() {
    assert_eq!(parse_err("[]").kind(), &ParseErrorKind::EmptySectionName);
    assert_eq!(parse_err("[   ]").kind(), &ParseErrorKind::EmptySectionName);
}

#[test]
fn missing_equals() {
    let e = parse_err("[a]\n\njust words\n");
    assert_eq!(e.line(), 3);
    assert_eq!(e.kind(), &ParseErrorKind::MissingEquals);
}

#[test]
fn empty_key() {
    assert_eq!(parse_err("[a]\n = 1").kind(), &ParseErrorKind::EmptyKey);
}

#[test]
fn duplicate_key_in_same_section() {
    let e = parse_err("[a]\nk = 1\nk = 2\n");
    assert_eq!(e.line(), 3);
    assert_eq!(e.kind(), &ParseErrorKind::DuplicateKey("k".to_string()));
}

#[test]
fn same_key_in_different_sections_is_fine() {
    let doc = Document::parse("k = 0\n[a]\nk = 1\n[b]\nk = 2\n").unwrap();
    assert_eq!(doc.get("", "k"), Some("0"));
    assert_eq!(doc.get("a", "k"), Some("1"));
    assert_eq!(doc.get("b", "k"), Some("2"));
}

#[test]
fn duplicate_section() {
    let e = parse_err("[a]\nk = 1\n[b]\n[a]\n");
    assert_eq!(e.line(), 4);
    assert_eq!(e.kind(), &ParseErrorKind::DuplicateSection("a".to_string()));
}

#[test]
fn parse_error_message_mentions_the_line() {
    let e = parse_err("[a]\nk = 1\nk = 2\n");
    assert_eq!(e.to_string(), "3行目: キー `k` が重複しています");
}

// ---------------------------------------------------------------------------
// get_parsed
// ---------------------------------------------------------------------------

#[test]
fn get_parsed_converts_to_the_requested_type() {
    let doc = doc();
    let port: u16 = doc.get_parsed("server", "port").unwrap();
    assert_eq!(port, 8080);
    let debug: bool = doc.get_parsed("", "debug").unwrap();
    assert!(debug);
    let host: std::net::IpAddr = doc.get_parsed("server", "host").unwrap();
    assert!(host.is_loopback());
}

#[test]
fn get_parsed_missing() {
    let r: Result<u16, _> = doc().get_parsed("server", "timeout");
    assert_eq!(
        r,
        Err(GetError::Missing {
            section: "server".to_string(),
            key: "timeout".to_string(),
        })
    );
}

#[test]
fn get_parsed_invalid() {
    // "127.0.0.1" は u16 にならない
    let r: Result<u16, _> = doc().get_parsed("server", "host");
    assert_eq!(
        r,
        Err(GetError::Invalid {
            section: "server".to_string(),
            key: "host".to_string(),
            value: "127.0.0.1".to_string(),
        })
    );
}

// ---------------------------------------------------------------------------
// Section（中身を借りて覗く型）
// ---------------------------------------------------------------------------

#[test]
fn sections_are_in_file_order_and_exclude_root() {
    let doc = doc();
    let names: Vec<&str> = doc.sections().map(|s| s.name()).collect();
    assert_eq!(names, vec!["server", "database"]);
}

#[test]
fn section_entries_are_in_file_order() {
    let doc = doc();
    let server = doc.section("server").unwrap();
    let entries: Vec<(&str, &str)> = server.entries().collect();
    assert_eq!(entries, vec![("host", "127.0.0.1"), ("port", "8080")]);
    assert_eq!(server.len(), 2);
    assert!(!server.is_empty());
}

#[test]
fn root_section_always_exists() {
    let doc = Document::parse("[a]\nk = 1\n").unwrap();
    let root = doc.section("").unwrap();
    assert_eq!(root.name(), "");
    assert!(root.is_empty());
    assert!(doc.section("b").is_none());
}

#[test]
fn values_borrowed_from_section_outlive_the_section_value() {
    // Section は一時的な値でも、取り出した &str は Document が生きている間使える
    let doc = doc();
    let host: &str = {
        let server: Section<'_> = doc.section("server").unwrap();
        server.get("host").unwrap()
    };
    assert_eq!(host, "127.0.0.1");
}

// ---------------------------------------------------------------------------
// 書き出し（Display）
// ---------------------------------------------------------------------------

#[test]
fn display_writes_normalized_ini() {
    let doc = Document::parse("; c\n a=1 \n[x]\nk=v\n\n\n[y]\n").unwrap();
    assert_eq!(doc.to_string(), "a = 1\n\n[x]\nk = v\n\n[y]\n");
}

#[test]
fn display_without_root_entries() {
    let doc = Document::parse("[x]\nk = v\n").unwrap();
    assert_eq!(doc.to_string(), "[x]\nk = v\n");
}

#[test]
fn display_then_parse_round_trips() {
    let original = doc();
    let again = Document::parse(&original.to_string()).unwrap();
    assert_eq!(again, original);
}

// ---------------------------------------------------------------------------
// ライブラリとしての約束（Lesson 16-1, 16-2）
// ---------------------------------------------------------------------------

#[test]
fn public_types_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Document>();
    assert_send_sync::<Section<'static>>();
    assert_send_sync::<ParseError>();
    assert_send_sync::<GetError>();
}

#[test]
fn errors_work_with_question_mark_into_box_dyn_error() {
    // アプリケーション側でよくある書き方で、両方のエラーを ? で扱えること
    fn load(text: &str) -> Result<u16, Box<dyn std::error::Error + Send + Sync>> {
        let doc: Document = text.parse()?;
        Ok(doc.get_parsed("server", "port")?)
    }
    assert_eq!(load("[server]\nport = 80").unwrap(), 80);
    assert!(load("[server").is_err());
    assert!(load("[server]\nport = x").is_err());
}

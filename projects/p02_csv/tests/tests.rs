use p02_csv::{parse_line, CsvError, CsvReader};

fn strings(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

// ---------- parse_line（状態機械） ----------

#[test]
fn simple_fields() {
    assert_eq!(parse_line("a,b,c", 1), Ok(strings(&["a", "b", "c"])));
}

#[test]
fn empty_fields() {
    assert_eq!(parse_line("a,,c", 1), Ok(strings(&["a", "", "c"])));
    assert_eq!(parse_line("a,", 1), Ok(strings(&["a", ""])));
    assert_eq!(parse_line("", 1), Ok(strings(&[""])));
}

#[test]
fn quoted_field_can_contain_commas() {
    assert_eq!(
        parse_line(r#""Tokyo, Japan",100"#, 1),
        Ok(strings(&["Tokyo, Japan", "100"]))
    );
}

#[test]
fn doubled_quote_is_a_literal_quote() {
    assert_eq!(
        parse_line(r#""say ""hi""",x"#, 1),
        Ok(strings(&[r#"say "hi""#, "x"]))
    );
}

#[test]
fn unterminated_quote_reports_the_line() {
    assert_eq!(
        parse_line(r#"a,"open"#, 7),
        Err(CsvError::UnterminatedQuote { line: 7 })
    );
}

#[test]
fn quote_in_the_middle_of_an_unquoted_field() {
    assert_eq!(
        parse_line(r#"ab"c,d"#, 3),
        Err(CsvError::UnexpectedQuote { line: 3, column: 3 })
    );
}

#[test]
fn character_after_closing_quote() {
    assert_eq!(
        parse_line(r#""ab"c,d"#, 2),
        Err(CsvError::CharAfterClosingQuote { line: 2, column: 5 })
    );
}

#[test]
fn non_ascii_columns_are_counted_in_characters() {
    // 「東京」は2文字。列番号はバイトではなく文字で数える
    assert_eq!(
        parse_line(r#"東京"x"#, 1),
        Err(CsvError::UnexpectedQuote { line: 1, column: 3 })
    );
}

// ---------- CsvReader（イテレータ） ----------

const SAMPLE: &str = "name,city,score\nalice,\"Tokyo, JP\",90\n\nbob,Osaka,85\n";

#[test]
fn reader_reads_headers_and_records() {
    let reader = CsvReader::new(SAMPLE.as_bytes()).unwrap();
    assert_eq!(reader.headers(), &strings(&["name", "city", "score"])[..]);

    let records: Vec<_> = reader.collect::<Result<_, _>>().unwrap();
    assert_eq!(records.len(), 2); // 空行は読み飛ばす
    assert_eq!(records[0].get("city"), Some("Tokyo, JP"));
    assert_eq!(records[1].get("name"), Some("bob"));
    assert_eq!(records[1].get("unknown"), None);
}

#[test]
fn field_count_mismatch_reports_the_real_line_number() {
    // 4行目（空行を含めて数える）のフィールドが足りない
    let input = "a,b\n1,2\n\n3\n";
    let results: Vec<_> = CsvReader::new(input.as_bytes()).unwrap().collect();
    assert_eq!(results.len(), 2);
    assert!(results[0].is_ok());
    assert_eq!(
        results[1].as_ref().unwrap_err(),
        &CsvError::FieldCountMismatch {
            line: 4,
            expected: 2,
            found: 1
        }
    );
}

#[test]
fn empty_input_has_no_header() {
    assert!(matches!(
        CsvReader::new("".as_bytes()),
        Err(CsvError::MissingHeader)
    ));
    assert!(matches!(
        CsvReader::new("\n\n".as_bytes()),
        Err(CsvError::MissingHeader)
    ));
}

#[test]
fn records_share_the_same_headers() {
    // 何件読んでも、ヘッダーは Rc で共有されている（値の取り出しで確認）
    let input = "k\n1\n2\n3\n";
    let values: Vec<String> = CsvReader::new(input.as_bytes())
        .unwrap()
        .map(|r| r.unwrap().get("k").unwrap().to_string())
        .collect();
    assert_eq!(values, strings(&["1", "2", "3"]));
}

use ex047_lib_and_bin::{count_words, run};

#[test]
fn count_words_basic() {
    assert_eq!(count_words("hello big world"), 3);
}

#[test]
fn count_words_handles_extra_whitespace_and_empty() {
    assert_eq!(count_words("  a   b  "), 2);
    assert_eq!(count_words(""), 0);
}

#[test]
fn run_reads_a_file_and_reports_the_count() {
    let path = std::env::temp_dir().join("ex047_test_input.txt");
    std::fs::write(&path, "one two three").unwrap();
    let result = run(path.to_str().unwrap());
    std::fs::remove_file(&path).ok();
    assert_eq!(result.unwrap(), "3 words");
}

#[test]
fn run_returns_err_for_missing_file() {
    let result = run("/definitely/does/not/exist.txt");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
}

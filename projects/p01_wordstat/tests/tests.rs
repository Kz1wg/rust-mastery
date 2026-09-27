use p01_wordstat::{count_words, normalize, parse_args, run, Args, CliError, WordCount};

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

fn wc(word: &str, count: usize) -> WordCount {
    WordCount {
        word: word.to_string(),
        count,
    }
}

// ---------- parse_args ----------

#[test]
fn parse_path_only_uses_defaults() {
    let a = parse_args(&args(&["notes.txt"])).unwrap();
    assert_eq!(
        a,
        Args {
            path: "notes.txt".into(),
            top: 10,
            ignore_case: false
        }
    );
}

#[test]
fn parse_all_options_in_any_order() {
    let a = parse_args(&args(&["--ignore-case", "notes.txt", "--top", "3"])).unwrap();
    assert_eq!(
        a,
        Args {
            path: "notes.txt".into(),
            top: 3,
            ignore_case: true
        }
    );
}

#[test]
fn parse_missing_path() {
    assert!(matches!(
        parse_args(&args(&["--top", "3"])),
        Err(CliError::MissingPath)
    ));
}

#[test]
fn parse_unknown_option() {
    let e = parse_args(&args(&["--foo", "a.txt"])).unwrap_err();
    assert!(matches!(e, CliError::UnknownOption(o) if o == "--foo"));
}

#[test]
fn parse_invalid_top_values() {
    assert!(
        matches!(parse_args(&args(&["a.txt", "--top", "x"])), Err(CliError::InvalidTop(v)) if v == "x")
    );
    assert!(
        matches!(parse_args(&args(&["a.txt", "--top", "0"])), Err(CliError::InvalidTop(v)) if v == "0")
    );
    // 値が無い
    assert!(
        matches!(parse_args(&args(&["a.txt", "--top"])), Err(CliError::InvalidTop(v)) if v.is_empty())
    );
}

#[test]
fn parse_two_paths_is_an_error() {
    let e = parse_args(&args(&["a.txt", "b.txt"])).unwrap_err();
    assert!(matches!(e, CliError::UnexpectedArgument(a) if a == "b.txt"));
}

// ---------- normalize / count_words ----------

#[test]
fn normalize_strips_surrounding_symbols() {
    assert_eq!(normalize("Hello,", false), Some("Hello".to_string()));
    assert_eq!(normalize("(world)", false), Some("world".to_string()));
    assert_eq!(normalize("Rust!", true), Some("rust".to_string()));
    assert_eq!(normalize("don't", false), Some("don't".to_string()));
    assert_eq!(normalize("---", false), None);
}

#[test]
fn count_words_orders_by_count_then_word() {
    let result = count_words("b a b c a b", false);
    assert_eq!(result, vec![wc("b", 3), wc("a", 2), wc("c", 1)]);
}

#[test]
fn count_words_respects_ignore_case() {
    assert_eq!(count_words("Rust rust RUST", true), vec![wc("rust", 3)]);
    assert_eq!(
        count_words("Rust rust", false),
        vec![wc("Rust", 1), wc("rust", 1)]
    );
}

// ---------- run（ファイルを使わずに、文字列を入力として渡す） ----------

#[test]
fn run_prints_top_n_lines() {
    let a = Args {
        path: "unused".into(),
        top: 2,
        ignore_case: true,
    };
    let input = "The cat and the dog. The END.";
    let out = run(&a, input.as_bytes()).unwrap();
    assert_eq!(out, "the\t3\nand\t1\n");
}

#[test]
fn run_with_fewer_words_than_top() {
    let a = Args {
        path: "unused".into(),
        top: 10,
        ignore_case: false,
    };
    assert_eq!(run(&a, "one".as_bytes()).unwrap(), "one\t1\n");
}

#[test]
fn run_with_empty_input() {
    let a = Args {
        path: "unused".into(),
        top: 5,
        ignore_case: false,
    };
    assert_eq!(run(&a, "".as_bytes()).unwrap(), "");
}

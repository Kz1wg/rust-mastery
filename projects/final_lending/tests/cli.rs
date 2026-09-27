//! 引数の解釈と、バイナリを実際に動かすテスト（R9, R10）。

use std::path::PathBuf;
use std::process::Command as Process;

use final_lending::cli::{parse_args, Command};
use final_lending::{BookId, Date};

fn args(s: &[&str]) -> Vec<String> {
    s.iter().map(|a| a.to_string()).collect()
}

#[test]
fn parse_command_and_options_in_any_order() {
    let inv = parse_args(&args(&[
        "return",
        "B1",
        "--today",
        "2026-09-25",
        "--data",
        "x.tsv",
    ]))
    .unwrap();
    assert_eq!(
        inv.command,
        Command::Return {
            book: BookId::new("B1").unwrap()
        }
    );
    assert_eq!(inv.today, Some("2026-09-25".parse::<Date>().unwrap()));
    assert_eq!(inv.data, PathBuf::from("x.tsv"));
}

#[test]
fn defaults() {
    let inv = parse_args(&args(&["list"])).unwrap();
    assert_eq!(inv.data, PathBuf::from("lending.tsv"));
    assert_eq!(inv.today, None);
}

#[test]
fn bad_arguments_are_usage_errors() {
    for a in [
        &[][..],
        &["fly"][..],
        &["borrow", "M1"][..],
        &["borrow", "M 1", "B1"][..],
        &["add-member", "M1", "visitor", "A"][..],
        &["list", "--today", "2026-13-01"][..],
        &["list", "--today"][..],
        &["list", "--verbose"][..],
    ] {
        assert!(parse_args(&args(a)).is_err(), "{a:?} は失敗するはず");
    }
}

/// テストごとに別のデータファイルを使う（テストは並行して走るため）。
fn data_file(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("final_lending_{}_{name}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("lending.tsv");
    let _ = std::fs::remove_file(&path);
    path
}

/// バイナリを動かし、(終了コード, 標準出力, 標準エラー) を返す。
fn lending(data: &PathBuf, a: &[&str]) -> (i32, String, String) {
    let out = Process::new(env!("CARGO_BIN_EXE_lending"))
        .arg("--data")
        .arg(data)
        .args(a)
        .output()
        .unwrap();
    (
        out.status.code().unwrap(),
        String::from_utf8(out.stdout).unwrap(),
        String::from_utf8(out.stderr).unwrap(),
    )
}

#[test]
fn end_to_end_session() {
    let data = data_file("session");
    let today = ["--today", "2026-09-01"];

    assert_eq!(lending(&data, &["add-book", "B1", "Rustの本"]).0, 0);
    assert_eq!(
        lending(&data, &["add-member", "M1", "general", "山田"]).0,
        0
    );

    let (code, out, _) = lending(&data, &[&["borrow", "M1", "B1"][..], &today].concat());
    assert_eq!(code, 0);
    assert!(out.contains("2026-09-15"), "{out}");

    // 別のプロセスから見ても、貸出中になっている（ファイルに保存されている）
    let (_, out, _) = lending(&data, &["list"]);
    assert!(out.contains("貸出中（M1、期限 2026-09-15）"), "{out}");

    let (_, out, _) = lending(&data, &["overdue", "--today", "2026-09-20"]);
    assert!(out.contains("B1\tRustの本\tM1\t5日"), "{out}");

    let (code, out, _) = lending(&data, &["return", "B1", "--today", "2026-09-20"]);
    assert_eq!(code, 0);
    assert!(out.contains("5日遅れ"), "{out}");
}

#[test]
fn rule_violation_exits_with_1_and_keeps_the_file() {
    let data = data_file("violation");
    lending(&data, &["add-book", "B1", "本"]);
    lending(&data, &["add-member", "M1", "general", "A"]);
    let before = std::fs::read_to_string(&data).unwrap();

    let (code, _, err) = lending(&data, &["return", "B1"]);
    assert_eq!(code, 1);
    assert!(err.contains("貸出中ではありません"), "{err}");
    assert_eq!(std::fs::read_to_string(&data).unwrap(), before);
}

#[test]
fn usage_error_exits_with_2() {
    let data = data_file("usage");
    let (code, _, err) = lending(&data, &["borrow"]);
    assert_eq!(code, 2);
    assert!(err.contains("usage:"), "{err}");
}

#[test]
fn broken_data_file_exits_with_3_and_shows_the_line() {
    let data = data_file("broken");
    std::fs::write(&data, "# lending v2\nmember\tM1\n").unwrap();
    let (code, _, err) = lending(&data, &["list"]);
    assert_eq!(code, 3);
    assert!(err.contains("2行目"), "{err}");
}

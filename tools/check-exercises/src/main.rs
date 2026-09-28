//! 演習の進捗確認 / 解答検証ツール（ARCHITECTURE.md §4.4, §4.5）。
//!
//! USAGE:
//!   cargo run -p check-exercises                  全演習の進捗を表示
//!   cargo run -p check-exercises -- --lesson 02   章で絞り込む（lessonの先頭一致）
//!   cargo run -p check-exercises -- --solutions   解答が全て通るか検証する（メンテナ向け）
//!
//! 依存crateはゼロ（標準ライブラリのみ）。exercise.toml / Cargo.toml の読み取りは
//! 自前の最小パーサー（下部の toml_get_string / toml_get_string_array）で行う。
//! これらはTOML全般には対応しない、このプロジェクトが書く形式専用の簡易パーサー。
//! （当初 `toml` クレートを使う予定だったが、その依存先 `indexmap` の新しいバージョンが
//! edition2024を要求し、開発環境のRust 1.75ではビルドできなかったため、
//! 依存を持たない実装に変更した。ARCHITECTURE.md の決定ログを参照）
//!
//! `cargo test` の出力は `--message-format=json` ではなく通常の人間向け出力を
//! 単純な文字列走査で解析する。

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
struct ExerciseMeta {
    id: String,
    lesson: String,
    dir: PathBuf,
}

#[derive(Debug, Clone, Copy)]
enum Category {
    Ok,
    /// todo!() 以外の理由で失敗しているテストがある（数は失敗の総数）。
    Failed(u32),
    /// 失敗しているテストは、すべて todo!() に届いたもの（数は失敗の総数）。
    /// 骨組みのままの演習も、書き途中で残りが todo!() だけの演習も、ここに入る。
    Todo(u32),
    Error,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut lesson_filter: Option<String> = None;
    let mut solutions_mode = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--lesson" => {
                i += 1;
                lesson_filter = args.get(i).cloned();
                if lesson_filter.is_none() {
                    eprintln!("error: --lesson には値が必要です（例: --lesson 02）");
                    std::process::exit(2);
                }
            }
            "--solutions" => solutions_mode = true,
            "--help" | "-h" => {
                print_help();
                return;
            }
            other => {
                eprintln!("error: unknown option: {other}");
                print_help();
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let root = workspace_root();
    let mut exercises = discover_exercises(&root);
    if let Some(filter) = &lesson_filter {
        exercises.retain(|e| e.lesson.starts_with(filter.as_str()));
        if exercises.is_empty() {
            eprintln!("該当する演習がありません（--lesson {filter}）");
            std::process::exit(1);
        }
    }

    if solutions_mode {
        run_solutions_mode(&root, &exercises);
    } else {
        run_progress_mode(&root, &exercises);
    }
}

fn print_help() {
    println!("check-exercises — 演習の進捗確認 / 解答検証ツール");
    println!();
    println!("USAGE:");
    println!("  cargo run -p check-exercises                  全演習の進捗を表示");
    println!("  cargo run -p check-exercises -- --lesson 02   章で絞り込む（lessonの先頭一致）");
    println!(
        "  cargo run -p check-exercises -- --solutions   解答が全て通るか検証する（メンテナ向け）"
    );
}

/// tools/check-exercises から見て、2階層上がworkspace root。
fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("tools/check-exercises はworkspace rootの2階層下にある想定です")
        .to_path_buf()
}

fn discover_exercises(root: &Path) -> Vec<ExerciseMeta> {
    let mut result = Vec::new();
    let exercises_dir = root.join("exercises");
    let Ok(entries) = fs::read_dir(exercises_dir) else {
        return result;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let toml_path = path.join("exercise.toml");
        let Ok(content) = fs::read_to_string(&toml_path) else {
            continue;
        };

        let dir_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let id = toml_get_string(&content, "id").unwrap_or(dir_name);
        let lesson = toml_get_string(&content, "lesson").unwrap_or_else(|| "??".to_string());

        result.push(ExerciseMeta {
            id,
            lesson,
            dir: path,
        });
    }

    result.sort_by(|a, b| a.id.cmp(&b.id));
    result
}

fn chapter_title(lesson: &str) -> String {
    let prefix = lesson.split('-').next().unwrap_or(lesson);
    const TITLES: &[(&str, &str)] = &[
        ("00", "00 Introduction"),
        ("01", "01 Ownership & Borrowing"),
        ("02", "02 Type Design"),
        ("03", "03 Enum & State Machine"),
        ("04", "04 Traits"),
        ("05", "05 Generics"),
        ("06", "06 Error Handling"),
        ("07", "07 Iterator"),
        ("08", "08 Lifetimes"),
        ("09", "09 Advanced Type System"),
        ("10", "10 Concurrency"),
        ("11", "11 Async Rust"),
        ("12", "12 Module & Architecture"),
        ("13", "13 Testing"),
        ("14", "14 Macros"),
        ("15", "15 Unsafe Rust"),
        ("16", "16 Library Design"),
        ("17", "17 Practical Projects"),
        ("18", "18 Final Project"),
    ];
    TITLES
        .iter()
        .find(|(p, _)| *p == prefix)
        .map(|(_, t)| t.to_string())
        .unwrap_or_else(|| format!("(unknown chapter {prefix})"))
}

/// この演習が、ルートworkspaceから独立したnested workspace（ex012のような構成）かどうか。
fn is_nested_workspace(ex_dir: &Path) -> bool {
    fs::read_to_string(ex_dir.join("Cargo.toml"))
        .map(|s| s.contains("[workspace]"))
        .unwrap_or(false)
}

/// `key = "value"` という行から文字列値を取り出す最小パーサー。
/// TOML全般には対応しない（このプロジェクトが書くexercise.toml / Cargo.toml専用）。
fn toml_get_string(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix(key) else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let rest = rest.trim();
        if let Some(inner) = rest.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            return Some(inner.to_string());
        }
    }
    None
}

fn run_tests(root: &Path, ex: &ExerciseMeta) -> Category {
    let output = if is_nested_workspace(&ex.dir) {
        Command::new("cargo")
            .args(["test", "--quiet"])
            .current_dir(&ex.dir)
            .output()
    } else {
        Command::new("cargo")
            .args(["test", "--quiet", "-p", &ex.id])
            .current_dir(root)
            .output()
    };

    let output = match output {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: cargo test を実行できませんでした（{}）: {e}", ex.id);
            return Category::Error;
        }
    };

    let mut combined = String::from_utf8_lossy(&output.stdout).into_owned();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    let (passed, failed) = sum_test_results(&combined);
    let todo_failures = count_todo_failures(&combined);

    // 注意: テストが1つも定義されていないcrate（0 passed; 0 failed）は、
    // ビルド失敗と区別できずここで Error 扱いになる。今のところ全ての演習は
    // 3つ以上のテストを持つため実害はないが、将来この分岐に頼る場合は注意。
    //
    // 骨組みのままでも通るテスト（型の性質を確かめるだけのテストなど）がある演習もあるので、
    // 「passed が 0 かどうか」ではなく「失敗が全部 todo!() によるものか」で判定する。
    if passed == 0 && failed == 0 {
        Category::Error
    } else if failed == 0 {
        Category::Ok
    } else if todo_failures == failed {
        Category::Todo(failed)
    } else {
        Category::Failed(failed)
    }
}

/// 失敗したテストのうち、todo!() で panic したものの数を数える。
///
/// cargo test は、失敗したテストごとに `---- <テスト名> stdout ----` で始まる節を出力する。
/// その節の中に todo!() のメッセージ（"not yet implemented"）があれば、未実装による失敗とみなす。
fn count_todo_failures(text: &str) -> u32 {
    let mut count = 0;
    let mut in_section = false;
    let mut section_is_todo = false;
    for line in text.lines() {
        let is_header = line.starts_with("---- ") && line.ends_with(" stdout ----");
        if is_header || line == "failures:" {
            if in_section && section_is_todo {
                count += 1;
            }
            in_section = is_header;
            section_is_todo = false;
        } else if in_section && line.contains("not yet implemented") {
            section_is_todo = true;
        }
    }
    if in_section && section_is_todo {
        count += 1;
    }
    count
}

fn sum_test_results(text: &str) -> (u32, u32) {
    let mut total_passed = 0u32;
    let mut total_failed = 0u32;
    for line in text.lines() {
        let Some(idx) = line.find("test result: ") else {
            continue;
        };
        let rest = &line[idx + "test result: ".len()..];
        if let Some((p, f)) = parse_counts(rest) {
            total_passed += p;
            total_failed += f;
        }
    }
    (total_passed, total_failed)
}

/// "ok. 5 passed; 0 failed; 0 ignored; ..." のような文字列から
/// (passed数, failed数) を取り出す。"passed"/"failed" という単語の
/// 直前のトークンを数値として読む、という単純な走査。
fn parse_counts(rest: &str) -> Option<(u32, u32)> {
    let normalized = rest.replace(';', " ");
    let tokens: Vec<&str> = normalized.split_whitespace().collect();
    let mut passed = None;
    let mut failed = None;
    for i in 1..tokens.len() {
        match tokens[i] {
            "passed" => passed = tokens[i - 1].parse::<u32>().ok(),
            "failed" => failed = tokens[i - 1].parse::<u32>().ok(),
            _ => {}
        }
    }
    match (passed, failed) {
        (Some(p), Some(f)) => Some((p, f)),
        _ => None,
    }
}

fn run_progress_mode(root: &Path, exercises: &[ExerciseMeta]) {
    let mut groups: BTreeMap<String, (String, Vec<&ExerciseMeta>)> = BTreeMap::new();
    for ex in exercises {
        let prefix = ex
            .lesson
            .split('-')
            .next()
            .unwrap_or(&ex.lesson)
            .to_string();
        let title = chapter_title(&ex.lesson);
        groups
            .entry(prefix)
            .or_insert_with(|| (title, Vec::new()))
            .1
            .push(ex);
    }

    let mut ok_count = 0u32;
    let total = exercises.len() as u32;

    for (title, exs) in groups.values() {
        println!("{title}");
        for ex in exs {
            let category = run_tests(root, ex);
            let (icon, note) = match category {
                Category::Ok => {
                    ok_count += 1;
                    ("✅", String::new())
                }
                Category::Failed(n) => ("❌", format!("{n} failed")),
                Category::Todo(n) => ("⬜", format!("{n} todo")),
                Category::Error => ("⚠️ ", "build error".to_string()),
            };
            println!("  {icon} {:<28} ({})  {note}", ex.id, ex.lesson);
        }
        println!();
    }

    println!("Progress: {ok_count} / {total}");
}

/// solutions/ の内容を一時的に上書きし、テストを実行後に必ず復元する。
/// Drop で復元するので、途中でpanicしても骨組みが失われない。
struct OverlayGuard {
    target: PathBuf,
    backup: Vec<u8>,
}

impl OverlayGuard {
    fn new(target: PathBuf, source: &Path) -> std::io::Result<Self> {
        let backup = fs::read(&target)?;
        let content = fs::read(source)?;
        fs::write(&target, content)?;
        Ok(Self { target, backup })
    }
}

impl Drop for OverlayGuard {
    fn drop(&mut self) {
        let _ = fs::write(&self.target, &self.backup);
    }
}

/// (演習側のファイル, 対応する解答側のファイル) のペアを、両方のファイルが
/// 実際に存在するものだけ集める。解答ディレクトリ配下の `.rs` を再帰的に走査するので、
/// lib.rs 1つの演習も、複数モジュール（ex049）や入れ子 workspace（ex012, ex048）も同じ扱いになる。
fn overlay_pairs(root: &Path, ex: &ExerciseMeta) -> Vec<(PathBuf, PathBuf)> {
    let solution_dir = root.join("solutions").join(&ex.id);
    let mut pairs = Vec::new();
    for sol in rust_files(&solution_dir) {
        let Ok(rel) = sol.strip_prefix(&solution_dir) else {
            continue;
        };
        let target = ex.dir.join(rel);
        if target.exists() {
            pairs.push((target, sol.clone()));
        }
    }
    pairs.sort();
    pairs
}

/// ディレクトリ配下の `.rs` ファイルを再帰的に集める（target/ は除く）。
fn rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().and_then(|n| n.to_str()) == Some("target") {
                continue;
            }
            out.extend(rust_files(&path));
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path);
        }
    }
    out
}

fn run_solutions_mode(root: &Path, exercises: &[ExerciseMeta]) {
    let mut passed = 0u32;
    let mut failed_names: Vec<String> = Vec::new();

    for ex in exercises {
        let pairs = overlay_pairs(root, ex);
        if pairs.is_empty() {
            println!("== {}: solutions/ が見つかりません（スキップ）==", ex.id);
            continue;
        }

        let category = {
            let guards: Vec<OverlayGuard> = pairs
                .iter()
                .filter_map(
                    |(target, source)| match OverlayGuard::new(target.clone(), source) {
                        Ok(g) => Some(g),
                        Err(e) => {
                            eprintln!("warning: {}: {e}", target.display());
                            None
                        }
                    },
                )
                .collect();
            let category = run_tests(root, ex);
            drop(guards); // ここで骨組みへ復元される
            category
        };

        let ok = matches!(category, Category::Ok);
        println!("== {} ==  {}", ex.id, if ok { "OK" } else { "NG" });
        if ok {
            passed += 1;
        } else {
            failed_names.push(ex.id.clone());
        }
    }

    println!();
    println!("Passed: {passed}  Failed: {}", failed_names.len());
    if !failed_names.is_empty() {
        println!("Failed exercises: {}", failed_names.join(", "));
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OUTPUT: &str = "\
running 3 tests
failures:

---- a stdout ----

thread 'a' panicked at src/lib.rs:3:5:
not yet implemented: 書いてください

---- b stdout ----

thread 'b' panicked at tests/tests.rs:9:5:
assertion `left == right` failed
  left: 1
 right: 2

failures:
    a
    b

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
";

    #[test]
    fn counts_only_failures_caused_by_todo() {
        assert_eq!(count_todo_failures(OUTPUT), 1);
        assert_eq!(sum_test_results(OUTPUT), (1, 2));
    }

    #[test]
    fn no_failures_means_no_todo() {
        assert_eq!(
            count_todo_failures("test result: ok. 3 passed; 0 failed;"),
            0
        );
    }
}

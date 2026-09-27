//! 薄いバイナリ。今日の日付を決め、`cli::run` を呼び、結果を表示するだけ。

use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use final_lending::cli::{self, CliError, USAGE};
use final_lending::Date;

/// システムの時計から今日の日付を得る（UTC）。
///
/// 時計を読むのはプログラム全体でここだけ。ドメインは日付を引数で受け取る。
fn system_today() -> Date {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    Date::from_unix_days((secs / 86_400) as i32)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let invocation = match cli::parse_args(&args) {
        Ok(inv) => inv,
        Err(e) => {
            eprintln!("error: {e}\n\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let today = invocation.today.unwrap_or_else(system_today);

    match cli::run(&invocation, today) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        // ルールで断られたのは、利用者の操作の問題（終了コード 1）
        Err(e @ CliError::Lending(_)) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
        // ファイルが読めない・壊れているのは、環境の問題（終了コード 3）
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(3)
        }
    }
}

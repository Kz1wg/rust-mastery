//! 薄いバイナリ。ロジックはすべて lib.rs にある（Lesson 12-2）。
//!
//! 使い方: wordstat [--top N] [--ignore-case] <file>

use p01_wordstat::{parse_args, run};
use std::fs::File;
use std::process::ExitCode;

fn main() -> ExitCode {
    let raw: Vec<String> = std::env::args().skip(1).collect();

    let args = match parse_args(&raw) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("usage: wordstat [--top N] [--ignore-case] <file>");
            return ExitCode::from(2);
        }
    };

    let file = match File::open(&args.path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {}: {e}", args.path);
            return ExitCode::from(1);
        }
    };

    match run(&args, file) {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

//! P3: ログ解析。
//!
//! 1行の形式: `<日時> <レベル> [<モジュール>] <メッセージ>`
//!
//! ```text
//! 2026-09-22T10:00:00 INFO [auth] user logged in
//! 2026-09-22T10:00:05 ERROR [db] connection refused
//! ```
//!
//! 解析できない行があっても、そこで止めずに集計を続ける。
//! 解析できなかった行は、理由と行番号つきで Summary に残す。

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::io::BufRead;
use std::str::FromStr;

/// ログのレベル。重要度の低い順に並べてあるので、比較（Ord）もその順になる。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl FromStr for Level {
    type Err = String;

    /// "DEBUG" / "INFO" / "WARN" / "ERROR" を読む（大文字のみ）。それ以外は Err（読めなかった文字列を返す）。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("4つの文字列をそれぞれの Level に変換し、それ以外は Err(s.to_string()) を返してください")
    }
}

/// 解析できた1行。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: Level,
    pub module: String,
    pub message: String,
}

/// 1行を解析できなかった理由。どの行かを持つ（行番号は 1 から）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// 必要な部分が足りない（例: レベルやモジュールが無い）。
    MissingField { line: usize, field: &'static str },
    /// 知らないレベル（例: "FATAL"）。
    UnknownLevel { line: usize, level: String },
    /// モジュールが `[...]` の形になっていない。
    BadModule { line: usize },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::MissingField { line, field } => {
                write!(f, "{line}行目: {field} がありません")
            }
            ParseError::UnknownLevel { line, level } => {
                write!(f, "{line}行目: 知らないレベルです: {level}")
            }
            ParseError::BadModule { line } => {
                write!(f, "{line}行目: モジュールは [名前] の形で書いてください")
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// 1行を解析する。
///
/// 手順:
/// 1. 空白で最大3つに分ける（splitn(3, ' ')）: 日時・レベル・残り
/// 2. レベルを Level に変換する（失敗したら UnknownLevel）
/// 3. 残りが `[` で始まり、`]` を含むこと（そうでなければ BadModule）
/// 4. `]` より後ろを前後の空白を除いてメッセージにする（空でもよい）
///
/// 日時・レベル・残りのどれかが無ければ MissingField（field は "timestamp" / "level" / "module"）。
pub fn parse_line(line: &str, line_no: usize) -> Result<LogEntry, ParseError> {
    let mut parts = line.splitn(3, ' ');
    let timestamp = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or(ParseError::MissingField {
            line: line_no,
            field: "timestamp",
        })?;
    let _ = (timestamp, &mut parts);
    todo!("上の手順の 2〜4 を実装してください")
}

/// モジュールごとのエラーの数。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleErrors {
    pub module: String,
    pub errors: usize,
}

/// 集計結果。
#[derive(Debug, Default)]
pub struct Summary {
    /// 解析できた行の数。
    pub total: usize,
    /// レベルごとの行数。
    pub by_level: BTreeMap<Level, usize>,
    /// ERROR の多いモジュール順（同じ数ならモジュール名の辞書順）。
    pub errors_by_module: Vec<ModuleErrors>,
    /// 解析できなかった行。
    pub invalid: Vec<ParseError>,
}

impl Summary {
    /// ERROR の割合（0.0〜1.0）。解析できた行が無ければ 0.0。
    pub fn error_rate(&self) -> f64 {
        todo!("by_level の Error の数を total で割ってください（total が 0 なら 0.0）")
    }
}

/// 入力を1行ずつ読んで集計する。空行は読み飛ばす（行番号は数える）。
/// 読み込み自体の失敗（io::Error）は、そこで集計を打ち切る。
pub fn summarize<R: BufRead>(input: R) -> std::io::Result<Summary> {
    let mut summary = Summary::default();
    let mut errors: HashMap<String, usize> = HashMap::new();

    for (index, line) in input.lines().enumerate() {
        let line = line?;
        let line_no = index + 1;
        if line.trim().is_empty() {
            continue;
        }
        let _ = (&mut summary, &mut errors, line_no);
        todo!("parse_line の結果で分け、Ok なら total・by_level・（ERROR なら）errors を数え、Err なら invalid に積んでください")
    }

    let mut by_module: Vec<ModuleErrors> = errors
        .into_iter()
        .map(|(module, errors)| ModuleErrors { module, errors })
        .collect();
    by_module.sort_by(|a, b| {
        b.errors
            .cmp(&a.errors)
            .then_with(|| a.module.cmp(&b.module))
    });
    summary.errors_by_module = by_module;
    Ok(summary)
}

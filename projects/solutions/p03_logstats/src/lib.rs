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
        match s {
            "DEBUG" => Ok(Level::Debug),
            "INFO" => Ok(Level::Info),
            "WARN" => Ok(Level::Warn),
            "ERROR" => Ok(Level::Error),
            other => Err(other.to_string()),
        }
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
    let level_str = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or(ParseError::MissingField {
            line: line_no,
            field: "level",
        })?;
    let rest = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or(ParseError::MissingField {
            line: line_no,
            field: "module",
        })?;

    let level = level_str
        .parse::<Level>()
        .map_err(|level| ParseError::UnknownLevel {
            line: line_no,
            level,
        })?;

    let rest = rest
        .strip_prefix('[')
        .ok_or(ParseError::BadModule { line: line_no })?;
    let (module, message) = rest
        .split_once(']')
        .ok_or(ParseError::BadModule { line: line_no })?;

    Ok(LogEntry {
        timestamp: timestamp.to_string(),
        level,
        module: module.to_string(),
        message: message.trim().to_string(),
    })
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
        if self.total == 0 {
            return 0.0;
        }
        let errors = self.by_level.get(&Level::Error).copied().unwrap_or(0);
        errors as f64 / self.total as f64
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
        match parse_line(&line, line_no) {
            Ok(entry) => {
                summary.total += 1;
                *summary.by_level.entry(entry.level).or_insert(0) += 1;
                if entry.level == Level::Error {
                    *errors.entry(entry.module).or_insert(0) += 1;
                }
            }
            Err(e) => summary.invalid.push(e),
        }
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

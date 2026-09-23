//! 時間の文字列を解析する小さなライブラリ（Lesson 16-3: ドキュメントと例）。
//!
//! この crate のテストの中心は、下の `parse_duration` のドキュメントにあるコード例です。
//! `cargo test` を実行すると、コード例がそのまま実行されます。
//!
//! crate の先頭に `#![deny(missing_docs)]` を付けているので、
//! ドキュメントの無い公開項目を足すとコンパイルできません。

#![deny(missing_docs)]

/// `parse_duration` が失敗したときの理由。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DurationError {
    /// 入力が空文字列だった。
    Empty,
    /// 数字の後に単位が無かった（例: `"90"`）。
    MissingUnit,
    /// 単位の前に数字が無かった（例: `"h"`）。
    MissingNumber,
    /// 知らない単位があった（例: `"5d"` の `d`）。
    UnknownUnit(char),
}

/// `"1h30m"` のような時間の文字列を、秒数に変換する。
///
/// 使える単位は `h`（時間）・`m`（分）・`s`（秒）で、組み合わせられます。
/// 単位は大きい順に並べる必要はありません。
///
/// # Examples
///
/// ```
/// use ex061_documentation::parse_duration;
///
/// assert_eq!(parse_duration("90s"), Ok(90));
/// assert_eq!(parse_duration("1h30m"), Ok(5400));
/// assert_eq!(parse_duration("2m1h"), Ok(3720)); // 順番は自由
/// ```
///
/// # Errors
///
/// 失敗した理由を [`DurationError`] で返します。
///
/// ```
/// use ex061_documentation::{parse_duration, DurationError};
///
/// assert_eq!(parse_duration(""), Err(DurationError::Empty));
/// assert_eq!(parse_duration("90"), Err(DurationError::MissingUnit));
/// assert_eq!(parse_duration("h"), Err(DurationError::MissingNumber));
/// assert_eq!(parse_duration("5d"), Err(DurationError::UnknownUnit('d')));
/// ```
pub fn parse_duration(s: &str) -> Result<u64, DurationError> {
    if s.is_empty() {
        return Err(DurationError::Empty);
    }

    let mut total: u64 = 0;
    let mut digits = String::new();

    for c in s.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
            continue;
        }
        // ここに来たら c は単位のはず。
        // digits が空なら MissingNumber、c が h/m/s 以外なら UnknownUnit(c)。
        // そうでなければ、digits を数にして単位に応じた秒数を total に足し、digits を空にする。
        let _ = (c, &mut total);
        todo!("上のコメントの3つの場合を実装してください")
    }

    todo!("ループの後、digits が残っていれば MissingUnit、そうでなければ total を Ok で返してください")
}

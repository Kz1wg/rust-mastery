//! 時間の文字列を解析する小さなライブラリ（Lesson 16-3: ドキュメントと例）。

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
        if digits.is_empty() {
            return Err(DurationError::MissingNumber);
        }
        let seconds_per_unit = match c {
            'h' => 3600,
            'm' => 60,
            's' => 1,
            other => return Err(DurationError::UnknownUnit(other)),
        };
        // digits は ASCII 数字だけなので parse は桁あふれ以外で失敗しない
        let n: u64 = digits.parse().map_err(|_| DurationError::MissingNumber)?;
        total += n * seconds_per_unit;
        digits.clear();
    }

    if !digits.is_empty() {
        return Err(DurationError::MissingUnit);
    }
    Ok(total)
}

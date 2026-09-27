//! 日付（年月日）。時刻やタイムゾーンは扱わない。
//!
//! 貸出期間の計算に必要なのは「日付の比較」と「n 日後」と「何日差か」だけなので、
//! 1970-01-01 からの通算日数（i32）を1つ持つだけの小さな型にしている。
//! 年月日との変換は、Howard Hinnant の days_from_civil / civil_from_days のアルゴリズムを使う。

use std::fmt;
use std::str::FromStr;

/// 年月日。1年1月1日〜9999年12月31日の範囲だけを扱う。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    /// 1970-01-01 を 0 とする通算日数。
    days: i32,
}

/// `YYYY-MM-DD` として読めなかった。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDateError(pub String);

impl fmt::Display for ParseDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "日付は YYYY-MM-DD の形で指定してください: {:?}", self.0)
    }
}

impl std::error::Error for ParseDateError {}

fn is_leap(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn days_in_month(y: i32, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(y) => 29,
        2 => 28,
        _ => 0,
    }
}

impl Date {
    /// 実在する年月日なら `Some`。2月30日や13月は `None`。
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<Date> {
        if !(1..=9999).contains(&year) || day == 0 || day > days_in_month(year, month) {
            return None;
        }
        let y = if month <= 2 { year - 1 } else { year };
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = y - era * 400;
        let m = month as i32;
        let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + day as i32 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        Some(Date {
            days: era * 146_097 + doe - 719_468,
        })
    }

    /// 1970-01-01 からの通算日数から作る（システム時計から今日の日付を得るときに使う）。
    pub fn from_unix_days(days: i32) -> Date {
        Date { days }
    }

    /// (年, 月, 日)
    pub fn ymd(self) -> (i32, u32, u32) {
        let z = self.days + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        let y = yoe + era * 400 + i32::from(m <= 2);
        (y, m, d)
    }

    /// n 日後。
    pub fn add_days(self, n: u32) -> Date {
        Date {
            days: self.days + n as i32,
        }
    }

    /// `self` が `earlier` の何日後か（前なら負の数）。
    pub fn days_since(self, earlier: Date) -> i32 {
        self.days - earlier.days
    }
}

impl FromStr for Date {
    type Err = ParseDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || ParseDateError(s.to_string());
        let mut parts = s.split('-');
        let (Some(y), Some(m), Some(d), None) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return Err(err());
        };
        if y.len() != 4 || m.len() != 2 || d.len() != 2 {
            return Err(err());
        }
        let y: i32 = y.parse().map_err(|_| err())?;
        let m: u32 = m.parse().map_err(|_| err())?;
        let d: u32 = d.parse().map_err(|_| err())?;
        Date::from_ymd(y, m, d).ok_or_else(err)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d) = self.ymd();
        write!(f, "{y:04}-{m:02}-{d:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(s: &str) -> Date {
        s.parse().unwrap()
    }

    #[test]
    fn epoch_is_zero() {
        assert_eq!(Date::from_ymd(1970, 1, 1), Some(Date::from_unix_days(0)));
    }

    #[test]
    fn round_trips_through_ymd() {
        for s in [
            "2000-02-29",
            "2024-12-31",
            "2026-09-25",
            "0001-01-01",
            "9999-12-31",
        ] {
            assert_eq!(date(s).to_string(), s);
        }
    }

    #[test]
    fn rejects_dates_that_do_not_exist() {
        assert_eq!(Date::from_ymd(2026, 2, 29), None); // うるう年ではない
        assert_eq!(Date::from_ymd(1900, 2, 29), None); // 100 で割り切れる年
        assert!(Date::from_ymd(2000, 2, 29).is_some()); // 400 で割り切れる年
        assert_eq!(Date::from_ymd(2026, 13, 1), None);
        assert_eq!(Date::from_ymd(2026, 4, 31), None);
    }

    #[test]
    fn rejects_malformed_text() {
        for s in [
            "2026-9-25",
            "2026/09/25",
            "2026-09",
            "2026-09-25-01",
            "abcd-ef-gh",
            "",
        ] {
            assert!(s.parse::<Date>().is_err(), "{s} は失敗するはず");
        }
    }

    #[test]
    fn arithmetic_crosses_month_and_year() {
        assert_eq!(date("2026-12-25").add_days(14), date("2027-01-08"));
        assert_eq!(date("2024-02-20").add_days(10), date("2024-03-01"));
        assert_eq!(date("2027-01-08").days_since(date("2026-12-25")), 14);
        assert_eq!(date("2026-01-01").days_since(date("2026-01-02")), -1);
    }
}

//! 本と利用者の ID。
//!
//! どちらも「英数字とハイフン、1〜16文字」という同じ規則の newtype。
//! 同じ定義を2回書く代わりに、小さな macro_rules! で作っている（Lesson 14-1:
//! 「型ごとに同じ impl を並べる」は、マクロが正当化される典型的な場面）。
//! 別の選択肢（PhantomData を使う `Id<T>`）は、本文 18-3 で比較している。

use std::fmt;
use std::str::FromStr;

/// ID として受け付けない文字列だった。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidId {
    /// どの ID か（"本の ID" など）。
    pub kind: &'static str,
    pub value: String,
}

impl fmt::Display for InvalidId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}は英数字とハイフンの1〜16文字で指定してください: {:?}",
            self.kind, self.value
        )
    }
}

impl std::error::Error for InvalidId {}

fn is_valid_id(s: &str) -> bool {
    (1..=16).contains(&s.len()) && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident, $label:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            /// 規則に合う文字列なら ID を作る。
            pub fn new(s: &str) -> Result<Self, InvalidId> {
                if is_valid_id(s) {
                    Ok($name(s.to_string()))
                } else {
                    Err(InvalidId {
                        kind: $label,
                        value: s.to_string(),
                    })
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl FromStr for $name {
            type Err = InvalidId;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $name::new(s)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

define_id!(
    /// 本（1冊ずつの資料）の ID。例: `B001`
    BookId,
    "本の ID"
);

define_id!(
    /// 利用者の ID。例: `M001`
    MemberId,
    "利用者の ID"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_ascii_alphanumeric_and_hyphen() {
        assert_eq!(BookId::new("B-001").unwrap().as_str(), "B-001");
        assert!(MemberId::new("m1").is_ok());
        assert!(BookId::new("1234567890123456").is_ok()); // ちょうど16文字
    }

    #[test]
    fn rejects_empty_long_and_other_characters() {
        for s in ["", "12345678901234567", "B 001", "B\t1", "本1", "B_001"] {
            assert!(BookId::new(s).is_err(), "{s:?} は失敗するはず");
        }
    }

    #[test]
    fn error_says_which_id() {
        let e = MemberId::new("").unwrap_err();
        assert_eq!(e.kind, "利用者の ID");
    }
}

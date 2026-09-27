//! 利用者と、利用者の種別ごとの貸出ルール。

use std::fmt;
use std::str::FromStr;

use crate::ids::MemberId;

/// 利用者の種別。種別によって、借りられる冊数と期間が変わる。
///
/// 最初の版（v1）には種別がなく、全員が「3冊・14日」だった。
/// 「職員はもっと借りられるようにしたい」という変更要求を受けて足したもの（本文 18-8）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberKind {
    General,
    Staff,
}

/// 貸出のルール。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Policy {
    /// 同時に借りられる冊数。
    pub max_loans: usize,
    /// 貸出期間（日）。
    pub loan_days: u32,
}

impl MemberKind {
    /// この種別のルール。
    ///
    /// ルールを種別の `match` 1か所に集めている。種別を足すと、ここでコンパイルエラーになり、
    /// ルールを決め忘れることがない。
    pub fn policy(self) -> Policy {
        match self {
            MemberKind::General => Policy {
                max_loans: 3,
                loan_days: 14,
            },
            MemberKind::Staff => Policy {
                max_loans: 10,
                loan_days: 30,
            },
        }
    }
}

/// `general` / `staff` 以外が指定された。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidMemberKind(pub String);

impl fmt::Display for InvalidMemberKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "利用者の種別は general か staff で指定してください: {:?}",
            self.0
        )
    }
}

impl std::error::Error for InvalidMemberKind {}

impl FromStr for MemberKind {
    type Err = InvalidMemberKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "general" => Ok(MemberKind::General),
            "staff" => Ok(MemberKind::Staff),
            other => Err(InvalidMemberKind(other.to_string())),
        }
    }
}

impl fmt::Display for MemberKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            MemberKind::General => "general",
            MemberKind::Staff => "staff",
        })
    }
}

/// 利用者。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    pub id: MemberId,
    pub name: String,
    pub kind: MemberKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_round_trips_through_text() {
        // FromStr は文字列の match なので、種別を足し忘れてもコンパイラは教えてくれない。
        // 全ての種別について、書いて読み戻せることをここで確かめる
        for kind in [MemberKind::General, MemberKind::Staff] {
            assert_eq!(kind.to_string().parse::<MemberKind>(), Ok(kind));
        }
    }

    #[test]
    fn unknown_kind_is_rejected() {
        assert!("visitor".parse::<MemberKind>().is_err());
        assert!("General".parse::<MemberKind>().is_err()); // 大文字・小文字は区別する
    }
}

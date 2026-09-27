//! Library をテキストに保存し、テキストから読み戻す。
//!
//! 形式（タブ区切り、1行1レコード）:
//!
//! ```text
//! # lending v2
//! member  M001  general  山田 花子
//! book    B001  Rustの本  available
//! book    B002  型の本    loan  M001  2026-09-01  2026-09-15
//! ```
//!
//! - 1行目は形式の名前と版。将来形式を変えたときに、古いファイルを見分けるため
//! - v1 は利用者の種別が無かった版（member の行が `member ID 名前`）。
//!   v1 のファイルも読めるようにし、種別は general とみなす（本文 18-8）。保存は常に v2
//! - member の行は book の行より前に書く（貸出中の本が、存在する利用者を指していることを確かめるため）
//! - 名前や書名に含まれるタブ・改行・`\` は `\t` `\n` `\\` に置き換える。
//!   保存形式の都合で「書名にタブを使えない」という制限をドメインに持ち込まないため
//!
//! ドメイン（library.rs）はこの形式を知らない。形式を JSON などに変えても、変わるのはこのファイルだけ。

use std::fmt;

use crate::date::Date;
use crate::ids::{BookId, MemberId};
use crate::library::{Book, BookState, LendingError, Library, Loan};
use crate::member::MemberKind;

const HEADER: &str = "# lending v2";
const HEADER_V1: &str = "# lending v1";

/// 読み込みの失敗。何行目で何が起きたか。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreError {
    pub line: usize,
    pub kind: StoreErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreErrorKind {
    /// 1行目が `# lending v2`（または v1）ではない。
    UnsupportedFormat,
    /// member / book 以外の行。
    UnknownRecord(String),
    /// 項目の数が合わない。
    WrongFieldCount,
    /// 項目の値が読めない（ID・日付・種別・エスケープなど）。
    InvalidField(String),
    /// 値は読めたが、データとして矛盾している（ID の重複など）。
    Inconsistent(LendingError),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}行目: ", self.line)?;
        match &self.kind {
            StoreErrorKind::UnsupportedFormat => {
                write!(f, "このプログラムで読める形式（{HEADER}）ではありません")
            }
            StoreErrorKind::UnknownRecord(r) => write!(f, "知らない種類の行です: {r:?}"),
            StoreErrorKind::WrongFieldCount => write!(f, "項目の数が合いません"),
            StoreErrorKind::InvalidField(msg) => write!(f, "{msg}"),
            StoreErrorKind::Inconsistent(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for StoreError {}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
    out
}

fn unescape(s: &str) -> Option<String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next()? {
            '\\' => out.push('\\'),
            't' => out.push('\t'),
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            _ => return None,
        }
    }
    Some(out)
}

/// Library をテキストにする。
pub fn save(library: &Library) -> String {
    let mut out = String::new();
    write_library(&mut out, library).expect("String への書き込みは失敗しない");
    out
}

fn write_library(out: &mut impl fmt::Write, library: &Library) -> fmt::Result {
    writeln!(out, "{HEADER}")?;
    for m in library.members() {
        writeln!(out, "member\t{}\t{}\t{}", m.id, m.kind, escape(&m.name))?;
    }
    for b in library.books() {
        write!(out, "book\t{}\t{}\t", b.id, escape(&b.title))?;
        match &b.state {
            BookState::Available => writeln!(out, "available")?,
            BookState::OnLoan(loan) => {
                writeln!(out, "loan\t{}\t{}\t{}", loan.member, loan.since, loan.due)?
            }
        }
    }
    Ok(())
}

/// テキストから Library を読み戻す。
pub fn load(text: &str) -> Result<Library, StoreError> {
    let mut library = Library::new();
    let mut lines = text.lines().enumerate().map(|(i, l)| (i + 1, l));

    let version = match lines.next().map(|(_, first)| first.trim_end()) {
        Some(HEADER) => 2,
        Some(HEADER_V1) => 1,
        _ => {
            return Err(StoreError {
                line: 1,
                kind: StoreErrorKind::UnsupportedFormat,
            })
        }
    };

    for (line, raw) in lines {
        if raw.trim().is_empty() {
            continue;
        }
        let at = |kind| StoreError { line, kind };
        let fields: Vec<&str> = raw.trim_end_matches('\r').split('\t').collect();
        match fields[0] {
            "member" => {
                let (id, kind, name) = match (version, &fields[..]) {
                    (2, [_, id, kind, name]) => (*id, field::<MemberKind>(kind, line)?, *name),
                    // v1 には種別が無い。当時の利用者は全員「一般」だった
                    (1, [_, id, name]) => (*id, MemberKind::General, *name),
                    _ => return Err(at(StoreErrorKind::WrongFieldCount)),
                };
                let id: MemberId = field(id, line)?;
                let name = text_field(name, line)?;
                library
                    .add_member(id, &name, kind)
                    .map_err(|e| at(StoreErrorKind::Inconsistent(e)))?;
            }
            "book" => {
                let (id, title, state) = match fields[..] {
                    [_, id, title, "available"] => (id, title, BookState::Available),
                    [_, id, title, "loan", member, since, due] => {
                        let loan = Loan {
                            member: field::<MemberId>(member, line)?,
                            since: field::<Date>(since, line)?,
                            due: field::<Date>(due, line)?,
                        };
                        (id, title, BookState::OnLoan(loan))
                    }
                    _ => return Err(at(StoreErrorKind::WrongFieldCount)),
                };
                let book = Book {
                    id: field::<BookId>(id, line)?,
                    title: text_field(title, line)?,
                    state,
                };
                library
                    .restore_book(book)
                    .map_err(|e| at(StoreErrorKind::Inconsistent(e)))?;
            }
            other => return Err(at(StoreErrorKind::UnknownRecord(other.to_string()))),
        }
    }
    Ok(library)
}

/// `FromStr` を実装した型として1項目を読む。失敗はその型のエラーメッセージを持つ。
fn field<T>(s: &str, line: usize) -> Result<T, StoreError>
where
    T: std::str::FromStr,
    T::Err: fmt::Display,
{
    s.parse().map_err(|e: T::Err| StoreError {
        line,
        kind: StoreErrorKind::InvalidField(e.to_string()),
    })
}

fn text_field(s: &str, line: usize) -> Result<String, StoreError> {
    unescape(s).ok_or_else(|| StoreError {
        line,
        kind: StoreErrorKind::InvalidField(format!("エスケープが正しくありません: {s:?}")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_round_trips() {
        for s in [
            "plain",
            "a\tb",
            "line1\nline2",
            "back\\slash",
            "\\t literally",
            "",
        ] {
            assert_eq!(unescape(&escape(s)).as_deref(), Some(s));
        }
    }

    #[test]
    fn unknown_escape_is_rejected() {
        assert_eq!(unescape("bad\\x"), None);
        assert_eq!(unescape("trailing\\"), None);
    }
}

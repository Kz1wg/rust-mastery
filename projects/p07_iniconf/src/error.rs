//! このライブラリが返すエラー。

use std::fmt;

/// 解析の失敗。何行目で、何が起きたかを持つ。
///
/// フィールドは非公開にして、読み出し用のメソッドを用意している。
/// こうしておくと、後から情報（列番号など）を足しても利用者のコードを壊さない。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    line: usize,
    kind: ParseErrorKind,
}

impl ParseError {
    /// ライブラリの中だけで作る。利用者が勝手に作る必要はない。
    pub(crate) fn new(line: usize, kind: ParseErrorKind) -> Self {
        ParseError { line, kind }
    }

    /// 問題のあった行（1から数える）。
    pub fn line(&self) -> usize {
        self.line
    }

    /// 何が起きたか。
    pub fn kind(&self) -> &ParseErrorKind {
        &self.kind
    }
}

/// 解析の失敗の種類。
///
/// `#[non_exhaustive]` なので、利用者の `match` には `_` の腕が必要になる。
/// そのかわり、将来ここに種類を足しても、利用者のコードは壊れない（Lesson 03-4, 16-2）。
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// `[name` のように、`]` で閉じていない。
    UnclosedSection,
    /// `[]` や `[  ]` のように、セクション名が空。
    EmptySectionName,
    /// `key value` のように、`=` がない。
    MissingEquals,
    /// `= value` のように、キーが空。
    EmptyKey,
    /// 同じセクションの中に、同じキーが2回出てきた。
    DuplicateKey(String),
    /// 同じ名前のセクションが2回出てきた。
    DuplicateSection(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line = self.line;
        match &self.kind {
            ParseErrorKind::UnclosedSection => write!(f, "{line}行目: `]` で閉じられていません"),
            ParseErrorKind::EmptySectionName => write!(f, "{line}行目: セクション名が空です"),
            ParseErrorKind::MissingEquals => write!(f, "{line}行目: `=` がありません"),
            ParseErrorKind::EmptyKey => write!(f, "{line}行目: キーが空です"),
            ParseErrorKind::DuplicateKey(key) => {
                write!(f, "{line}行目: キー `{key}` が重複しています")
            }
            ParseErrorKind::DuplicateSection(name) => {
                write!(f, "{line}行目: セクション `[{name}]` が重複しています")
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// 値を取り出すときの失敗（`Document::get_parsed`）。
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetError {
    /// そのセクション・キーが見つからない。
    Missing { section: String, key: String },
    /// 見つかったが、求められた型に変換できない。
    Invalid {
        section: String,
        key: String,
        value: String,
    },
}

impl fmt::Display for GetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetError::Missing { section, key } => {
                write!(f, "[{section}] の `{key}` が見つかりません")
            }
            GetError::Invalid {
                section,
                key,
                value,
            } => write!(f, "[{section}] の `{key}` の値 `{value}` を変換できません"),
        }
    }
}

impl std::error::Error for GetError {}

//! 解析した結果（Document）と、その1セクションを覗くための型（Section）。

use std::fmt;
use std::str::FromStr;

use crate::error::{GetError, ParseError};

/// 1つのセクションの中身。ライブラリの外には見せない。
///
/// キーの順番を保つため、HashMap ではなく Vec に入れている。
/// 設定ファイルのキーは多くても数十個なので、線形探索で十分速い。
/// 表現を非公開にしてあるので、後から HashMap に変えても利用者は壊れない。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SectionData {
    pub(crate) name: String,
    pub(crate) entries: Vec<(String, String)>,
}

impl SectionData {
    pub(crate) fn new(name: &str) -> Self {
        SectionData {
            name: name.to_string(),
            entries: Vec::new(),
        }
    }

    pub(crate) fn get(&self, key: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// 解析済みの INI 文書。
///
/// - 最初のセクション見出しより前に書かれたキーは、名前が `""`（空文字列）の
///   「ルートセクション」に入る
/// - セクションとキーは、ファイルに書かれた順番を保つ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    /// 先頭は必ずルートセクション（名前は ""）。
    sections: Vec<SectionData>,
}

impl Document {
    /// ライブラリの中（parse）から作る。`sections[0]` はルートセクションであること。
    pub(crate) fn from_sections(sections: Vec<SectionData>) -> Self {
        debug_assert!(sections.first().is_some_and(|s| s.name.is_empty()));
        Document { sections }
    }

    /// 文字列を解析する。`s.parse::<Document>()` と同じ。
    pub fn parse(input: &str) -> Result<Document, ParseError> {
        crate::parse::parse(input)
    }

    /// セクションとキーを指定して、値を取り出す。ルートセクションは `""` で指定する。
    pub fn get(&self, section: &str, key: &str) -> Option<&str> {
        todo!("section で探し、見つかったら Section::get で key を探してください")
    }

    /// 値を取り出し、`T` に変換する（`FromStr` を実装した型なら何でもよい）。
    pub fn get_parsed<T: FromStr>(&self, section: &str, key: &str) -> Result<T, GetError> {
        // 見つからない → GetError::Missing、parse に失敗 → GetError::Invalid
        todo!("get で探し、見つかった値を parse してください")
    }

    /// 名前でセクションを探す。`""` を渡すとルートセクション（常に存在する）。
    pub fn section(&self, name: &str) -> Option<Section<'_>> {
        todo!("self.sections から名前の一致するものを find し、Section で包んでください")
    }

    /// 名前のあるセクションを、書かれた順に返す（ルートセクションは含まない）。
    pub fn sections(&self) -> impl Iterator<Item = Section<'_>> {
        // 骨組みでは型を合わせるために iter::empty を置いている。実装したら if ごと消してよい
        if false {
            std::iter::empty()
        } else {
            todo!("先頭のルートセクションを飛ばし、残りを Section で包んで返してください")
        }
    }
}

impl FromStr for Document {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Document::parse(s)
    }
}

/// 文書を INI 形式で書き出す。
///
/// - ルートセクションのキーを先頭に（見出しなし）
/// - 続いて各セクションを `[name]` の見出しとキーで
/// - 各行は `key = value`。かたまりの間には空行を1つ
///
/// 書き出したものを `parse` すると、元と同じ `Document` に戻る。
impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 上のコメントの規則どおりに writeln! で書き出す。
        // 空のルートセクションは書かない。かたまりの「間」にだけ空行を入れる
        todo!("ルート、各セクションの順に writeln! で書き出してください")
    }
}

/// 1つのセクションを覗くための型。`Document` の中身を借りているだけで、コピーはしない。
///
/// 中身の表現（`SectionData`）を直接公開せず、この型を通して見せる。
/// `Copy` なので、気軽に値として受け渡せる。
#[derive(Debug, Clone, Copy)]
pub struct Section<'a> {
    data: &'a SectionData,
}

impl<'a> Section<'a> {
    /// セクション名（ルートセクションなら ""）。
    pub fn name(&self) -> &'a str {
        &self.data.name
    }

    /// キーを指定して値を取り出す。
    pub fn get(&self, key: &str) -> Option<&'a str> {
        todo!("self.data から key を探してください（SectionData::get が使えます）")
    }

    /// `(キー, 値)` を書かれた順に返す。
    pub fn entries(&self) -> impl Iterator<Item = (&'a str, &'a str)> + 'a {
        // 骨組みでは型を合わせるために iter::empty を置いている。実装したら if ごと消してよい
        if false {
            std::iter::empty()
        } else {
            todo!("self.data.entries を iter し、(String, String) を (&str, &str) に map してください")
        }
    }

    /// キーの数。
    pub fn len(&self) -> usize {
        self.data.entries.len()
    }

    /// キーが1つもないか。
    pub fn is_empty(&self) -> bool {
        self.data.entries.is_empty()
    }
}

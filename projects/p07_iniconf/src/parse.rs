//! 文字列から Document を組み立てる。外からは `Document::parse` / `str::parse` で使う。

use crate::document::{Document, SectionData};
use crate::error::{ParseError, ParseErrorKind};

/// 1行ずつ読んで、Document を組み立てる。
///
/// 行の種類（前後の空白は取り除いてから判定する）:
/// - 空行、`;` か `#` で始まる行: 無視する（コメント）
/// - `[` で始まる行: セクションの見出し
/// - それ以外: `key = value`（最初の `=` で分ける。値の中に `=` があってもよい）
pub(crate) fn parse(input: &str) -> Result<Document, ParseError> {
    // 手順の例:
    // 1. sections を「ルートセクションだけが入った Vec」で始める
    // 2. input.lines().enumerate() で1行ずつ読む（行番号は 1 から数える）
    // 3. 行の種類で分けて、見出しなら sections に push、キーなら最後のセクションに push
    //    それぞれの失敗は ParseError::new(行番号, 種類) で返す
    // 4. 最後に Document::from_sections(sections)
    todo!("上の規則どおりに1行ずつ解析してください")
}

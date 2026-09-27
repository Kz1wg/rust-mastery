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
    let mut sections = vec![SectionData::new("")];

    for (index, raw) in input.lines().enumerate() {
        let line_no = index + 1;
        let line = raw.trim();
        let err = |kind| ParseError::new(line_no, kind);

        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        if let Some(rest) = line.strip_prefix('[') {
            let name = rest
                .strip_suffix(']')
                .ok_or(err(ParseErrorKind::UnclosedSection))?
                .trim();
            if name.is_empty() {
                return Err(err(ParseErrorKind::EmptySectionName));
            }
            if sections.iter().any(|s| s.name == name) {
                return Err(err(ParseErrorKind::DuplicateSection(name.to_string())));
            }
            sections.push(SectionData::new(name));
            continue;
        }

        let (key, value) = line
            .split_once('=')
            .ok_or(err(ParseErrorKind::MissingEquals))?;
        let (key, value) = (key.trim(), value.trim());
        if key.is_empty() {
            return Err(err(ParseErrorKind::EmptyKey));
        }

        let current = sections
            .last_mut()
            .expect("ルートセクションが最初に入っているので、空になることはない");
        if current.get(key).is_some() {
            return Err(err(ParseErrorKind::DuplicateKey(key.to_string())));
        }
        current.entries.push((key.to_string(), value.to_string()));
    }

    Ok(Document::from_sections(sections))
}

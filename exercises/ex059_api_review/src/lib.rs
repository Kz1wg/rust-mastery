//! Lesson 16-1: 公開APIのレビュー
//!
//! 元の API（このままでは実装しない）:
//!
//! ```text
//! pub fn analyze(text: String, ignore_case: bool, skip_numbers: bool, mode: &str)
//!     -> Vec<(String, usize)>
//! ```
//!
//! を、利用者の立場で見直した形にしてある。
//! - text は &str で受ける（読むだけなので）
//! - bool の並びと文字列の mode は、Unit（enum）と Options（ビルダー）に
//! - 戻り値のタプルは、名前付きの Count 構造体に

use std::collections::HashMap;

/// 解析の単位。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    /// 空白区切りの単語ごとに数える。
    Words,
    /// 空白以外の文字ごとに数える。
    Chars,
}

/// 解析の設定。`Options::default()` から、必要なものだけを変える。
#[derive(Debug, Clone)]
pub struct Options {
    unit: Unit,
    ignore_case: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            unit: Unit::Words,
            ignore_case: false,
        }
    }
}

impl Options {
    /// 数える単位を設定する。
    pub fn unit(mut self, unit: Unit) -> Self {
        self.unit = unit;
        self
    }

    /// 大文字・小文字を区別しないかどうかを設定する。
    pub fn ignore_case(mut self, yes: bool) -> Self {
        self.ignore_case = yes;
        self
    }
}

/// 1つの項目と、その出現回数。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Count {
    pub item: String,
    pub occurrences: usize,
}

/// text を解析し、項目ごとの出現回数を返す。
/// 結果は、出現回数の多い順（同じ回数なら item の辞書順）に並べる。
pub fn analyze(text: &str, options: &Options) -> Vec<Count> {
    let text = if options.ignore_case {
        text.to_lowercase()
    } else {
        text.to_string()
    };

    let mut counts: HashMap<String, usize> = HashMap::new();
    match options.unit {
        Unit::Words => {
            for word in text.split_whitespace() {
                *counts.entry(word.to_string()).or_insert(0) += 1;
            }
        }
        Unit::Chars => {
            todo!(
                "空白以外の文字ごとに counts を数えてください（char::is_whitespace で空白を判定）"
            )
        }
    }

    let mut result: Vec<Count> = counts
        .into_iter()
        .map(|(item, occurrences)| Count { item, occurrences })
        .collect();
    todo!("result を、出現回数の多い順、同じ回数なら item の辞書順に並べて返してください（sort_by と cmp を使う）")
}

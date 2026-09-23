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
            for c in text.chars().filter(|c| !c.is_whitespace()) {
                *counts.entry(c.to_string()).or_insert(0) += 1;
            }
        }
    }

    let mut result: Vec<Count> = counts
        .into_iter()
        .map(|(item, occurrences)| Count { item, occurrences })
        .collect();
    result.sort_by(|a, b| {
        b.occurrences
            .cmp(&a.occurrences)
            .then_with(|| a.item.cmp(&b.item))
    });
    result
}

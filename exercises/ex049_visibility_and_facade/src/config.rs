use std::collections::BTreeMap;

pub struct Config {
    entries: BTreeMap<String, String>,
}

impl Config {
    pub(crate) fn from_entries(entries: BTreeMap<String, String>) -> Self {
        Config { entries }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        todo!("normalize_key で正規化したキーで entries を引き、&str を返してください")
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// crate 内の複数モジュール（config と parser）から使うヘルパー。
/// 外部には見せない。
pub(crate) fn normalize_key(key: &str) -> String {
    todo!("前後の空白を取り除き、小文字にした String を返してください")
}

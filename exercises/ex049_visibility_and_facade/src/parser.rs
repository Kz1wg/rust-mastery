use crate::config::{normalize_key, Config};
use std::collections::BTreeMap;

/// "key=value" の行を解析して Config を作る。空行は無視する。
pub fn parse_config(text: &str) -> Config {
    let mut entries = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            todo!("normalize_key でキーを正規化し、値は trim して entries に入れてください");
        }
    }
    Config::from_entries(entries)
}

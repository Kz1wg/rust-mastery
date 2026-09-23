use crate::config::{normalize_key, Config};
use std::collections::BTreeMap;

pub fn parse_config(text: &str) -> Config {
    let mut entries = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            entries.insert(normalize_key(key), value.trim().to_string());
        }
    }
    Config::from_entries(entries)
}

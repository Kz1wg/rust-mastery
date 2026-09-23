use std::collections::BTreeMap;

pub struct Config {
    entries: BTreeMap<String, String>,
}

impl Config {
    pub(crate) fn from_entries(entries: BTreeMap<String, String>) -> Self {
        Config { entries }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(&normalize_key(key)).map(|v| v.as_str())
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

pub(crate) fn normalize_key(key: &str) -> String {
    key.trim().to_lowercase()
}

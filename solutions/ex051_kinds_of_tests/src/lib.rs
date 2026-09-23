use std::collections::BTreeMap;

/// ```
/// use ex051_kinds_of_tests::word_frequency;
///
/// let freq = word_frequency("Hello, hello world!");
/// assert_eq!(freq.get("hello"), Some(&2));
/// assert_eq!(freq.get("world"), Some(&1));
/// ```
pub fn word_frequency(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for word in text.split_whitespace() {
        let word = normalize(word);
        if !word.is_empty() {
            *counts.entry(word).or_insert(0) += 1;
        }
    }
    counts
}

fn normalize(word: &str) -> String {
    word.trim_matches(|c| matches!(c, ',' | '.' | '!' | '?'))
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_lowercases() {
        assert_eq!(normalize("Hello"), "hello");
    }

    #[test]
    fn normalize_strips_surrounding_punctuation() {
        assert_eq!(normalize("world!"), "world");
        assert_eq!(normalize("...wait?"), "wait");
    }

    #[test]
    fn normalize_keeps_inner_characters() {
        assert_eq!(normalize("don't"), "don't");
    }
}

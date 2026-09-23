//! Lesson 13-2: unit / integration / doctest
//!
//! この crate には3種類のテストがすでに用意されている。
//! - 単体テスト: このファイルの一番下の `mod tests`（private な normalize を確かめる）
//! - 統合テスト: tests/tests.rs（公開APIの word_frequency を確かめる）
//! - ドキュメントテスト: word_frequency の説明にあるコード例
//!
//! 関数を実装すると、3種類すべてが通るようになる。

use std::collections::BTreeMap;

/// 文章の中の単語ごとの出現回数を数える。大文字・小文字は区別せず、
/// 単語の前後の記号（, . ! ?）は取り除く。
///
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
            todo!("counts の word の数を1増やしてください（entry と or_insert を使う）");
        }
    }
    counts
}

/// 単語を小文字にし、前後の記号（, . ! ?）を取り除く。
/// private なので、外（統合テスト）からは呼べない。単体テストで確かめる。
fn normalize(word: &str) -> String {
    todo!("前後の , . ! ? を取り除き、小文字にして返してください（trim_matches と to_lowercase）")
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

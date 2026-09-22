//! Lesson 01-3: 所有権で設計する
//!
//! この演習では、シグネチャはすでに決まっている（本文の Solution / Deep Dive を参照）。
//! なぜこの受け方が選ばれているかを、実装しながら確認すること。

/// テキスト中の空白区切りの単語数を数える。
///
/// 読むだけの処理なので、`String` ではなく `&str` で受ける
/// （`&String` からも `&str` のリテラルからも呼び出せる）。
pub fn count_words(text: &str) -> usize {
    todo!("空白（複数の空白や改行も含む）で区切って単語数を数えてください")
}

pub struct Config {
    name: String,
}

impl Config {
    /// `name` を保存するので、所有権を受け取る必要がある。
    /// `impl Into<String>` で受けることで、`&str` からも `String` からも呼べる。
    pub fn new(name: impl Into<String>) -> Self {
        todo!("name を String に変換して保存してください")
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

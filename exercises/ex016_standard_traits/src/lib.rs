//! Lesson 04-4: 標準trait実装の設計
//!
//! - `Percentage` は失敗しうる変換なので `From` ではなく `TryFrom` を実装する。
//! - `Password` は `Display` を実装せず（誰に見せてよい表現も存在しないため）、
//!   `Debug` は中身を隠すよう手動で実装する。

#[derive(Debug, PartialEq)]
pub struct Percentage(u8);

#[derive(Debug, PartialEq)]
pub struct PercentageError;

impl Percentage {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl TryFrom<&str> for Percentage {
    type Error = PercentageError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        todo!("s を u8 にparseし、0..=100 の範囲なら Ok、そうでなければ Err(PercentageError) を返してください")
    }
}

pub struct Password(String);

impl Password {
    pub fn new(value: impl Into<String>) -> Self {
        Password(value.into())
    }
}

// Password には Display を実装しない
// （誰に見せてもよい「表示用の文字列表現」がそもそも存在しないため）。

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("中身を漏らさない表現（例: \"Password(***)\"）を書き込んでください")
    }
}

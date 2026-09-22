//! Lesson 06-1: Resultの型設計
//!
//! 元の設計（このままでは実装しない）:
//!
//! ```text
//! fn parse_config(s: &str) -> Result<i32, String> {
//!     let trimmed = s.trim();
//!     if trimmed.is_empty() { return Err("設定が空です".to_string()); }
//!     trimmed.parse::<i32>().map_err(|_| "数値として解釈できません".to_string())
//! }
//! ```
//!
//! 呼び出し側が「空だった」場合と「数値でなかった」場合を区別できるよう、
//! enumに置き換えてある。

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    Empty,
    NotANumber(String),
}

pub fn parse_config(s: &str) -> Result<i32, ConfigError> {
    todo!("s.trim() が空なら Err(ConfigError::Empty)、数値でなければ Err(ConfigError::NotANumber(元の文字列))、成功すれば Ok(値) を返してください")
}

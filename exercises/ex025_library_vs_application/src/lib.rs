//! Lesson 06-4: libraryとapplicationのerror設計
//!
//! load_config は「ライブラリ」の関数として、io::Errorの種類ごとに
//! 区別できる ConfigError を返す。run_app は「アプリケーション」の末端として、
//! Box<dyn Error> に集約する（ConfigError が std::error::Error を実装しているため
//! 自動変換される）。

use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    NotFound(std::io::Error),
    PermissionDenied(std::io::Error),
    Other(std::io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NotFound(_) => write!(f, "設定ファイルが見つかりません"),
            ConfigError::PermissionDenied(_) => write!(f, "設定ファイルへのアクセス権がありません"),
            ConfigError::Other(e) => write!(f, "設定の読み込みに失敗しました: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

/// ライブラリの関数: io::Error の種類ごとに区別できる ConfigError を返す。
pub fn load_config(path: &str) -> Result<String, ConfigError> {
    std::fs::read_to_string(path).map_err(|e| todo!(
        "e.kind() が NotFound なら ConfigError::NotFound(e)、PermissionDenied なら ConfigError::PermissionDenied(e)、それ以外は ConfigError::Other(e) を返してください"
    ))
}

/// アプリケーションの末端: 具体的な種類を区別せず、表示するだけでよい。
/// ConfigError は std::error::Error を実装しているので、`?` で自動的に
/// Box<dyn Error> に変換される。
pub fn run_app(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = load_config(path)?;
    todo!("content を大文字にして Ok で返してください（to_uppercase）")
}

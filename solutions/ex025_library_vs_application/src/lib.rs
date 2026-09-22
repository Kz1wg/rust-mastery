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

pub fn load_config(path: &str) -> Result<String, ConfigError> {
    std::fs::read_to_string(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => ConfigError::NotFound(e),
        std::io::ErrorKind::PermissionDenied => ConfigError::PermissionDenied(e),
        _ => ConfigError::Other(e),
    })
}

pub fn run_app(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = load_config(path)?;
    Ok(content.to_uppercase())
}

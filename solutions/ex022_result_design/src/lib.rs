#[derive(Debug, PartialEq)]
pub enum ConfigError {
    Empty,
    NotANumber(String),
}

pub fn parse_config(s: &str) -> Result<i32, ConfigError> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(ConfigError::Empty);
    }
    trimmed
        .parse::<i32>()
        .map_err(|_| ConfigError::NotANumber(trimmed.to_string()))
}

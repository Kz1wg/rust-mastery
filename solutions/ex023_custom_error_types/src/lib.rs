use std::fmt;

#[derive(Debug)]
pub enum ReadNumberError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl fmt::Display for ReadNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadNumberError::Io(e) => write!(f, "input error: {e}"),
            ReadNumberError::Parse(e) => write!(f, "invalid number: {e}"),
        }
    }
}

impl std::error::Error for ReadNumberError {}

impl From<std::io::Error> for ReadNumberError {
    fn from(e: std::io::Error) -> Self {
        ReadNumberError::Io(e)
    }
}

impl From<std::num::ParseIntError> for ReadNumberError {
    fn from(e: std::num::ParseIntError) -> Self {
        ReadNumberError::Parse(e)
    }
}

pub fn read_number_from_str(content: &str) -> Result<i32, ReadNumberError> {
    Ok(content.trim().parse::<i32>()?)
}

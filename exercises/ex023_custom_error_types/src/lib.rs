//! Lesson 06-2: 独自Error型
//!
//! ReadNumberError は Io と Parse の2種類の原因を区別できる enum で、
//! Display と std::error::Error を実装している（Box<dyn Error> にも変換できる）。

use std::fmt;

#[derive(Debug)]
pub enum ReadNumberError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl fmt::Display for ReadNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Io の場合は \"input error: <元のエラー>\"、Parse の場合は \"invalid number: <元のエラー>\" のように書き込んでください")
    }
}

impl std::error::Error for ReadNumberError {}

impl From<std::io::Error> for ReadNumberError {
    fn from(e: std::io::Error) -> Self {
        todo!("ReadNumberError::Io でラップしてください")
    }
}

impl From<std::num::ParseIntError> for ReadNumberError {
    fn from(e: std::num::ParseIntError) -> Self {
        todo!("ReadNumberError::Parse でラップしてください")
    }
}

pub fn read_number_from_str(content: &str) -> Result<i32, ReadNumberError> {
    Ok(content.trim().parse::<i32>()?)
}

#[derive(Debug, PartialEq)]
pub struct MyError {
    pub source: std::num::ParseIntError,
}

impl From<std::num::ParseIntError> for MyError {
    fn from(e: std::num::ParseIntError) -> Self {
        MyError { source: e }
    }
}

pub fn parse_two_numbers(a: &str, b: &str) -> Result<i32, MyError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}

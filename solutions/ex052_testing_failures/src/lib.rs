#[derive(Debug, PartialEq)]
pub enum AgeError {
    Empty,
    NotANumber,
    TooOld,
}

pub fn parse_age(s: &str) -> Result<u8, AgeError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(AgeError::Empty);
    }
    let n: u8 = s.parse().map_err(|_| AgeError::NotANumber)?;
    if n > 150 {
        return Err(AgeError::TooOld);
    }
    Ok(n)
}

pub fn get_item(items: &[i32], index: usize) -> i32 {
    if index >= items.len() {
        panic!("index out of range: {index} (len {})", items.len());
    }
    items[index]
}

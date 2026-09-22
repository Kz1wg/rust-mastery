//! Lesson 06-3: error propagation
//!
//! parse_two_numbers は、2つの文字列をそれぞれ数値にパースし、その合計を返す。
//! `?` が ParseIntError を MyError に変換できるのは、From を実装しているから。

#[derive(Debug, PartialEq)]
pub struct MyError {
    pub source: std::num::ParseIntError,
}

impl From<std::num::ParseIntError> for MyError {
    fn from(e: std::num::ParseIntError) -> Self {
        todo!("MyError構造体を、sourceフィールドにeを入れて返してください")
    }
}

/// a・b をそれぞれ数値としてパースし、合計を返す。
/// `?` は、パースに失敗したとき ParseIntError を MyError に自動変換する
/// （上の From 実装があるおかげ）。
pub fn parse_two_numbers(a: &str, b: &str) -> Result<i32, MyError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    todo!("x と y の合計を Ok で返してください")
}

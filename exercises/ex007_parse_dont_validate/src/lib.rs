//! Lesson 02-3: 構築時検証
//!
//! `Percentage` と `NonEmptyString` は、private フィールドと
//! 構築関数（`new`）を通してしか作れない。
//! 一度作られた値は「必ず検証済み」であることを、外部からの直接構築が
//! コンパイルできないこと（`compile_fail`）で確認している。

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(u8);

#[derive(Debug, PartialEq)]
pub struct PercentageError;

impl Percentage {
    /// `value` は 0..=100 の範囲でなければならない。
    ///
    /// フィールドが private なので、`Percentage(150)` のように
    /// 直接構築することはできない:
    ///
    /// ```compile_fail
    /// use ex007_parse_dont_validate::Percentage;
    ///
    /// let p = Percentage(150); // private field
    /// ```
    pub fn new(value: u8) -> Result<Self, PercentageError> {
        todo!("0..=100 の範囲なら Ok、そうでなければ Err を返してください")
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

pub struct NonEmptyString(String);

impl NonEmptyString {
    /// 空文字列なら `None` を返す。
    pub fn new(value: String) -> Option<Self> {
        todo!("value が空なら None、そうでなければ Some(NonEmptyString(value)) を返してください")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 検証済みの `Percentage` を文字列にする。
/// `Percentage` 自体が検証済みであることを型で保証しているので、
/// この関数の中で範囲を再検証する必要はない。
pub fn describe(p: Percentage) -> String {
    todo!("\"50%\" のような文字列を返してください（数値と % を連結する形式）")
}

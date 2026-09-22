#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(u8);

#[derive(Debug, PartialEq)]
pub struct PercentageError;

impl Percentage {
    /// ```compile_fail
    /// use ex007_parse_dont_validate::Percentage;
    ///
    /// let p = Percentage(150); // private field
    /// ```
    pub fn new(value: u8) -> Result<Self, PercentageError> {
        if value <= 100 {
            Ok(Percentage(value))
        } else {
            Err(PercentageError)
        }
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn new(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            Some(NonEmptyString(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn describe(p: Percentage) -> String {
    format!("{}%", p.value())
}

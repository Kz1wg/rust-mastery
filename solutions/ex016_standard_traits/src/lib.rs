#[derive(Debug, PartialEq)]
pub struct Percentage(u8);

#[derive(Debug, PartialEq)]
pub struct PercentageError;

impl Percentage {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl TryFrom<&str> for Percentage {
    type Error = PercentageError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let n: u8 = s.parse().map_err(|_| PercentageError)?;
        if n <= 100 {
            Ok(Percentage(n))
        } else {
            Err(PercentageError)
        }
    }
}

pub struct Password(String);

impl Password {
    pub fn new(value: impl Into<String>) -> Self {
        Password(value.into())
    }
}

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Password(***)")
    }
}

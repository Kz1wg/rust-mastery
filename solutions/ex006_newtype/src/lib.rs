#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Meters(f64);

impl Meters {
    pub fn new(value: f64) -> Self {
        Meters(value)
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Feet(f64);

impl Feet {
    pub fn new(value: f64) -> Self {
        Feet(value)
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

/// `Meters` 同士を足す。
///
/// `Feet` を渡すと、型が違うためコンパイルできない:
///
/// ```compile_fail
/// use ex006_newtype::{add_meters, Feet, Meters};
///
/// let a = Meters::new(1.0);
/// let b = Feet::new(1.0);
/// add_meters(a, b); // Feet は Meters ではない
/// ```
pub fn add_meters(a: Meters, b: Meters) -> Meters {
    Meters(a.0 + b.0)
}

impl From<Feet> for Meters {
    /// 1 ft = 0.3048 m
    fn from(feet: Feet) -> Self {
        Meters(feet.value() * 0.3048)
    }
}

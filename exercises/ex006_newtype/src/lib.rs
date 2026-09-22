//! Lesson 02-2: newtype pattern
//!
//! `Meters` と `Feet` は同じ `f64` を包んでいるが、別の型である。
//! `add_meters` に `Feet` を渡すと、コンパイルエラーになることを
//! ドキュメントテスト（`compile_fail`）で確認している。

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
    todo!("a と b の値を足した Meters を返してください")
}

impl From<Feet> for Meters {
    /// 1 ft = 0.3048 m
    fn from(feet: Feet) -> Self {
        todo!("feet.value() に 0.3048 を掛けて Meters にしてください")
    }
}

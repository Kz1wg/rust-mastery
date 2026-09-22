#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize>(pub [f64; N]);

impl<const N: usize> Vector<N> {
    /// ```compile_fail
    /// use ex036_type_level_constraints::Vector;
    ///
    /// let a = Vector([1.0, 2.0]);
    /// let b = Vector([1.0, 2.0, 3.0]);
    /// a.dot(&b); // Vector<2> と Vector<3> は別の型
    /// ```
    pub fn dot(&self, other: &Vector<N>) -> f64 {
        self.0.iter().zip(&other.0).map(|(a, b)| a * b).sum()
    }

    pub fn add(&self, other: &Vector<N>) -> Vector<N> {
        Vector(std::array::from_fn(|i| self.0[i] + other.0[i]))
    }
}

mod sealed {
    pub trait Sealed {}
}

/// ```compile_fail
/// use ex036_type_level_constraints::Unit;
///
/// struct Feet;
/// impl Unit for Feet {
///     const SYMBOL: &'static str = "ft"; // Feet は Sealed を実装できない
/// }
/// ```
pub trait Unit: sealed::Sealed {
    const SYMBOL: &'static str;
}

pub struct Meters;
pub struct Seconds;

impl sealed::Sealed for Meters {}
impl sealed::Sealed for Seconds {}

impl Unit for Meters {
    const SYMBOL: &'static str = "m";
}

impl Unit for Seconds {
    const SYMBOL: &'static str = "s";
}

pub fn format_value<U: Unit>(value: f64) -> String {
    format!("{value}{}", U::SYMBOL)
}

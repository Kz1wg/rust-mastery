//! Lesson 09-3: 型レベルでの制約表現
//!
//! - Vector<N>: 次元 N を型に持たせる。次元の違うベクトルどうしの演算はコンパイルできない。
//! - Unit: sealed trait。利用者は使えるが、実装はこの crate の中だけに限られる。
//!   doctest は別 crate として実行されるので、「利用者の立場」から封印を確かめられる。

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize>(pub [f64; N]);

impl<const N: usize> Vector<N> {
    /// 内積。次元の違うベクトルは渡せない:
    ///
    /// ```compile_fail
    /// use ex036_type_level_constraints::Vector;
    ///
    /// let a = Vector([1.0, 2.0]);
    /// let b = Vector([1.0, 2.0, 3.0]);
    /// a.dot(&b); // Vector<2> と Vector<3> は別の型
    /// ```
    pub fn dot(&self, other: &Vector<N>) -> f64 {
        todo!("成分ごとの積を合計してください（Result は要りません）")
    }

    /// 成分ごとの和。
    pub fn add(&self, other: &Vector<N>) -> Vector<N> {
        todo!("成分ごとに足した新しい Vector を返してください（長さ N の配列を作る）")
    }
}

mod sealed {
    pub trait Sealed {}
}

/// 単位。この crate の外からは実装できない:
///
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

/// 値に単位記号を付けて文字列にする（例: 3 と Meters なら "3m"）。
pub fn format_value<U: Unit>(value: f64) -> String {
    todo!("value の後ろに U::SYMBOL を付けた文字列を返してください")
}

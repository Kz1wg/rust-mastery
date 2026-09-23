pub fn square(x: i64) -> i64 {
    x * x
}

/// ```
/// use ex053_when_to_use_macros::max_of;
/// assert_eq!(max_of!(3, 9, 2), 9);
/// ```
#[macro_export]
macro_rules! max_of {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+) => {{
        let a = $x;
        let b = $crate::max_of!($($rest),+);
        if a > b {
            a
        } else {
            b
        }
    }};
}

//! Lesson 14-1: マクロが必要な瞬間
//!
//! - square は関数で書く（マクロにすると、引数の式が2回評価される）
//! - max_of! は可変長の引数を取るので、マクロでなければ書けない

/// 2乗を返す。関数なので、引数の式は呼び出し前に1回だけ評価される。
pub fn square(x: i64) -> i64 {
    todo!("x の2乗を返してください")
}

/// 可変長の引数から最大値を返すマクロ。
///
/// ```
/// use ex053_when_to_use_macros::max_of;
/// assert_eq!(max_of!(3, 9, 2), 9);
/// ```
#[macro_export]
macro_rules! max_of {
    // 引数が1つのときは、それ自身が最大値
    ($x:expr) => {
        $x
    };
    // 2つ以上のときは、先頭と「残りの最大値」を比べる
    ($x:expr, $($rest:expr),+) => {{
        let a = $x;
        let b = $crate::max_of!($($rest),+);
        // if false { a } は「戻り値の型を a と同じにする」ための骨組み。実装したら消してよい。
        if false {
            a
        } else {
            let _ = &b;
            todo!("a と b の大きい方を返してください")
        }
    }};
}

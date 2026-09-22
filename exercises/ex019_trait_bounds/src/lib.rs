//! Lesson 05-2: trait boundsの設計
//!
//! 元の設計（このままでは実装しない）:
//!
//! ```text
//! fn describe<T>(x: T) -> String
//! where
//!     T: std::fmt::Display + Clone + Default + PartialEq + std::fmt::Debug,
//! {
//!     format!("value: {x}")
//! }
//! ```
//!
//! 本体は Display しか使っていないのに、4つも余計なboundが付いていた。
//! 実際に必要なboundだけを残したシグネチャに直してある。

/// 値を "value: X" という形式の文字列にする。
pub fn describe<T: std::fmt::Display>(x: T) -> String {
    todo!("\"value: \" と x を連結した文字列を返してください")
}

/// 2つの値が等しいかどうかを返す（同じ型であることを要求する）。
///
/// `impl PartialEq` を2つ並べる書き方ではなく、名前付きのgenericパラメータを
/// 使っている理由は、Lesson 05-3 で扱う。
pub fn values_equal<T: PartialEq>(a: T, b: T) -> bool {
    todo!("a と b が等しいかどうかを返してください")
}

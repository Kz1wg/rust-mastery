//! Lesson 02-1: bool と文字列が表現してしまう不正な状態
//!
//! 本文の Bad Example は次のような形だった（このままでは実装しない）:
//!
//! ```text
//! struct Light { red: bool, yellow: bool, green: bool }
//! struct Order { paid: bool, shipped: bool, delivered: bool }
//! ```
//!
//! この演習では、あらかじめ enum に置き換えた**型**を用意してある。
//! 学習者は、その enum を使ったロジック（本体）を実装する。

/// 信号機の状態。青 → 黄 → 赤 → 青 … と循環する。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

impl Light {
    /// 次の状態を返す。遷移順は Green -> Yellow -> Red -> Green。
    pub fn next(self) -> Self {
        todo!("Green -> Yellow -> Red -> Green の順に遷移させてください")
    }
}

/// 注文の状態。4つの状態しか存在しない
/// （元の `Order { paid: bool, shipped: bool, delivered: bool }` では
/// 2^3 = 8通りの組み合わせが書けてしまっていた）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Delivered,
}

/// 全ての状態を1つずつ返す（順序は問わない）。
/// enum のバリアントを増減させたら、この関数もあわせて直すことになる
/// —— 「意味のある状態が何通りあるか」を1箇所で管理する、という意図。
pub fn all_statuses() -> Vec<OrderStatus> {
    todo!("4つの OrderStatus を、重複なく全て返してください")
}

/// 状態を表す短い文字列を返す。
pub fn label(status: OrderStatus) -> &'static str {
    todo!("Pending/Paid/Shipped/Delivered に対応する文字列を返してください")
}

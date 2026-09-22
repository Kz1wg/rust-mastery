//! Lesson 04-2: generic vs trait object
//!
//! 同じ Shape trait を、(A) 静的ディスパッチ版と (B) 動的ディスパッチ版の
//! 両方で使う。(A) は同じ型のスライスしか受け付けられず、(B) は異なる型を
//! 混在できることを、テストで確認する。

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!("半径から面積を計算してください（std::f64::consts::PI を使う）")
    }
}

pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        todo!("一辺の長さから面積を計算してください")
    }
}

/// (A) 静的ディスパッチ: 同じ型のスライスのみ受け付ける。
///
/// `Circle` と `Square` を同じ `Vec` に入れることはできない:
///
/// ```compile_fail
/// use ex014_generic_vs_dyn::{Circle, Square};
///
/// let shapes = vec![Circle { radius: 1.0 }, Square { side: 2.0 }]; // 型が違う
/// ```
pub fn total_area_generic<T: Shape>(shapes: &[T]) -> f64 {
    todo!("shapes の area() を合計してください")
}

/// (B) 動的ディスパッチ: 異なる型を混在できる。
pub fn total_area_dyn(shapes: &[Box<dyn Shape>]) -> f64 {
    todo!("shapes の area() を合計してください")
}

//! Lesson 05-1: genericにするメリットはあるか
//!
//! min_max_i32 と min_max_f64 は同じロジックの重複。1つのgeneric関数に
//! まとめる（この演習では、まとめた後の関数を実装する）。

/// スライスの (最小値, 最大値) を返す。空のスライスなら None。
pub fn min_max<T: PartialOrd + Copy>(list: &[T]) -> Option<(T, T)> {
    todo!("list が空なら None を、そうでなければ (最小値, 最大値) の Some を返してください")
}

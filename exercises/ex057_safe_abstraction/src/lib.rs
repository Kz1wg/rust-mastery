//! Lesson 15-2: safe abstraction を作る
//!
//! どちらの関数も普通の fn（unsafe fn ではない）として公開する。
//! つまり「どんな引数で呼ばれても、メモリが壊れない」ことを、関数自身が保証する。

/// スライスを mid の位置で2つに分け、両方を可変で返す。
/// mid が長さを超えていたら、"out of bounds" を含むメッセージで panic する（メモリは壊さない）。
pub fn my_split_at_mut<T>(s: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = s.len();
    // ここで前提を確かめる（これが無いと、範囲外のメモリを指すスライスを作ってしまう）
    todo!("mid が len 以下であることを assert で確かめ、SAFETY コメントを書いて from_raw_parts_mut で2つのスライスを作ってください")
}

/// 先頭と末尾への可変参照を同時に返す。要素が2つ未満なら None。
pub fn first_and_last_mut<T>(s: &mut [T]) -> Option<(&mut T, &mut T)> {
    if s.len() < 2 {
        return None;
    }
    todo!("my_split_at_mut で分けてから、前半の先頭と後半の末尾を返してください（unsafe を直接使わずに書けます）")
}

//! Lesson 10-1: Send / Sync
//!
//! スレッドに渡す値は Send かつ 'static である必要がある。
//! Rc は Send ではないので渡せない（compile_fail doctest で確認）。

/// data を2つに分け、2つのスレッドでそれぞれ合計し、足し合わせて返す。
///
/// Rc はスレッドに渡せない:
///
/// ```compile_fail
/// use std::rc::Rc;
///
/// let data = Rc::new(vec![1u64, 2, 3]);
/// let d = Rc::clone(&data);
/// std::thread::spawn(move || d.iter().sum::<u64>()); // Rc は Send ではない
/// ```
pub fn spawn_sum(data: Vec<u64>) -> u64 {
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);
    let (left, right) = (left.to_vec(), right.to_vec());
    todo!("left と right をそれぞれ別スレッドで合計し、join した結果を足して返してください")
}

/// 型が Send であることをコンパイル時に確かめるヘルパー。
pub fn assert_send<T: Send>() {}

/// 型が Sync であることをコンパイル時に確かめるヘルパー。
pub fn assert_sync<T: Sync>() {}

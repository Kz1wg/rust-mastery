//! Lesson 07-3: 遅延評価とallocation
//!
//! doubled は Vec ではなく impl Iterator を返す。呼び出し側が collect() するかどうかを選べる。
//!
//! 注意: 「中間の Vec を作っていないこと」はテストでは判定できない。
//! 例えば `nums.iter().map(...).collect::<Vec<_>>().into_iter()` と書いても
//! 型は impl Iterator で、テストは全て通ってしまう。
//! 実装後、自分のコードが途中で collect していないかを自分でレビューすること。
//! 戻り値の `+ '_` の意味は Lesson 07-3 本文を参照。

/// 各要素を2倍にする。中間の Vec を作らない。
pub fn doubled(nums: &[i32]) -> impl Iterator<Item = i32> + '_ {
    nums.iter().map(|x| todo!("xを2倍にした値を返してください"))
}

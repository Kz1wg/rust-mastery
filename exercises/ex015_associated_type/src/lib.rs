//! Lesson 04-3: associated type
//!
//! IntStack は i32 専用のスタックであり、他の型で Stack を実装することは
//! ない。この「1つの型につきItemは1つに決まる」関係を associated type
//! で表現する。

pub trait Stack {
    type Item;

    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
}

pub struct IntStack {
    items: Vec<i32>,
}

impl IntStack {
    pub fn new() -> Self {
        IntStack { items: Vec::new() }
    }
}

impl Default for IntStack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack for IntStack {
    type Item = i32;

    fn push(&mut self, item: i32) {
        todo!("item を self.items に追加してください")
    }

    fn pop(&mut self) -> Option<i32> {
        todo!("self.items の末尾を取り出して返してください")
    }
}

/// S::Item を使う generic 関数。呼び出し側は Item の型を書く必要がない。
pub fn drain_all<S: Stack>(s: &mut S) -> Vec<S::Item> {
    todo!("s から pop() できるだけ取り出し、Vec にして返してください（順序は pop の順）")
}

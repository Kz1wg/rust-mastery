//! Lesson 07-2: Iterator trait
//!
//! Fibonacci は next() だけを実装すれば、take・map・filter などの
//! adapterが全て自動で使えるようになる。

pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        todo!("現在のaを返し、a・bを次のフィボナッチ数に更新してください（常にSomeを返す）")
    }
}

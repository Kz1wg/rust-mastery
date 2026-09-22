//! Lesson 08-2: 構造体が参照を持つ設計
//!
//! `next_word` の戻り値は `Option<&'a str>`。返す単語はパーサ自身ではなく
//! 元の入力（'a）を指しているので、単語を持ったまま次の単語を取り出せる。
//! （`Option<&str>` と書くと、省略規則で `&mut self` に結びついてしまう。）

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    /// 空白で区切られた次の単語を返す。残りが無ければ None。
    pub fn next_word(&mut self) -> Option<&'a str> {
        todo!("self.pos 以降の先頭の空白を飛ばし、次の空白（または末尾）までを返して、self.pos を進めてください")
    }
}

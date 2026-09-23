//! Lesson 12-3: workspaceの設計
//!
//! ドメインのロジック。storage を知らないので、DB もファイルも無しでテストできる。

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(id: u32, title: &str) -> Self {
        Task {
            id,
            title: title.to_string(),
            done: false,
        }
    }

    pub fn complete(&mut self) {
        todo!("done を true にしてください")
    }
}

/// 未完了のタスクの数を数える。
pub fn count_pending(tasks: &[Task]) -> usize {
    todo!("done が false のタスクの数を返してください")
}

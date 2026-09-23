//! core に依存する側。メモリ上の簡易ストレージ。
//! core は storage を知らないので、この crate を変更しても core のテストは影響を受けない。

use ex048_core::Task;

#[derive(Default)]
pub struct MemoryStore {
    tasks: Vec<Task>,
}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore { tasks: Vec::new() }
    }

    pub fn add(&mut self, task: Task) {
        todo!("tasks に task を追加してください")
    }

    pub fn get(&self, id: u32) -> Option<&Task> {
        todo!("id が一致する Task を探して返してください")
    }

    pub fn pending_count(&self) -> usize {
        todo!("core の count_pending を使って未完了の数を返してください")
    }
}

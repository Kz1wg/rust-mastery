//! Lesson 10-3: interior mutability
//!
//! Cell は値をまるごと出し入れする（失敗しない）。
//! RefCell は参照を借りる（借用規則を破ると実行時panic）。

use std::cell::{Cell, RefCell};

pub struct Counter {
    count: Cell<u32>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            count: Cell::new(0),
        }
    }

    /// &self のまま数を増やす。
    pub fn increment(&self) {
        todo!("count の値を1増やしてください（get と set を使う）")
    }

    pub fn get(&self) -> u32 {
        todo!("count の現在の値を返してください")
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Logger {
    entries: RefCell<Vec<String>>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            entries: RefCell::new(Vec::new()),
        }
    }

    /// &self のままログを追加する。
    pub fn log(&self, message: &str) {
        todo!("entries を可変で借りて、message を追加してください")
    }

    pub fn count(&self) -> usize {
        todo!("entries を借りて、要素数を返してください")
    }

    /// 中の参照は外に出せない（借用が終わると無効になるため）。クローンを返す。
    pub fn entries(&self) -> Vec<String> {
        todo!("entries を借りて、その内容のクローンを返してください")
    }

    /// すでに借用されている場合、panic せず false を返す。
    pub fn try_log(&self, message: &str) -> bool {
        todo!("try_borrow_mut を使い、成功したら追加して true、失敗したら false を返してください")
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

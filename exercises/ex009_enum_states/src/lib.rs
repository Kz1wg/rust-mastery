//! Lesson 03-1: enumで状態を表す
//!
//! 元の設計は次の形だった（このままでは実装しない）:
//!
//! ```text
//! struct Connection {
//!     socket: Option<Socket>,
//!     error: Option<String>,
//!     retry_count: u32,
//! }
//! ```
//!
//! `socket` と `error` の組み合わせのうち、意味があるのは
//! 「未接続」「接続中」「失敗（エラーあり）」の3通りだけだった。
//! すでに enum に置き換えてある。学習者はロジック（本体）を実装する。

#[derive(Debug, Clone, PartialEq)]
pub enum Connection {
    Disconnected { retry_count: u32 },
    Connected { retry_count: u32 },
    Failed { retry_count: u32, error: String },
}

impl Connection {
    /// 新規作成時は Disconnected、retry_count は 0。
    pub fn new() -> Self {
        Connection::Disconnected { retry_count: 0 }
    }

    /// 接続する。retry_count は変えない。
    pub fn connect(self) -> Self {
        todo!("どの状態からでも Connected にし、retry_count は保持してください")
    }

    /// 接続に失敗する。retry_count を1増やし、Failed にする。
    pub fn fail(self, error: String) -> Self {
        todo!("retry_count を1増やし、Failed（retry_count と error を持つ）にしてください")
    }

    pub fn is_connected(&self) -> bool {
        todo!("Connected のときだけ true を返してください")
    }

    pub fn retry_count(&self) -> u32 {
        todo!("どの状態からでも retry_count を取り出してください")
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self::new()
    }
}

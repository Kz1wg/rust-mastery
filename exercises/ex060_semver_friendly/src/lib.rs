//! Lesson 16-2: semver と拡張性
//!
//! 3つとも「後から拡張しても、利用者のコードを壊さない」形にしてある。
//! - Config: #[non_exhaustive] なので、利用者は構造体リテラルで作れない（フィールドを足しても壊れない）
//! - Level: #[non_exhaustive] なので、利用者の match にはワイルドカードが必要（バリアントを足しても壊れない）
//! - Store: 新しいメソッドにはデフォルト実装を付ける（利用者の impl を壊さない）
//!
//! 利用者（別crate）は Config を構造体リテラルで作れない:
//!
//! ```compile_fail
//! use ex060_semver_friendly::Config;
//! let c = Config { path: String::new(), retries: 0, verbose: false };
//! ```
//!
//! 利用者の match には、ワイルドカードが必要:
//!
//! ```compile_fail
//! use ex060_semver_friendly::Level;
//! fn f(l: Level) -> u8 {
//!     match l {
//!         Level::Low => 0,
//!         Level::High => 1,
//!     }
//! }
//! ```

/// 設定。`Config::new` と `with_*` メソッドで作る。
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub path: String,
    pub retries: u8,
    pub verbose: bool,
}

impl Config {
    /// path だけを指定して作る。retries は 3、verbose は false になる。
    pub fn new(path: &str) -> Self {
        todo!("path を String にし、retries は 3、verbose は false で作ってください")
    }

    /// 再試行の回数を設定する。
    pub fn with_retries(mut self, retries: u8) -> Self {
        todo!("self.retries を設定して self を返してください")
    }

    /// 詳細な出力をするかどうかを設定する。
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        todo!("self.verbose を設定して self を返してください")
    }
}

/// 重要度。将来バリアントが増えるかもしれないので #[non_exhaustive] にしてある。
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

impl Level {
    /// 表示用の名前。Low は "low"、High は "high"。
    pub fn label(&self) -> &'static str {
        todo!("バリアントごとに表示名を返してください")
    }
}

/// 値を保存する場所。
pub trait Store {
    /// 必須メソッド。実装する側が必ず書く。
    fn get(&self) -> u32;

    /// 後から追加したメソッド。デフォルト実装があるので、既存の実装を壊さない。
    /// "value = N" の形の文字列を返す。
    fn describe(&self) -> String {
        todo!("self.get() を使って、value = N の形の文字列を返してください（format! を使う）")
    }
}

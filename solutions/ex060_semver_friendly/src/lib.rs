//! ```compile_fail
//! use ex060_semver_friendly::Config;
//! let c = Config { path: String::new(), retries: 0, verbose: false };
//! ```
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
    pub fn new(path: &str) -> Self {
        Config {
            path: path.to_string(),
            retries: 3,
            verbose: false,
        }
    }

    pub fn with_retries(mut self, retries: u8) -> Self {
        self.retries = retries;
        self
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

impl Level {
    pub fn label(&self) -> &'static str {
        match self {
            Level::Low => "low",
            Level::High => "high",
        }
    }
}

pub trait Store {
    fn get(&self) -> u32;

    fn describe(&self) -> String {
        format!("value = {}", self.get())
    }
}

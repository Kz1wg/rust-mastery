//! Lesson 12-4: 依存の方向と公開範囲
//!
//! 内部のモジュール構成（config / parser）は隠し、crate のルートに
//! 必要なものだけを pub use で再エクスポートする。
//!
//! 内部モジュールは外から見えない:
//!
//! ```compile_fail
//! let _ = ex049_visibility_and_facade::parser::parse_pair("a=1"); // private module
//! ```
//!
//! pub(crate) のヘルパーも外から見えない:
//!
//! ```compile_fail
//! let _ = ex049_visibility_and_facade::config::normalize_key(" A "); // private module
//! ```

mod config;
mod parser;

// 利用者から見える名前はこの2つだけ。内部構成を変えても壊れない。
pub use config::Config;
pub use parser::parse_config;

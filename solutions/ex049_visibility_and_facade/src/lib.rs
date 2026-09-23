//! ```compile_fail
//! let _ = ex049_visibility_and_facade::parser::parse_pair("a=1"); // private module
//! ```
//!
//! ```compile_fail
//! let _ = ex049_visibility_and_facade::config::normalize_key(" A "); // private module
//! ```

mod config;
mod parser;

pub use config::Config;
pub use parser::parse_config;

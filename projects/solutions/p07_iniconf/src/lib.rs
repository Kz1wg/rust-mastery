//! INI 形式の設定ファイルを読む、小さなライブラリ。
//!
//! ```
//! use p07_iniconf::Document;
//!
//! let text = "
//! name = demo
//!
//! [server]
//! host = 127.0.0.1
//! port = 8080
//! ";
//!
//! let doc: Document = text.parse()?;
//! assert_eq!(doc.get("", "name"), Some("demo"));          // ルートセクションは ""
//! assert_eq!(doc.get("server", "host"), Some("127.0.0.1"));
//!
//! let port: u16 = doc.get_parsed("server", "port")?;      // 型を指定して取り出す
//! assert_eq!(port, 8080);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! 解析に失敗すると、何行目で何が起きたかを返します。
//!
//! ```
//! use p07_iniconf::{Document, ParseErrorKind};
//!
//! let err = Document::parse("[server\nport = 1").unwrap_err();
//! assert_eq!(err.line(), 1);
//! assert_eq!(err.kind(), &ParseErrorKind::UnclosedSection);
//! ```
//!
//! エラーの種類は `#[non_exhaustive]` です。将来種類が増えても壊れないよう、`match` には `_` が必要です。
//!
//! ```compile_fail
//! use p07_iniconf::ParseErrorKind;
//!
//! fn describe(kind: &ParseErrorKind) -> &'static str {
//!     match kind {
//!         ParseErrorKind::UnclosedSection => "unclosed",
//!         ParseErrorKind::EmptySectionName => "empty name",
//!         ParseErrorKind::MissingEquals => "no =",
//!         ParseErrorKind::EmptyKey => "empty key",
//!         ParseErrorKind::DuplicateKey(_) => "dup key",
//!         ParseErrorKind::DuplicateSection(_) => "dup section",
//!         // `_ => ...` がないので、ライブラリの外ではコンパイルできない
//!     }
//! }
//! ```

// 中の構成（どのファイルに何があるか）は非公開にして、使ってほしいものだけを
// crate の直下に並べる（facade、Lesson 12-4）。
// こうしておくと、ファイルの分け方を後で変えても、利用者の `use` は壊れない。
mod document;
mod error;
mod parse;

pub use document::{Document, Section};
pub use error::{GetError, ParseError, ParseErrorKind};

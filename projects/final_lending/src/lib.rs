//! 図書室の貸出管理（Chapter 18 Final Project の例題の参考実装）。
//!
//! モジュールの依存の向き（矢印の先に依存する。逆向きの依存はない）:
//!
//! ```text
//! main.rs → cli ──→ store ──→ library ──→ member ──→ ids
//!             │                  │                    
//!             └──────────────────┴──────→ date
//! ```
//!
//! - `date` / `ids` / `member` / `library`: ドメイン。ファイルも CLI も知らない
//! - `store`: 保存形式。ドメインを知っているが、ドメインは store を知らない
//! - `cli`: 引数の解釈と、ファイルの読み書き。全部を知っている唯一の場所

pub mod cli;
pub mod date;
pub mod ids;
pub mod library;
pub mod member;
pub mod store;

pub use date::Date;
pub use ids::{BookId, MemberId};
pub use library::{Book, BookState, LendingError, Library, Loan, ReturnReceipt};
pub use member::{Member, MemberKind, Policy};

//! Lesson 03-4: `#[non_exhaustive]` と将来の拡張
//!
//! これは「ライブラリ側」のcrate。`Event` に `#[non_exhaustive]` を
//! 付けているので、利用者（`app` crate）は `match` で必ず
//! ワイルドカードの腕（`_ =>`）を書く必要がある。
//!
//! `Scroll` は、`app` がこのコードを書いた**後から**追加されたバリアント
//! だと想定してほしい（`#[non_exhaustive]` のおかげで、これは破壊的変更にならない）。

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    Click { x: i32, y: i32 },
    Key(char),
    Scroll { delta: i32 },
}

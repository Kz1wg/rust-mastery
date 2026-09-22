//! Lesson 03-4: `#[non_exhaustive]` と将来の拡張
//!
//! これは「利用者側」のcrate。`ex012_lib::Event` は `#[non_exhaustive]` なので、
//! ここでの `match` は必ずワイルドカードの腕を書かないとコンパイルできない
//! （試しに `_ =>` を消してみると `error[E0004]` になる）。

use ex012_lib::Event;

/// 既知のイベント（`Click` / `Key`）は説明文を返す。
/// それ以外（`Scroll` を含む、将来追加されるかもしれないイベント）は、
/// 汎用的なメッセージを返す。
pub fn handle(event: Event) -> String {
    todo!("Click と Key はそれぞれ説明文を、それ以外は \"unknown event\" のような汎用メッセージを返してください")
}

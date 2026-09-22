# 演習インデックス（Phase 4 の実装対象）

全12演習（ex001〜ex012）の一覧。仕様の詳細は各 Lesson の Exercise 節を参照。全て実装済み（骨組み・テスト・`exercise.toml`・`solutions/`）。

| ID | Lesson | 題材 | 判定方法 |
| --- | --- | --- | --- |
| ex001_move_semantics | 01-1 | `longest_owned` / `longest_ref` | 通常テスト |
| ex002_borrow_errors | 01-2 | 借用エラーの修正、`split_at_mut` | 通常テスト |
| ex003_ownership_design | 01-3 | シグネチャを自分で決める | 通常テスト（複数の呼び出し形式でコンパイルできること） |
| ex004_compare_signatures | 01-4 | 3つのシグネチャで `dedup_sorted` | 通常テスト |
| ex005_invalid_states | 02-1 | 信号機（`Light::next`）・注文（`OrderStatus`） | 通常テスト（実装済み） |
| ex006_newtype | 02-2 | `Meters` / `Feet` / `From<Feet> for Meters` | 通常テスト＋ `compile_fail` doctest（実装済み） |
| ex007_parse_dont_validate | 02-3 | `Percentage` / `NonEmptyString` | 通常テスト＋ `compile_fail` doctest（実装済み） |
| ex008_visibility | 02-4 | `Temperature`、`Inventory`（予約／解除） | 通常テスト＋ `compile_fail` doctest ×2（実装済み） |
| ex009_enum_states | 03-1 | `Connection`（Disconnected/Connected/Failed） | 通常テスト（実装済み） |
| ex010_state_machine | 03-2 | ドアの状態機械（Open/Closed/Locked） | 全（状態×操作）の網羅テスト（実装済み） |
| ex011_typestate | 03-3 | `RequestBuilder<NoUrl/HasUrl>` | 通常テスト＋ `compile_fail` doctest（実装済み） |
| ex012_non_exhaustive | 03-4 | `ex012_lib`/`ex012_app` の2crate（独立workspace） | 通常テスト（実装済み） |

## 設計上の注意（Phase 4で検討）

- `NOTES.md`（振り返りの記述）は `cargo test` で判定できない。自己チェック用として扱い、`check-exercises` では存在確認のみ行うか検討する。
- ex002 は「借用エラーを含むコード」を配布する予定だったが、実装ではコメント（`//! ```text` ブロック）で Bad Example を示すだけにし、実際のコードとしては配布していない（そのままだとコンパイルできず `cargo test` が起動しないため）。
- ex012 は複数 crate を含むため、他の演習と異なる構造になる（`check-exercises` の対応が必要）。
- ex005・ex008 は、本文（Lesson）が想定した「学習者が型定義そのものを書き換える」という体験を、骨組みでは実現していない（型定義は固定し、メソッド本体のみ `todo!()` にした）。ex003 と同じ判断（ARCHITECTURE.md の演習crate規約に合わせるため）。DESIGN.md の決定ログを参照。
- `todo!()` の本体に `{}` を含むメッセージ文字列を書くと、`format!` の位置引数として解釈されコンパイルエラーになる（ex007・ex009・ex010・ex011 で発生・修正済み。何度も踏んだ罠なので、演習作成時は `todo!()` のメッセージに波かっこを書かないこと）。

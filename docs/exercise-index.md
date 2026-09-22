# 演習インデックス（Phase 4 の実装対象）

Phase 3 の Lesson に記載した演習の一覧。仕様の詳細は各 Lesson の Exercise 節を参照。
Phase 4 で、骨組み（`todo!()`）・テスト・`exercise.toml`・`solutions/` を作成する。

| ID | Lesson | 題材 | 判定方法 |
| --- | --- | --- | --- |
| ex001_move_semantics | 01-1 | `longest_owned` / `longest_ref` | 通常テスト |
| ex002_borrow_errors | 01-2 | 借用エラーの修正、`split_at_mut` | 通常テスト |
| ex003_ownership_design | 01-3 | シグネチャを自分で決める | 通常テスト（複数の呼び出し形式でコンパイルできること） |
| ex004_compare_signatures | 01-4 | 3つのシグネチャで `dedup_sorted` | 通常テスト |
| ex005_invalid_states | 02-1 | 信号機・注文の enum 化 | 通常テスト |
| ex006_newtype | 02-2 | `Meters` / `Feet` | 通常テスト＋ `compile_fail` doctest |
| ex007_parse_dont_validate | 02-3 | `Percentage` / `NonEmptyString` | 通常テスト＋ `compile_fail` doctest |
| ex008_visibility | 02-4 | `Temperature`、API監査（`Inventory`） | 通常テスト＋ `compile_fail` doctest |
| ex009_enum_states | 03-1 | `Connection` の enum 化 | 通常テスト |
| ex010_state_machine | 03-2 | ドアの状態機械 | 全（状態×操作）の網羅テスト |
| ex011_typestate | 03-3 | `RequestBuilder` | `compile_fail` doctest |
| ex012_non_exhaustive | 03-4 | 2 crate の workspace | 通常テスト（`app` 側の `_ =>` の挙動） |

## 設計上の注意（Phase 4で検討）

- `NOTES.md`（振り返りの記述）は `cargo test` で判定できない。自己チェック用として扱い、`check-exercises` では存在確認のみ行うか検討する。
- ex002 は「借用エラーを含むコード」を配布する。そのままだとコンパイルできず `cargo test` が起動しないため、エラー箇所はコメントアウトして配布する。
- ex012 は複数 crate を含むため、他の演習と異なる構造になる（`check-exercises` の対応が必要）。

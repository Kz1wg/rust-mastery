# 12 Module & Architecture

コードが増えてきたとき、「どこに何を置くか」を決めるのがこの章の主題です。
判断の軸は一貫しています——**依存の方向を一方向に保ち、公開する範囲を必要最小限にする**こと。

この章では、**この教材リポジトリ自身**も実例として使います。

## この章の到達目標

- 「ファイルを分ける」と「責務を分ける」の違いを説明できる
- `lib.rs` と `main.rs` を分け、テストしやすい構造にできる
- workspace を使うべき場面と、crate の境界の引き方を説明できる
- `pub` / `pub(crate)` / `pub use` を使い分け、公開APIを設計できる

## Lesson一覧

| Lesson | 問い |
| --- | --- |
| [12-1 module分割の基準](12-1-module-boundaries.md) | ファイルを分けることと責務を分けることの違い |
| [12-2 lib / bin の分離](12-2-lib-and-bin.md) | main.rs に何を置き、何を置かないか |
| [12-3 workspaceの設計](12-3-workspaces.md) | crate境界はどこに引くか |
| [12-4 依存の方向と公開範囲](12-4-dependencies-and-visibility.md) | `pub` は誰との約束か |

# 11 Async Rust

`async` / `await` は「軽いスレッド」ではありません。コンパイラが関数を**状態機械に変換**し、
それを**ランタイムが繰り返し呼び出す**仕組みです。

この章では、**依存crateを使わずに**最小のランタイムを自分で書き、
「`await` したとき何が起きているか」を手で確かめます。その上で、実務で使う tokio に触れます。

## この章の到達目標

- `Future` と `poll` / `Waker` の関係を説明でき、最小のランタイムを読める
- `Pin` / `Unpin` が必要になった理由（自己参照する状態機械）を説明できる
- `await` をまたぐ所有権・借用の制約と、`Send` な `Future` の条件を説明できる
- async とスレッドを、問題の性質（I/O待ちかCPU計算か）で選べる
- tokio が何を提供しているかを説明できる

## Lesson一覧

| Lesson | 問い |
| --- | --- |
| [11-1 `Future` とは何か](11-1-what-is-a-future.md) | async/awaitは何に展開されるのか |
| [11-2 `Pin` / `Unpin`](11-2-pin.md) | なぜ「動かない」ことが必要なのか |
| [11-3 asyncと所有権](11-3-async-and-ownership.md) | `Send` なFutureとは何か |
| [11-4 asyncを使うべきか](11-4-async-vs-threads.md) | スレッドと比べたtrade-off、そしてtokio |

# 10 Concurrency

Rust の並行性は、新しい機能というより **Chapter 01 の所有権と借用の規則を、スレッドに延長したもの**です。
「同時に複数の読み手か、1人の書き手か」という規則（01-2）が、そのままデータ競合の防止になっています。

## この章の到達目標

- `Send` と `Sync` が何を保証し、なぜ自動で実装されるのかを説明できる
- `Rc` / `Arc` / `Mutex` / `RwLock` を、共有と可変性の観点で使い分けられる
- interior mutability（`Cell` / `RefCell`）が、借用チェックを実行時に移す仕組みだと説明できる
- 共有する設計と、メッセージで渡す設計を比べて選べる

## Lesson一覧

| Lesson | 問い |
| --- | --- |
| [10-1 `Send` / `Sync`](10-1-send-sync.md) | 何を保証し、なぜ自動traitなのか |
| [10-2 共有と可変性](10-2-arc-mutex.md) | 共有可変状態をどう設計するか |
| [10-3 interior mutability](10-3-interior-mutability.md) | 実行時の借用チェックは何を引き受けるか |
| [10-4 メッセージパッシング](10-4-message-passing.md) | 共有しない、という選択肢 |

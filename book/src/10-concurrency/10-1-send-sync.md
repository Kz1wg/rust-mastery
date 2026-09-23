# Lesson 10-1: `Send` / `Sync`

## Concept

- **`Send`**: その型の値を、**別のスレッドへ移動**してよい
- **`Sync`**: その型への**参照を、複数のスレッドで共有**してよい（`&T` が `Send` であることと同じ）

どちらも自分で実装するものではなく、**中身から自動的に決まります**。

## Why?

「なぜこの型はスレッドに渡せないのか」というエラーは、この2つの trait で説明されます。
意味を知らないと、`Arc` で包むなど場当たり的な対処に走りがちです。

## Bad Example

```rust,compile_fail,E0277
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);
    let d = Rc::clone(&data);

    std::thread::spawn(move || {
        println!("{d:?}");
    });
}
```

## Problem

`Rc` の参照カウントは、**排他制御なしの単純な足し算・引き算**です。
2つのスレッドが同時に `Rc::clone` や drop をすると、カウントが壊れ、
早すぎる解放や解放漏れが起きます。そのため `Rc` は `Send` ではありません。

## Think

> **問い**:
> 1. `Rc` の代わりに何を使えばよいですか？
> 2. `Send` / `Sync` を**自分で実装する**のではなく、コンパイラが自動で決めるのはなぜでしょうか？

<details>
<summary>Solution</summary>

1. `Arc`（Atomic Reference Counted）です。カウントの増減がアトミック操作になっており、複数スレッドから安全に扱えます。

```rust
use std::sync::Arc;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let d = Arc::clone(&data);

    let handle = std::thread::spawn(move || {
        println!("{d:?}");
    });
    handle.join().unwrap();
}
```

2. `Send` / `Sync` は**構造から決まる性質**だからです。「すべてのフィールドが `Send` なら、その構造体も `Send`」
という規則で自動的に決まります（auto trait）。自分で書く必要があるのは、`unsafe` を使って
コンパイラには見えない保証を自分で用意した場合だけです（`unsafe impl Send for ...`、Chapter 15）。

| 型 | `Send` | `Sync` | 理由 |
| --- | --- | --- | --- |
| `i32`, `String`, `Vec<T>`（`T` が満たすなら） | ○ | ○ | 特別な共有状態を持たない |
| `Rc<T>` | × | × | 参照カウントが非アトミック |
| `Arc<T>`（`T: Send + Sync`） | ○ | ○ | カウントがアトミック |
| `RefCell<T>` | ○ | **×** | 借用フラグが非アトミック。**移動はできるが共有はできない** |
| `Mutex<T>`（`T: Send`） | ○ | ○ | 排他制御を持っている |
| `MutexGuard` | × | ○ | ロックを取ったスレッドで解放する必要がある |

`RefCell` が「`Send` だが `Sync` ではない」のは、この2つが別の概念であることを示す良い例です。
**まるごと別スレッドへ渡すのは安全**（元のスレッドからは使えなくなるため）ですが、
**参照を2つのスレッドで共有するのは危険**（借用フラグが壊れる）です。

```rust,compile_fail,E0277
use std::cell::RefCell;

fn is_sync<T: Sync>() {}

fn main() {
    is_sync::<RefCell<u8>>();
}
```

</details>

## Deep Dive: もう1つの条件 `'static`

スレッドに渡すクロージャには、`Send` のほかに `'static` も要求されます（Lesson 08-3）。

```rust,compile_fail,E0373
fn main() {
    let data = vec![1, 2, 3];
    std::thread::spawn(|| {
        println!("{data:?}");
    });
}
```

`thread::spawn` で作ったスレッドは、`spawn` を呼んだ関数より長く生きるかもしれません。
借用した `data` がいつまで有効かを保証できないので、`move` で所有権ごと渡すか、
`Arc` で共有する必要があります。

> **補足**: 「元のスコープを超えない」ことが保証できる仕組みとして、標準ライブラリには
> `std::thread::scope`（scoped threads）があります。これを使うと、ローカル変数を借用したまま
> スレッドを起動できます。この教材では扱いませんが、`'static` が邪魔になったときの選択肢として覚えておいてください。

## Exercise

**`ex038_send_sync`** — `cargo test -p ex038_send_sync` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `spawn_sum` | `Vec<u64>` を2つに分け、2つのスレッドで合計して足し合わせる。`Rc` を渡そうとするとコンパイルできないことは `compile_fail` doctest で確認 |
| `assert_send` / `assert_sync` | 型が `Send` / `Sync` かをコンパイル時に確かめるヘルパー |

## Challenge

`Mutex<Rc<i32>>` は `Sync` でしょうか。理由を、上の表の規則から説明してください。

## Review

- [ ] `Send` と `Sync` の違いを説明できる
- [ ] `Rc` が `Send` でない理由を説明できる
- [ ] `RefCell` が `Send` だが `Sync` でない理由を説明できる
- [ ] スレッドに渡すクロージャが `'static` を要求される理由を説明できる

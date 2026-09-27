# Lesson 11-3: asyncと所有権

## Concept

11-1 で見たとおり、**`.await` をまたいで生きている変数は、状態機械の中に保存されます**。
だから「`Future` が `Send` かどうか」は、**`.await` をまたいで何を持っているか**で決まります。

## Why?

マルチスレッドのランタイム（tokio の既定）にタスクを渡すには、その `Future` が `Send` である必要があります。
`Send` でない値を `.await` をまたいで持っていると、そこで弾かれます。

## Bad Example: `Rc` を `.await` をまたいで持つ

```rust,compile_fail
use std::rc::Rc;

fn assert_send<T: Send>(_: T) {}

async fn inner() {}

fn main() {
    let future = async {
        let counter = Rc::new(1);
        inner().await; // ここで状態機械に counter が保存される
        println!("{counter}");
    };
    assert_send(future);
}
```

## Problem

エラーは `future cannot be sent between threads safely` となり、
「`Rc` が `.await` をまたいで保持されている」ことを指摘してくれます。

`Rc` は `Send` ではありません（10-1）。それを状態機械が抱えている以上、
その状態機械（＝`Future`）自体も `Send` になれません。

## Think

> **問い**: 次の2つは、どちらも `Rc` を使っています。片方だけ `Send` な `Future` になります。どちらでしょうか。

```rust,ignore
// (A)
async {
    {
        let counter = Rc::new(1);
        println!("{counter}");
    }                        // ここでスコープが閉じ、counter は drop される
    inner().await;
}

// (B)
async {
    let counter = Rc::new(1);
    println!("{counter}");
    inner().await;           // counter はまだ生きている（このブロックの終わりまで）
}
```

<details>
<summary>Solution</summary>

**(A) が `Send` です。** `counter` は内側のブロックが閉じた時点で drop されるので、
`.await` の地点では存在せず、状態機械にも保存されません。

(B) は「`println!` で使い終わっている」ように見えますが、**`counter` はブロックの終わりまで生きています**。
`.await` の地点でまだ生存しているので、状態機械に保存され、`Future` は `Send` でなくなります。

```rust
use std::rc::Rc;

fn assert_send<T: Send>(_: T) {}

async fn inner() {}

fn main() {
    let a = async {
        {
            let counter = Rc::new(1);
            println!("{counter}");
        } // ここで drop される
        inner().await;
    };
    assert_send(a); // OK
}
```

**注意（罠）**: `drop(counter)` を明示的に呼ぶだけでは**足りません**。

```rust,compile_fail
use std::rc::Rc;

fn assert_send<T: Send>(_: T) {}

async fn inner() {}

fn main() {
    let b = async {
        let counter = Rc::new(1);
        println!("{counter}");
        drop(counter); // 明示的に drop しても……
        inner().await;
    };
    assert_send(b); // これは通らない
}
```

コンパイラの判定は**スコープを単位に**しており、制御フローを追って「もう drop 済みだ」と判断してくれません
（Rust 1.75 で確認）。`Send` にしたいなら、**ブロックで囲んでスコープを閉じる**のが確実です。

境目は「`.await` の地点で、その変数がまだスコープの中にいるか」です。
複数スレッドで動かしたいなら、`Rc` を `Arc` に変えるのが最も素直な解決です。

</details>

## 実務で最も多い形: ロックを `.await` をまたいで持つ

```rust,ignore
// 危険な形
let mut guard = mutex.lock().unwrap();
*guard += 1;
some_async_io().await;   // ロックを持ったまま待つ
println!("{guard}");
```

これには2つの問題があります。

1. `std::sync::MutexGuard` は `Send` ではないので、**マルチスレッドのランタイムに渡せません**（コンパイルエラー）
2. 仮に渡せたとしても、**I/Oを待っている間ずっとロックを占有**し、他のタスクを止めます

正しい形は、**`.await` の前にロックを手放す**ことです。

```rust,ignore
{
    let mut guard = mutex.lock().unwrap();
    *guard += 1;
} // ここでロックが解放される
some_async_io().await;
```

（非同期の世界で「ロックを持ったまま待ちたい」場合は、`tokio::sync::Mutex` のような
非同期対応のロックを使います。ただし、まずは「`.await` をまたがない設計」を検討してください。）

## `'static` が要求される理由

タスクをランタイムに渡す関数（tokio の `spawn` など）は、`Future` に `Send + 'static` を要求します。
理由は `thread::spawn` と同じです（10-1）——タスクがいつまで実行されるか、渡す側には分からないからです。

そのため、**借用を `.await` をまたいで持ち歩く設計**はうまくいかないことが多く、
`Arc` で共有するか、所有権ごと渡す形に落ち着きます。

## Exercise

**[`ex044_async_ownership`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex044_async_ownership)** — `cargo test -p ex044_async_ownership` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `sum_shared` | `Arc<Vec<u64>>` を受け取る `async fn`。`.await` をまたいで `Arc` を保持しても `Send` のまま |
| `assert_send_future` | `Future` が `Send` であることをコンパイル時に確かめるヘルパー |

`compile_fail` doctest で、`Rc` を `.await` をまたいで持つと `Send` でなくなることを確認しています。

## Challenge

`sum_shared` の `Arc` を `Rc` に変え、`assert_send_future` に渡してみてください。
エラーメッセージのどこに「`.await` をまたいで保持されている」ことが書かれているか探してください。

## Review

- [ ] `Future` が `Send` かどうかが、`.await` をまたぐ値で決まることを説明できる
- [ ] 同じ `Rc` でも、`.await` をまたぐかどうかで結果が変わることを説明できる
- [ ] ロックを `.await` をまたいで持つことの2つの問題を説明できる

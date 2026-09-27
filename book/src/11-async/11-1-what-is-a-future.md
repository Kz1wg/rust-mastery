# Lesson 11-1: `Future` とは何か

## Concept

フードコートで料理を注文すると、呼び出しベルを渡されます。

- 料理ができるまで、席で別のことをしていられる
- できたらベルが鳴るので、取りに行く

`async` で書いた処理も、これと同じ形で動きます。

| フードコート | Rust の async |
| --- | --- |
| 注文票（まだ料理になっていない） | `Future`（まだ終わっていない処理） |
| 「できましたか？」と確認する | `poll`（進められるところまで進めてもらう） |
| 「まだです」「できました」 | `Poll::Pending` / `Poll::Ready(値)` |
| 呼び出しベル | `Waker`（準備ができたら知らせる仕組み） |
| 何度も確認しに来てくれる店員 | ランタイム |

`Future` は、**それ自体では何もしません**。誰かが「進みましたか？」と `poll` を呼んで初めて、少しずつ進みます。
その `poll` を繰り返し呼ぶ役目を持っているのが**ランタイム**です。

実際の `Future` の定義は、次のメソッドを1つ持つ trait です（`Pin` は 11-2 で説明するので、今は気にしなくて構いません）。

```rust,ignore
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

「進めるところまで進めて、終わったら `Ready(値)`、まだなら `Pending` を返す」関数です。

## Why?

この仕組みを知らないと、「`async fn` を呼んだのに何も起きない」「`.await` を付け忘れて静かに無視された」
といった現象の理由が分かりません。

## Bad Example: 呼んだのに実行されない

```rust
async fn work() -> u32 {
    println!("running");
    42
}

fn main() {
    work(); // 何も表示されない（警告は出る）
    println!("done");
}
```

## Problem

`async fn` を呼んでも、関数の中身は実行されません。返ってくるのは**「実行方法を持った値」**（`Future`）だけです。
`poll` されて初めて中身が動きます。スレッドと決定的に違う点です。

## Think

> **問い**: この `Future` を実際に動かすには、何が必要ですか？
> `poll` を呼ぶとき、引数の `Context` には何を渡せばよいでしょうか。

<details>
<summary>Hint</summary>

`poll` が `Pending` を返したとき、ランタイムは「いつ再度 `poll` すべきか」を知る必要があります。
そのために `Context` は `Waker`（「準備ができたら起こしてくれ」という通知口）を持っています。

</details>

<details>
<summary>Solution</summary>

最小のランタイム（`block_on`）は、標準ライブラリだけで書けます。

```rust
use std::future::Future;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Wake, Waker};

/// 「起きろ」という通知を受け取るだけの仕組み
struct Signal {
    woken: Mutex<bool>,
    cv: Condvar,
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        *self.woken.lock().unwrap() = true;
        self.cv.notify_one();
    }
}

/// Future を完了まで実行する。最小のランタイム。
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = Box::pin(future); // 動かないよう固定する（11-2）
    let signal = Arc::new(Signal {
        woken: Mutex::new(false),
        cv: Condvar::new(),
    });
    let waker = Waker::from(Arc::clone(&signal));
    let mut cx = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => {
                // 起こされるまで待つ
                let mut woken = signal.woken.lock().unwrap();
                while !*woken {
                    woken = signal.cv.wait(woken).unwrap();
                }
                *woken = false;
            }
        }
    }
}

async fn work() -> u32 {
    println!("running");
    42
}

fn main() {
    assert_eq!(block_on(work()), 42); // ここで初めて "running" が表示される
}
```

ランタイムがやっているのは、突き詰めれば次の3つだけです。

1. `Future` を `poll` する
2. `Pending` が返ったら、`Waker` で起こされるまで待つ
3. 起こされたら、また `poll` する

tokio のような本物のランタイムは、これに加えて「複数のタスクを管理する」「OSのI/O通知（epoll等）と
`Waker` を結びつける」「スレッドプールに分配する」といった仕事をします。**核心は同じ**です。

</details>

## 自分で `Future` を実装してみる

```rust
# use std::future::Future;
# use std::sync::{Arc, Condvar, Mutex};
# use std::task::{Wake, Waker};
# struct Signal { woken: Mutex<bool>, cv: Condvar }
# impl Wake for Signal {
#     fn wake(self: Arc<Self>) { *self.woken.lock().unwrap() = true; self.cv.notify_one(); }
# }
# pub fn block_on<F: Future>(future: F) -> F::Output {
#     let mut future = Box::pin(future);
#     let signal = Arc::new(Signal { woken: Mutex::new(false), cv: Condvar::new() });
#     let waker = Waker::from(Arc::clone(&signal));
#     let mut cx = Context::from_waker(&waker);
#     loop {
#         match future.as_mut().poll(&mut cx) {
#             Poll::Ready(v) => return v,
#             Poll::Pending => {
#                 let mut w = signal.woken.lock().unwrap();
#                 while !*w { w = signal.cv.wait(w).unwrap(); }
#                 *w = false;
#             }
#         }
#     }
# }
use std::pin::Pin;
use std::task::{Context, Poll};

/// n 回 Pending を返してから Ready になる Future
struct Countdown {
    n: u32,
}

impl Future for Countdown {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.n == 0 {
            Poll::Ready("finished")
        } else {
            self.n -= 1;
            cx.waker().wake_by_ref(); // 「すぐまた poll してくれ」
            Poll::Pending
        }
    }
}

fn main() {
    assert_eq!(block_on(Countdown { n: 3 }), "finished");
}
```

`wake_by_ref()` を呼び忘れると、`block_on` は「起こされるのを待つ」まま**永久に止まります**。
`Pending` を返す側には、**必ず後で起こす責任がある**——これが `Future` の契約です。

## Deep Dive: `async fn` は状態機械になる

```rust,ignore
async fn example() -> u32 {
    let a = step_one().await;
    let b = step_two(a).await;
    a + b
}
```

`.await` のたびに処理は一時停止し、後で `poll` されたときに**続きから**再開します。
続きから再開するには、「今どこまで進んだか」と「それまでに作った変数」を覚えておく必要があります。

コンパイラは、その「覚えておく箱」を自動で作ります。おおよそ次のような `enum` です。
（このように「今どの状態にいるか」と「状態から状態への移り方」でできた仕組みを**状態機械**と呼びます。
Lesson 03-2 で自分で作ったものと同じ考え方です。）

```text
enum ExampleState {
    Start,
    WaitingOnStepOne { /* 必要な変数 */ },
    WaitingOnStepTwo { a: u32, /* ... */ },
    Done,
}
```

`poll` が呼ばれるたびに、「今どの状態か」を見て続きから再開します。
`.await` の地点が、そのまま**状態の切れ目**です。
そして「`.await` をまたいで生きている変数」は、この `enum` の中に保存されます——
この事実が、11-2（`Pin`）と11-3（所有権）の話につながります。

## Exercise

**[`ex042_future_basics`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex042_future_basics)** — `cargo test -p ex042_future_basics` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `Countdown` | `Future` を実装する（`wake_by_ref` を忘れると止まるので注意） |
| `run_add` | `async fn add` を `block_on` で実行する |
| `poll_count` | 完了までに `poll` が何回呼ばれたかを数える |

`block_on` は用意済みです（上のコードと同じもの）。

## Challenge

`Countdown::poll` から `cx.waker().wake_by_ref()` を削除して、テストを実行してみてください。
（止まるので `Ctrl+C` で中断してください。）なぜ止まるのか、`block_on` のコードを見て説明してください。

## Review

- [ ] `async fn` を呼んだだけでは何も実行されない理由を説明できる
- [ ] ランタイムの仕事が「poll する / 待つ / 起こされたらまた poll する」だと説明できる
- [ ] `Pending` を返す側に「後で起こす責任」があることを説明できる
- [ ] `async fn` が状態機械に変換され、`.await` が状態の切れ目になることを説明できる

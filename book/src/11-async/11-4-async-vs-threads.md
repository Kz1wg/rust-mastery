# Lesson 11-4: asyncを使うべきか

## Concept

async は「速くなる魔法」ではありません。**待ち時間の多い仕事を、少ないスレッドで大量に抱える**ための仕組みです。
CPUを使い切る計算には、ほとんど役に立ちません。

## Why?

「非同期のほうが速い」と思い込んで async を導入すると、複雑さだけが増え、
場合によっては遅くなります（ランタイムの管理コスト、タスク切り替え、`Arc` の増加）。

## 判断の軸: 待っているのか、計算しているのか

| 仕事の性質 | 向いている道具 | 理由 |
| --- | --- | --- |
| **I/O待ちが大半**（ネットワーク、ファイル、DB）で、同時接続が多い | async | 待っている間、スレッドを占有しない。1スレッドで数千の接続を扱える |
| **CPU計算が大半**（画像処理、集計、暗号） | スレッド（`std::thread`、`rayon`） | 待ち時間が無いので、async にしても得がない。むしろオーバーヘッド |
| 同時実行数が**少ない**（数個〜数十） | スレッド | スレッドで足りる。async の複雑さが見合わない |
| CLIツールなど、非同期の必要がない | どちらも不要 | 同期のコードが最も単純 |

**スレッドが足りなくなって初めて async を検討する**、という順番で十分です。
1万の同時接続を1万スレッドで捌くのは現実的ではありません（スタックだけでメモリを使い切ります）が、
10個の並行処理ならスレッドで十分です。

## 並行 ≠ 並列

async の重要な性質は、**1つのスレッドの中でも複数の仕事を交互に進められる**ことです。

```rust
# use std::future::Future;
# use std::pin::Pin;
# use std::sync::{Arc, Condvar, Mutex};
# use std::task::{Context, Poll, Wake, Waker};
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
# struct Join2<A: Future, B: Future> { a: Pin<Box<A>>, b: Pin<Box<B>>, a_out: Option<A::Output>, b_out: Option<B::Output> }
# fn join2<A: Future, B: Future>(a: A, b: B) -> Join2<A, B> {
#     Join2 { a: Box::pin(a), b: Box::pin(b), a_out: None, b_out: None }
# }
# impl<A: Future, B: Future> Future for Join2<A, B> where A::Output: Unpin, B::Output: Unpin {
#     type Output = (A::Output, B::Output);
#     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
#         let me = self.get_mut();
#         if me.a_out.is_none() {
#             if let Poll::Ready(v) = me.a.as_mut().poll(cx) { me.a_out = Some(v); }
#         }
#         if me.b_out.is_none() {
#             if let Poll::Ready(v) = me.b.as_mut().poll(cx) { me.b_out = Some(v); }
#         }
#         if me.a_out.is_some() && me.b_out.is_some() {
#             Poll::Ready((me.a_out.take().unwrap(), me.b_out.take().unwrap()))
#         } else { Poll::Pending }
#     }
# }
# struct Steps { n: u32, label: &'static str, log: Arc<Mutex<Vec<&'static str>>> }
# impl Future for Steps {
#     type Output = u32;
#     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
#         self.log.lock().unwrap().push(self.label);
#         if self.n == 0 { return Poll::Ready(0); }
#         self.n -= 1;
#         cx.waker().wake_by_ref();
#         Poll::Pending
#     }
# }
fn main() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let a = Steps { n: 2, label: "a", log: Arc::clone(&log) };
    let b = Steps { n: 2, label: "b", log: Arc::clone(&log) };

    block_on(join2(a, b));

    // スレッドは1つだけなのに、a と b が交互に進んでいる
    assert_eq!(*log.lock().unwrap(), vec!["a", "b", "a", "b", "a", "b"]);
}
```

**並行（concurrency）**は「複数の仕事を切り替えながら進めること」、
**並列（parallelism）**は「複数のCPUで同時に走らせること」です。
async が直接提供するのは前者で、後者はランタイムがスレッドプールを持って初めて実現します。

## 実務のランタイム: tokio

ここまで書いた `block_on` は、1つの `Future` を完了まで走らせるだけの最小実装です。
実務では **tokio** を使うのが一般的です（2026年9月時点で 1.x 系が現行。
1.51.x と 1.53.x が長期サポート版）。

```toml
[dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros", "net", "time"] }
```

```rust,ignore
#[tokio::main] // main を async にし、ランタイムを起動する
async fn main() {
    // タスクを起こす（Future は Send + 'static である必要がある → 11-3）
    let handle = tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        "done"
    });

    // 複数を同時に待つ（自作した join2 の完成版）
    let (a, b) = tokio::join!(async { 1 }, async { 2 });
    assert_eq!(a + b, 3);

    assert_eq!(handle.await.unwrap(), "done");
}
```

tokio が提供しているのは、この章で自作しなかった部分です。

| 機能 | この章の `block_on` | tokio |
| --- | --- | --- |
| `Future` を完了まで実行 | ○ | `block_on` / `#[tokio::main]` |
| 複数タスクの管理 | ×（1つだけ） | `tokio::spawn`。複数のスレッドに仕事を振り分け、手の空いたスレッドが他の仕事を引き取る（work-stealing） |
| OSのI/O通知（epoll など）との連携 | × | 「データが届いた」などのOSからの通知を受け取り、対応する `Waker` を鳴らす仕組み（reactor）を持つ |
| 非同期のネットワーク・ファイル・タイマー | × | `tokio::net` / `fs` / `time` |
| 非同期対応の同期プリミティブ | × | `tokio::sync::{Mutex, mpsc, oneshot}` |

**使うときの注意**:

- `features` は必要なものだけ有効にする（`full` は手軽ですが、ビルド時間とバイナリが膨らみます）
- **ランタイムの中で重いCPU計算やブロッキング呼び出しをしない**。ワーカースレッドを占有して、
  他のタスクが止まります。必要なら `tokio::task::spawn_blocking` で逃がします
- ライブラリを書くときは、特定のランタイムに依存しないか、依存するなら明記する
  （これは Chapter 16 の API 設計の話でもあります）

この教材の演習は依存crateを増やさない方針なので tokio は使いませんが、
**Chapter 17 の実践プロジェクト（非同期データ取得）で実際に使います**。

## Deep Dive: 「async は伝染する」

`async fn` を呼べるのは `async` な文脈だけです。そのため、深いところで1つ async にすると、
呼び出し元がすべて `async` になっていきます（"function coloring" と呼ばれる問題）。

これは async を導入するかどうかの判断が、**局所的な選択ではなく設計全体の選択**であることを意味します。
「この関数だけ非同期にする」は、たいてい成立しません。

## Exercise

**[`ex045_join2`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex045_join2)** — `cargo test -p ex045_join2` で判定します。

`Join2` の `poll` を実装します（`Future` を2つ持ち、両方が `Ready` になったら結果の組を返す）。
テストでは、**スレッドを1つも使わずに**2つの処理が交互に進むことを、ログの順序で確認します。

## Challenge

`join2` を3つ以上に一般化するにはどうすればよいでしょうか。
`Vec<Pin<Box<dyn Future<Output = T>>>>` を持つ `JoinAll` を書いてみてください
（tokio の `join_all` が、まさにこれを提供しています）。

## Review

- [ ] async が「I/O待ちが多く、同時実行数が多い」場合の道具だと説明できる
- [ ] 並行と並列の違いを説明できる
- [ ] tokio が提供している機能を、自作の `block_on` との差として説明できる
- [ ] ランタイムの中でブロッキング処理をしてはいけない理由を説明できる
- [ ] async が呼び出し元へ伝染することを理解している

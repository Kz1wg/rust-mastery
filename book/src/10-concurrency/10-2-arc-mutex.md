# Lesson 10-2: 共有と可変性

## Concept

Rust の規則は「同時に、複数の読み手か、1人の書き手か」（01-2）です。
複数のスレッドが**同じデータを変更したい**とき、この規則をコンパイル時に守らせることはできません。
そこで、**実行時に守らせる道具**が `Mutex` と `RwLock` です。

## Why?

`Arc<Mutex<T>>` という組み合わせは定型句として広まっていますが、
`Arc` と `Mutex` がそれぞれ別の問題を解いていることを理解していないと、
`Mutex<Arc<T>>` のように意味の違う書き方をしてしまいます。

## 2つの問題、2つの道具

| 問題 | 道具 |
| --- | --- |
| 1つの値を、複数のスレッドが**所有**したい（誰が最後に解放するか決められない） | `Arc<T>` |
| 1つの値を、複数のスレッドが**変更**したい（同時の書き込みを防ぎたい） | `Mutex<T>` / `RwLock<T>` |

両方必要なので `Arc<Mutex<T>>` になります。順序が逆の `Mutex<Arc<T>>` は
「共有できない箱の中に、共有可能なポインタが入っている」形で、ふつうは意図と違います。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut guard = c.lock().unwrap();
                *guard += 1;
            } // guard がここで drop され、ロックが解放される
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(*counter.lock().unwrap(), 4000);
}
```

`lock()` が返す `MutexGuard` は、**drop されたときにロックを解放します**。
明示的な `unlock` はありません。`guard` という変数がスコープを抜けると、自動でロックが解放されます。
「作ったときに確保し、捨てたときに自動で後片付けする」このやり方を **RAII** と呼びます
（ファイルが閉じられる、メモリが解放される、なども同じ仕組みです）。解放し忘れが起こらない設計です。

## Think

> **問い**: 上のコードで、`lock()` の戻り値が `Result` なのはなぜでしょうか。どんなときに `Err` になりますか？

<details>
<summary>Solution</summary>

ロックを持っているスレッドが**panic した**場合、その `Mutex` は **poisoned（毒されている）** 状態になります。
中のデータが「変更の途中で放置された」可能性があるためです。以降の `lock()` は `Err` を返します。

```rust,no_run
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(1);
    let _ = std::panic::catch_unwind(|| {
        let _guard = m.lock().unwrap();
        panic!("boom");
    });

    assert!(m.is_poisoned());
    assert!(m.lock().is_err());
}
```

`unwrap()` で済ませているコードが多いのは、「壊れているかもしれないデータを使い続けるより、
このスレッドも落ちるほうがよい」という判断です。回復させたい場合は
`Err` から `into_inner()` で中身を取り出せますが、**データが半端な状態かもしれない**ことを
理解したうえで扱う必要があります。

</details>

## `RwLock`: 読み手が多いとき

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let config = Arc::new(RwLock::new(vec!["a".to_string()]));

    let readers: Vec<_> = (0..3)
        .map(|_| {
            let c = Arc::clone(&config);
            thread::spawn(move || c.read().unwrap().len())
        })
        .collect();

    let total: usize = readers.into_iter().map(|h| h.join().unwrap()).sum();
    assert_eq!(total, 3);

    config.write().unwrap().push("b".to_string());
    assert_eq!(config.read().unwrap().len(), 2);
}
```

| | `Mutex` | `RwLock` |
| --- | --- | --- |
| 同時に読める数 | 1 | 複数 |
| 同時に書ける数 | 1 | 1（読み手がいない間だけ） |
| 向いている場面 | 読み書きが同程度、短い操作 | 読みが圧倒的に多い |
| 注意 | — | 実装が複雑な分オーバーヘッドがあり、書き手が待たされ続けることもある |

**迷ったら `Mutex`** です。`RwLock` は「読みが大半」という測定できる理由があるときに選びます。

## Deep Dive: デッドロックはコンパイラが防いでくれない

Rust はデータ競合を防ぎますが、**デッドロックは防ぎません**。
2つのロックを、スレッドごとに違う順序で取ると止まります。

```text
スレッドA: lock(1) → lock(2) を待つ
スレッドB: lock(2) → lock(1) を待つ   ← 両方とも永久に進まない
```

対策は昔から変わりません。

- **ロックの取得順序を、プログラム全体で統一する**
- ロックを持ったまま、他のロックを取ったり、時間のかかる処理や外部呼び出しをしない
- そもそもロックを1つに減らす、あるいは共有をやめる（10-4）

「データ競合が無い」ことと「正しく動く」ことは別だ、というのが Rust の並行性の重要な境界線です。

## Exercise

**[`ex039_arc_mutex`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex039_arc_mutex)** — `cargo test -p ex039_arc_mutex` で判定します。

| 関数 | 仕様 |
| --- | --- |
| `parallel_increment(threads, per_thread)` | `Arc<Mutex<u64>>` を共有し、各スレッドが `per_thread` 回加算した合計を返す |
| `total_length(words)` | `Arc<RwLock<Vec<String>>>` を3スレッドから読み、長さの合計を返す |

## Challenge

`parallel_increment` の `Mutex<u64>` を `std::sync::atomic::AtomicU64` に置き換えてみてください。
ロックを使う場合と比べて、何が単純になり、何ができなくなりますか（複数の値を「まとめて」更新できるか、など）。

## Review

- [ ] `Arc` と `Mutex` がそれぞれ解いている問題を説明できる
- [ ] `MutexGuard` の drop でロックが解放される（RAII）ことを説明できる
- [ ] `lock()` が `Result` を返す理由（poisoning）を説明できる
- [ ] `Mutex` と `RwLock` を選べる
- [ ] デッドロックはコンパイラが防がないことを知っている

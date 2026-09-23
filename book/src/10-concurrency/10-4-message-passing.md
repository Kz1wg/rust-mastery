# Lesson 10-4: メッセージパッシング

## Concept

10-2 では「1つのデータを複数のスレッドで共有し、ロックで守る」設計を見ました。
もう1つの選択肢は、**そもそも共有しない**ことです。データの所有権をチャネル経由で渡せば、
ロックも、デッドロックの心配も要りません。

## Why?

共有可変状態は、正しく書けてもコストがかかります。ロックの範囲、取得順序、poisoning——
どれも設計の負担です。共有しなくて済むなら、それが一番単純です。

## Bad Example: 結果を集めるためだけに共有する

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn sum_shared(data: Vec<u64>) -> u64 {
    let total = Arc::new(Mutex::new(0u64));
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);
    let (left, right) = (left.to_vec(), right.to_vec());

    let mut handles = Vec::new();
    for chunk in [left, right] {
        let total = Arc::clone(&total);
        handles.push(thread::spawn(move || {
            let partial: u64 = chunk.iter().sum();
            *total.lock().unwrap() += partial; // 合計のためだけにロック
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let result = *total.lock().unwrap();
    result
}

fn main() {
    assert_eq!(sum_shared(vec![1, 2, 3, 4]), 10);
}
```

## Problem

各スレッドが計算するのは**自分の担当分の合計**だけで、他のスレッドの途中結果を見る必要はありません。
それなのに、結果を持ち帰るためだけに `Arc<Mutex<u64>>` を導入しています。
ロックは「同時に触られると困るから」使うものですが、ここでは**同時に触る必要がそもそもありません**。

## Think

> **問い**: 各スレッドの結果を、共有せずに集める方法を2つ挙げてください。

<details>
<summary>Hint</summary>

1つはすでに使っています——`thread::spawn` の戻り値です。
もう1つは、値を送るための専用の通り道です。

</details>

<details>
<summary>Solution</summary>

**方法1: `JoinHandle` の戻り値**

```rust
use std::thread;

fn sum_join(data: Vec<u64>) -> u64 {
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);
    let (left, right) = (left.to_vec(), right.to_vec());

    let handles: Vec<_> = [left, right]
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.iter().sum::<u64>()))
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

fn main() {
    assert_eq!(sum_join(vec![1, 2, 3, 4]), 10);
}
```

スレッドのクロージャが返した値は、`join()` で受け取れます。**結果が2つだけなら、これが最も単純です。**

**方法2: チャネル（`std::sync::mpsc`）**

```rust
use std::sync::mpsc;
use std::thread;

fn sum_channel(data: Vec<u64>, workers: usize) -> u64 {
    let (tx, rx) = mpsc::channel();
    let chunk_size = data.len().div_ceil(workers.max(1));

    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let tx = tx.clone();
        thread::spawn(move || {
            let partial: u64 = chunk.iter().sum();
            tx.send(partial).unwrap();
        });
    }
    drop(tx); // 送信側を全て手放すと、rx の繰り返しが終わる

    rx.iter().sum()
}

fn main() {
    assert_eq!(sum_channel(vec![1, 2, 3, 4, 5], 2), 15);
}
```

mpsc は multi-producer, single-consumer（送信側は複数、受信側は1つ）の意味です。
**送る時点で所有権が移る**ので、送った側はもうその値を触れません。これがデータ競合を防ぎます。

`drop(tx)` が重要です。送信側が1つでも残っていると、受信側は「まだ来るかもしれない」と待ち続けます。
ループの中で `clone` した送信側はスレッド終了時に drop されますが、**元の `tx` は残っている**ので、
明示的に手放す必要があります。

</details>

## 共有 vs メッセージ

| | 共有（`Arc<Mutex<T>>`） | メッセージ（チャネル / `join`） |
| --- | --- | --- |
| 向いている場面 | 複数スレッドが**同じ状態を読み書き**する（カウンタ、キャッシュ、接続プール） | 仕事を分けて**結果を集める**、パイプライン処理 |
| 難しさ | ロックの範囲・順序、デッドロック、poisoning | チャネルを閉じ忘れると受信側が止まる |
| 所有権 | 共有したまま | 送信で移動する |

**まずメッセージで書けないかを考えてください。** 書けるなら、考えることが減ります。
「本当に同じ状態を複数スレッドが読み書きする」ときだけ、共有とロックに進みます。

## Deep Dive: チャネルが閉じるとき

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel::<u8>();
    drop(rx); // 受信側がいなくなった
    assert!(tx.send(1).is_err()); // 送信は失敗する（unwrap すると panic）
}
```

受信側が drop されると、`send` は `Err` を返します。`unwrap()` で済ませているコードは、
受信側が先に終了した瞬間に panic します。受信側が途中で終了しうる設計（タイムアウト、エラーで打ち切りなど）では、
`send` の結果を見て静かに終了するか、`Err` を上位へ伝えるかを決めてください。

## Exercise

**`ex041_message_passing`** — `cargo test -p ex041_message_passing` で判定します。

| 関数 | 仕様 |
| --- | --- |
| `sum_with_join(data)` | データを2つに分け、`JoinHandle` の戻り値で合計する |
| `sum_with_channel(data, workers)` | `mpsc` で各ワーカーの部分和を集めて合計する（`drop(tx)` を忘れないこと） |
| `send_to_closed_channel()` | 受信側を drop した後の `send` が `Err` になることを返す |

## Challenge

`sum_with_channel` から `drop(tx)` を消すと、テストはどうなりますか。
（実行が終わらなくなるので、確認したら `Ctrl+C` で止めてください。）
なぜ止まるのかを、「送信側が何本残っているか」で説明してください。

## Review

- [ ] 結果を集めるだけなら、共有よりメッセージや `join` が単純だと説明できる
- [ ] `mpsc` で所有権が移ることが、データ競合を防ぐ理由を説明できる
- [ ] 送信側を全て drop しないと受信側が終わらない理由を説明できる
- [ ] 共有とメッセージを、状況に応じて選べる

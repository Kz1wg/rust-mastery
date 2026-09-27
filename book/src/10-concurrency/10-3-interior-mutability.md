# Lesson 10-3: interior mutability

## Concept

通常、値を変更するには `&mut` が必要です。しかし `Cell` と `RefCell` は、
**`&self` しか持っていなくても中身を変更できます**。これを interior mutability（内部可変性）と言います。

借用の規則が消えるわけではありません。**チェックがコンパイル時から実行時へ移る**だけです。

## Why?

「`&self` しか受け取れないのに、中の状態を変えたい」場面は実際にあります。
キャッシュ、呼び出し回数の記録、グラフ構造の相互参照などです。

## Bad Example

```rust,compile_fail,E0594
struct Counter {
    count: u32,
}

impl Counter {
    // 呼ばれた回数を数えたいが、&self では変更できない
    fn get_and_count(&self) -> u32 {
        self.count += 1;
        self.count
    }
}

fn main() {}
```

## Think

> **問い**: `fn get_and_count(&mut self)` に変えれば解決します。それでも `&self` のままにしたい理由は何でしょうか？

<details>
<summary>Solution</summary>

`&mut self` にすると、**呼び出し側に「排他的な参照」を要求する**ことになります。
複数の場所から同時に（不変参照で）使われている型では、それが通らないことがあります。
また、「読むだけに見えるメソッド」が `&mut` を要求すると、呼び出し側の設計が窮屈になります。

`Cell` を使うと、`&self` のまま中身を差し替えられます。

```rust
use std::cell::Cell;

struct Counter {
    count: Cell<u32>,
}

impl Counter {
    fn get_and_count(&self) -> u32 {
        self.count.set(self.count.get() + 1);
        self.count.get()
    }
}

fn main() {
    let c = Counter { count: Cell::new(0) };
    assert_eq!(c.get_and_count(), 1);
    assert_eq!(c.get_and_count(), 2);
}
```

</details>

## `Cell` と `RefCell` の違い

| | `Cell<T>` | `RefCell<T>` |
| --- | --- | --- |
| 使い方 | 値を**まるごと出し入れ**（`get` / `set` / `replace`） | 参照を借りる（`borrow` / `borrow_mut`） |
| 中身への参照 | 取れない | 取れる |
| `T` の条件 | `get` は `T: Copy` が必要 | 無し |
| 失敗の仕方 | 失敗しない | 借用規則を破ると**実行時にpanic** |
| コスト | ほぼゼロ | 借用フラグの読み書き |

**`Cell` で済むなら `Cell`** です。`Cell` は失敗しようがないので、panic の心配がありません。
中身が `Vec` や `String` のように大きく、「その場で読み書きしたい」場合に `RefCell` を使います。

## `RefCell` の代償: 実行時panic

```rust
use std::cell::RefCell;

struct Logger {
    entries: RefCell<Vec<String>>,
}

impl Logger {
    fn log(&self, message: &str) {
        self.entries.borrow_mut().push(message.to_string());
    }

    fn count(&self) -> usize {
        self.entries.borrow().len()
    }
}

fn main() {
    let logger = Logger { entries: RefCell::new(Vec::new()) };
    logger.log("first");
    logger.log("second");
    assert_eq!(logger.count(), 2);
}
```

このコードは安全ですが、次のように書くと**実行時にpanic**します。

```rust,should_panic
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(vec![1]);
    let _a = cell.borrow_mut();
    let _b = cell.borrow_mut(); // panic: already borrowed: BorrowMutError
}
```

コンパイル時には何も言われません。**`RefCell` は、借用エラーをコンパイルエラーから実行時panicへ
引っ越しさせる道具**です。テストされていない経路で初めて落ちる可能性があるので、
「借用チェックを避けられる便利な道具」ではなく、**「実行時チェックの代償を払う選択」**として使ってください。

> `borrow_mut()` の代わりに `try_borrow_mut()` を使うと、panic ではなく `Result` を受け取れます。
> panic させたくない場面（ライブラリの内部など）では、こちらを検討してください。

## Deep Dive: スレッドとの関係

10-1 の表のとおり、`Cell` と `RefCell` は `Sync` では**ありません**。借用フラグやデータの更新が
アトミックではないからです。複数スレッドで共有したいなら、対応するのは次の型です。

| 単一スレッド | 複数スレッド |
| --- | --- |
| `Cell<T>` | `Mutex<T>` / `AtomicU32` など |
| `RefCell<T>` | `Mutex<T>` / `RwLock<T>` |
| `Rc<T>` | `Arc<T>` |

`Rc<RefCell<T>>` が単一スレッドの定型句、`Arc<Mutex<T>>` が複数スレッドの定型句、と対応しています。

## Exercise

**[`ex040_interior_mutability`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex040_interior_mutability)** — `cargo test -p ex040_interior_mutability` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `Counter`（`Cell<u32>`） | `&self` のまま数えられる `increment` / `get` |
| `Logger`（`RefCell<Vec<String>>`） | `log(&self, &str)` / `count(&self)` / `entries(&self) -> Vec<String>`（クローンを返す） |
| `try_log` | `try_borrow_mut` を使い、借用できないときは panic せず `false` を返す |

`entries` が `Vec<String>` のクローンを返す設計にしている理由を、`RefCell` の制約から説明してください。

## Challenge

`Logger::log` の中で `self.count()` を呼ぶコードを書いてみてください。panic しますか、しませんか。
`borrow()` と `borrow_mut()` が同時に存在するかどうかで考えてください。

## Review

- [ ] interior mutability が借用チェックを実行時へ移す仕組みだと説明できる
- [ ] `Cell` と `RefCell` を使い分けられる
- [ ] `RefCell` の代償（実行時panic）を理解し、`try_borrow_mut` という選択肢を知っている
- [ ] 単一スレッド用の型と、複数スレッド用の型の対応を説明できる

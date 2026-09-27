# Lesson 03-3: typestate pattern

## Concept

03-2 の `enum` 版では、無効な遷移は**実行時に** `Err` になりました。
遷移の順序がコードの中で決まっているなら、**コンパイル時に**検出できないでしょうか。

## Why?

「`start` の前に `finish` を呼ぶ」ようなミスは、テストを書かなければ実行して初めて分かります。
型に状態を持たせれば、そのミスを**書けなくする**ことができます。

## Bad Example

03-2 の `JobState` の使い方です。

```rust
#[derive(Debug, PartialEq)]
enum JobState {
    Pending,
    Running { started_at: u64 },
    Succeeded { started_at: u64, finished_at: u64 },
}

impl JobState {
    fn succeed(&mut self, now: u64) -> Result<(), String> {
        match self {
            JobState::Running { started_at } => {
                let started_at = *started_at;
                *self = JobState::Succeeded { started_at, finished_at: now };
                Ok(())
            }
            _ => Err("not running".to_string()),
        }
    }
}

fn main() {
    let mut s = JobState::Pending;
    // start を呼び忘れた。コンパイルは通り、実行時に Err になる
    assert!(s.succeed(5).is_err());
}
```

## Problem

このミスは、**プログラムを書いた時点で確定しているミス**です（外部入力に依存していない）。
それなのに、発見は実行時（またはテスト）まで遅れます。

## Think

> **問い**:
> 1. 「`Pending` のジョブには `finish` メソッドが**存在しない**」ようにするには、どう書きますか？
> 2. 状態を型パラメータにすると、どんな問題が新しく生じますか？

<details>
<summary>Hint</summary>

`Job<S>` のように、状態を**型パラメータ**にし、`impl Job<Running>` のように状態ごとに `impl` を分けます。
遷移メソッドが `self` を消費して**別の型**を返すと、どうなるでしょうか。

</details>

<details>
<summary>Solution</summary>

```rust
struct Pending;
struct Running {
    started_at: u64,
}
struct Done {
    started_at: u64,
    finished_at: u64,
}

struct Job<S> {
    id: u32,
    state: S,
}

impl Job<Pending> {
    fn new(id: u32) -> Self {
        Job { id, state: Pending }
    }

    fn start(self, now: u64) -> Job<Running> {
        Job { id: self.id, state: Running { started_at: now } }
    }
}

impl Job<Running> {
    fn finish(self, now: u64) -> Job<Done> {
        Job {
            id: self.id,
            state: Done { started_at: self.state.started_at, finished_at: now },
        }
    }
}

impl Job<Done> {
    fn duration(&self) -> u64 {
        self.state.finished_at - self.state.started_at
    }
}

fn main() {
    let job = Job::new(1).start(10).finish(15);
    assert_eq!(job.duration(), 5);
    assert_eq!(job.id, 1);
}
```

順序を間違えると、コンパイルエラーになります。

```rust,compile_fail,E0599
struct Pending;
struct Running;
struct Job<S> { state: S }

impl Job<Pending> {
    fn new() -> Self { Job { state: Pending } }
}
impl Job<Running> {
    fn finish(self) {}
}

fn main() {
    Job::<Pending>::new().finish(); // Pending には finish が存在しない
}
```

遷移メソッドは `self` を**消費**するので、遷移後に古い状態を使うこともできません。

```rust,compile_fail,E0382
struct Pending;
struct Running;
struct Job<S> { state: S }

impl Job<Pending> {
    fn new() -> Self { Job { state: Pending } }
    fn start(self) -> Job<Running> { Job { state: Running } }
}

fn main() {
    let job = Job::new();
    let _running = job.start();
    let _again = job.start(); // 古い job はもう使えない
}
```

**「状態を持つデータ」が、その状態のときにだけ存在する**点も利点です。
`Done` だけが `finished_at` を持ち、`Running` からは `duration()` を呼べません。

</details>

## トレードオフ: enum 版 vs typestate 版

| 観点 | `enum` 版（03-2） | typestate 版 |
| --- | --- | --- |
| 無効な遷移の検出 | 実行時（`Result`） | **コンパイル時** |
| 状態が実行時に決まる場合（DB・ネットワーク由来） | 自然に扱える | 扱いにくい（下記） |
| 異なる状態のジョブを1つの `Vec` に入れる | 簡単 | できない（型が別） |
| 型の数・コードの量 | 少ない | 多い |
| エラーメッセージ | 読みやすい | ジェネリクスが絡むと読みにくい |

### 異なる状態を1つの `Vec` に入れられない

```rust,compile_fail,E0308
struct Pending;
struct Running;
struct Job<S> { state: S }

fn main() {
    let mut jobs: Vec<Job<Pending>> = Vec::new();
    jobs.push(Job { state: Pending });
    jobs.push(Job { state: Running }); // Job<Running> は Job<Pending> ではない
}
```

複数の状態を混在させたい場合は、結局 `enum` で包む必要があります。

```rust
struct Pending;
struct Running;
struct Job<S> { state: S }

enum AnyJob {
    Pending(Job<Pending>),
    Running(Job<Running>),
}

fn main() {
    let jobs = vec![
        AnyJob::Pending(Job { state: Pending }),
        AnyJob::Running(Job { state: Running }),
    ];
    assert_eq!(jobs.len(), 2);
}
```

つまり、**状態が実行時に決まる（データベースから読んだ、ネットワークから届いた）場合、typestate は使えません**。
`enum` を併用するなら、typestate の利点は境界の内側に限られます。

## どちらを選ぶか

| 状況 | 選ぶ |
| --- | --- |
| 遷移の順序が**コード上で固定**（ビルダー、プロトコルのハンドシェイク、リソースの開閉） | typestate |
| 状態が**データとして**動的に決まる（永続化、外部イベント、複数インスタンスの管理） | `enum` |
| 状態数・遷移が多く、頻繁に変わる | `enum`（typestate は変更のコストが大きい） |
| 誤用の被害が大きく、コンパイル時に必ず防ぎたい | typestate |

**typestate は「より高度な設計」ではありません。** 適した場面が限られた道具です。

## Deep Dive: 状態がデータを持たないとき

`Pending` のような**データを持たない状態**を型パラメータに使うと、`struct Job<S> { id: u32 }` のように `S` を使わない定義になり、
コンパイラは「型パラメータ `S` が使われていない」と怒ります。
このときに使うのが `PhantomData` です（Chapter 09-1）。
今回は状態がデータを持つ形にして、この問題を避けました。

## Exercise

**[`ex011_typestate`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex011_typestate)** — `cargo test -p ex011_typestate` で判定します。

`RequestBuilder` を typestate で実装します。

| 型 | 仕様 |
| --- | --- |
| `RequestBuilder<NoUrl>` | `new()` と `url(self, &str) -> RequestBuilder<HasUrl>` |
| `RequestBuilder<HasUrl>` | `header(self, k, v) -> Self` と `build(self) -> Request` |

URL は `Option<String>` ではなく、**`HasUrl` 状態の中に持たせて**あります。
そのため `build` の中で `unwrap` が要りません——「URLがある」ことを型で表した結果です。
`url` を設定しないと `build` を呼べないことは、`compile_fail` doctest で確認しています。

任意で `NOTES.md` に、**この課題で typestate を使うのが適切と言える理由**を書いてください。
もしこれが「設定ファイルから読み込んだ値でリクエストを組み立てる」課題だったら、設計はどう変わりますか？

## Challenge

03-2 のドア（`Open` / `Closed` / `Locked`）を typestate で書き直し、`enum` 版と比べてください。
「ドアの状態が外部センサーから届く」という要件が加わったら、どうなりますか。

## Review

- [ ] typestate が遷移の誤りをコンパイル時に検出する仕組みを説明できる
- [ ] typestate が使えない（使いにくい）場面を説明できる
- [ ] `enum` 版と typestate 版を、状況に応じて選べる

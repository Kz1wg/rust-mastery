# Lesson 03-1: enumで状態を表す

## Concept

「ジョブが待機中・実行中・成功・失敗のいずれか」を、`struct` と `Option` で表すと何が起きるでしょうか。

## Why?

02-1 では `bool` が状態の組み合わせを増やすことを見ました。
`Option` フィールドも同じです。**「あるかもしれない」値が複数あると、組み合わせが増えます。**

## Bad Example

```rust
struct Job {
    started_at: Option<u64>,
    finished_at: Option<u64>,
    error: Option<String>,
}

fn main() {
    // これは何の状態？
    let job = Job {
        started_at: None,
        finished_at: Some(100),
        error: Some("boom".to_string()),
    };
    let _ = job;
}
```

## Problem

3つの `Option` で 2³ = 8 通りの状態を表せますが、有効なのは4つ（待機・実行中・成功・失敗）だけです。

| started_at | finished_at | error | 意味 |
| --- | --- | --- | --- |
| None | None | None | 待機中 |
| Some | None | None | 実行中 |
| Some | Some | None | 成功 |
| Some | Some | Some | 失敗 |
| **None** | **Some** | **Some** | **開始していないのに終了して失敗？** |

さらに、状態を判定するコードは「どのフィールドが `Some` か」を見て推測するしかなく、
`if job.error.is_some() { ... }` のような判定が**あちこちに散らばります**。

## Think

> **問い**:
> 1. 各状態が持つべきデータは何ですか？
> 2. 無効な状態を**書けなくする**設計を書いてください。
> 3. 「開始時刻」のように**複数の状態で共通するデータ**は、どう取り出しますか？

<details>
<summary>Hint</summary>

Rustの `enum` のバリアントは、それぞれ**別のフィールド**を持てます。

</details>

<details>
<summary>Solution</summary>

```rust
#[derive(Debug, PartialEq)]
enum JobState {
    Pending,
    Running { started_at: u64 },
    Succeeded { started_at: u64, finished_at: u64 },
    Failed { started_at: u64, finished_at: u64, error: String },
}

impl JobState {
    /// 共通データの取り出しは、メソッドに閉じ込める
    fn started_at(&self) -> Option<u64> {
        match self {
            JobState::Pending => None,
            JobState::Running { started_at }
            | JobState::Succeeded { started_at, .. }
            | JobState::Failed { started_at, .. } => Some(*started_at),
        }
    }

    fn is_finished(&self) -> bool {
        matches!(self, JobState::Succeeded { .. } | JobState::Failed { .. })
    }
}

fn main() {
    let s = JobState::Failed {
        started_at: 1,
        finished_at: 5,
        error: "boom".to_string(),
    };
    assert_eq!(s.started_at(), Some(1));
    assert!(s.is_finished());
    assert_eq!(JobState::Pending.started_at(), None);
}
```

**得られるもの**:

- 「開始していないのに完了時刻がある」状態は、**書けません**
- `match` は網羅性が検査されるので、状態を増やしたとき、対応が必要な箇所がコンパイルエラーで分かる
- 状態ごとに**必要なデータだけ**を持つ

**失うもの（トレードオフ）**:

| コスト | 内容 |
| --- | --- |
| 共通フィールドの取り出し | 上の `started_at()` のように、`match` を書く必要がある |
| 一部のフィールドだけ更新 | `enum` のバリアント内のフィールドを直接書き換えにくい（状態を作り直す） |
| 状態が多いと冗長 | 共通フィールドが多い場合、`struct` にまとめて共通部分を外に出す方法もある |

**共通部分が大きいなら、外側の `struct` に出す**という選択肢もあります。

```rust
enum Status {
    Pending,
    Running { started_at: u64 },
    Done { started_at: u64, finished_at: u64 },
}

struct Job {
    id: u32,          // 全状態で共通
    name: String,     // 全状態で共通
    status: Status,   // 状態ごとに違うデータだけを enum に
}

fn main() {
    let j = Job { id: 1, name: "build".to_string(), status: Status::Pending };
    let _ = j;
}
```

</details>

## Deep Dive: `Option` は「enum」である

`Option<T>` は標準ライブラリの `enum` です。

```rust
enum MyOption<T> {
    None,
    Some(T),
}

fn main() {
    let x: MyOption<i32> = MyOption::Some(1);
    let _ = x;
}
```

`Option` を使うのが**適切な場面**は、「値があるかないか」が**それ1つで独立した情報**のときです（例: 入力の任意のフィールド）。
複数の `Option` の**組み合わせに意味がある**場合は、その組み合わせを表す `enum` を作るサインです。

## Exercise

**[`ex009_enum_states`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex009_enum_states)** — `cargo test -p ex009_enum_states` で判定します。
（`cargo test` で自動判定するため、型定義や公開APIのシグネチャはあらかじめ用意してあります。本体の `todo!()` を実装してください。）

| 課題 | 仕様 |
| --- | --- |
| `Connection` | `enum Connection { Disconnected, Connected, Failed }`（各状態が `retry_count` を持ち、`Failed` は `error` も持つ）は用意済み |
| メソッド | `connect` / `fail`（`retry_count` を1増やす）/ `is_connected` / `retry_count` を実装 |

任意で `NOTES.md` に、元の `struct { socket: Option<Socket>, error: Option<String>, retry_count: u32 }` が
**表現できてしまっていた無効な状態**を2つ以上書いてください。

## Challenge

`enum` にしたことで、共通フィールドへのアクセスが面倒になりました。
上の `Job { id, name, status }` の形（外側に共通、内側に状態別）と、`JobState` 単体の形は、どういう基準で選びますか？

## Review

- [ ] 複数の `Option` が無効な状態を生むことを説明できる
- [ ] 状態ごとにデータを持つ `enum` を設計できる
- [ ] 共通データの取り出しに関するトレードオフを説明できる

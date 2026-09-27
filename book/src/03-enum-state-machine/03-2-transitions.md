# Lesson 03-2: 状態遷移をenumで設計する

## Concept

状態を `enum` で表しても、**状態を変える操作**が自由に書けるなら、不正な遷移（成功したジョブを待機中に戻す、など）は防げません。
「どの状態からどの状態へ移れるか」を、コードに書きます。

## Why?

「どんな状態があるか」と「どの状態からどの状態へ移れるか（遷移）」をまとめたものを、**状態機械**（state machine）と呼びます。
信号機（青 → 黄 → 赤 → 青）や、注文の流れ（注文済み → 支払い済み → 発送済み）がその例です。

状態機械の価値は、状態の一覧ではなく**遷移の規則**にあります。
規則がコードのあちこちに散らばると、規則の全体像を誰も把握できなくなります。

## 遷移図

```text
            start                 succeed
 Pending ──────────► Running ──────────────► Succeeded
                        │
                        │ fail
                        ▼
                     Failed
```

これ以外の遷移（例: `Pending` に対する `succeed`、`Succeeded` に対する `start`）は**無効**です。

## Bad Example

```rust
#[derive(Debug)]
enum JobState {
    Pending,
    Running { started_at: u64 },
    Succeeded { started_at: u64, finished_at: u64 },
}

impl JobState {
    fn succeed(&mut self, now: u64) {
        match self {
            JobState::Running { started_at } => {
                let started_at = *started_at;
                *self = JobState::Succeeded { started_at, finished_at: now };
            }
            _ => panic!("invalid transition"),
        }
    }
}

fn main() {
    let mut s = JobState::Running { started_at: 1 };
    s.succeed(5);
    println!("{s:?}");
}
```

## Problem

- 無効な遷移は `panic!` になる。呼び出し側は**回復できない**（`Result` ではないので）。
- 「この操作が失敗しうる」ことが、シグネチャから**読み取れない**。
- 外部入力（リトライ・重複したメッセージ）で無効な遷移が起きうる状況では、panic はバグではなく**通常の事象**を異常終了にしてしまう。

## Think

> **問い**: 無効な遷移の扱い方には、次のような選択肢があります。それぞれの利点と欠点を考えてください。

| 案 | シグネチャ | 無効な遷移のとき |
| --- | --- | --- |
| (A) | `fn succeed(&mut self, now) -> Result<(), TransitionError>` | `Err` を返し、状態は**変わらない** |
| (B) | `fn succeed(self, now) -> Result<JobState, TransitionError>` | `Err` を返す。元の状態は**消費されて失われる** |
| (C) | `fn succeed(self, now) -> JobState` | 状態を変えずに**黙って**返す |

<details>
<summary>Hint</summary>

(B) で `Err` を受け取った呼び出し側は、元の状態をまだ持っているでしょうか。
(C) では、呼び出し側は遷移が成功したことをどう知りますか。

</details>

<details>
<summary>Solution</summary>

| 案 | 利点 | 欠点 |
| --- | --- | --- |
| (A) `&mut self` → `Result<(), _>` | エラー時も状態が保たれる。呼び出し側が回復できる | `&mut` が必要（共有できない） |
| (B) `self` → `Result<Self, _>` | 遷移の前後を別の値として扱える。関数型的 | エラー時に元の状態が失われる。`Err` に元の状態を含めないと回復できない |
| (C) `self` → `Self` | 呼び出しが簡単 | **無効な遷移が黙って無視される。バグを隠す** |

一般的には **(A)** が扱いやすく、まず選ぶ案です。
(B) を選ぶなら、エラーに元の状態を含めます（`Err((self, error))` など）。
(C) は「無効なイベントは無視してよい」という**仕様**が明確な場合（UIのイベント処理など）に限ります。

```rust
#[derive(Debug, PartialEq)]
enum JobState {
    Pending,
    Running { started_at: u64 },
    Succeeded { started_at: u64, finished_at: u64 },
    Failed { started_at: u64, finished_at: u64, error: String },
}

#[derive(Debug, PartialEq)]
struct TransitionError {
    from: &'static str,
    action: &'static str,
}

impl JobState {
    fn name(&self) -> &'static str {
        match self {
            JobState::Pending => "Pending",
            JobState::Running { .. } => "Running",
            JobState::Succeeded { .. } => "Succeeded",
            JobState::Failed { .. } => "Failed",
        }
    }

    fn invalid(&self, action: &'static str) -> TransitionError {
        TransitionError { from: self.name(), action }
    }

    fn start(&mut self, now: u64) -> Result<(), TransitionError> {
        match self {
            JobState::Pending => {
                *self = JobState::Running { started_at: now };
                Ok(())
            }
            _ => Err(self.invalid("start")),
        }
    }

    fn succeed(&mut self, now: u64) -> Result<(), TransitionError> {
        match self {
            JobState::Running { started_at } => {
                let started_at = *started_at;
                *self = JobState::Succeeded { started_at, finished_at: now };
                Ok(())
            }
            _ => Err(self.invalid("succeed")),
        }
    }

    fn fail(&mut self, now: u64, error: String) -> Result<(), TransitionError> {
        match self {
            JobState::Running { started_at } => {
                let started_at = *started_at;
                *self = JobState::Failed { started_at, finished_at: now, error };
                Ok(())
            }
            _ => Err(self.invalid("fail")),
        }
    }
}

fn main() {
    let mut s = JobState::Pending;

    // 無効な遷移は Err になり、状態は変わらない
    assert_eq!(
        s.succeed(5),
        Err(TransitionError { from: "Pending", action: "succeed" })
    );
    assert_eq!(s, JobState::Pending);

    // 有効な遷移
    s.start(1).unwrap();
    s.succeed(5).unwrap();
    assert_eq!(s, JobState::Succeeded { started_at: 1, finished_at: 5 });

    // 終了状態からは、どこへも遷移できない
    assert!(s.start(6).is_err());
    assert!(s.fail(6, "x".to_string()).is_err());
}
```

**遷移の規則が、`impl JobState` の中に集約されている**ことに注目してください。
状態の外側から `*state = JobState::Pending` のように直接代入できると、この規則が守れません。
実際のアプリでは、`JobState` を private フィールドとして持つ型に包み（Lesson 02-4）、
**遷移メソッド経由でしか変更できない**ようにします。

</details>

## Deep Dive: エラー型に何を入れるか

上の `TransitionError` は `&'static str` で状態名を持っています。
この設計は簡単ですが、次の点でトレードオフがあります。

| 設計 | 利点 | 欠点 |
| --- | --- | --- |
| `&'static str`（今回） | 簡単。ヒープ確保なし | 文字列なので、呼び出し側が状態で分岐しにくい |
| 状態を表す別の `enum`（`StateKind`）を持つ | 呼び出し側が `match` で分岐できる | 型が1つ増える |
| `from: JobState` をそのまま持つ | 情報が最大 | `Clone` が必要、または所有権の問題が生じる |

エラー型の設計は Chapter 06 で詳しく扱います。

## Exercise

**[`ex010_state_machine`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex010_state_machine)** — `cargo test -p ex010_state_machine` で判定します。

ドアの状態機械を実装します。

```text
Open ──close──► Closed ──lock(code)──► Locked
  ▲               │  ▲                   │
  └─────open──────┘  └────unlock(code)───┘
```

| メソッド | 仕様 |
| --- | --- |
| `fn close(&mut self) -> Result<(), DoorError>` | `Open` のときだけ成功 |
| `fn open(&mut self) -> Result<(), DoorError>` | `Closed` のときだけ成功 |
| `fn lock(&mut self, code: u32) -> Result<(), DoorError>` | `Closed` のときだけ成功。コードを保存 |
| `fn unlock(&mut self, code: u32) -> Result<(), DoorError>` | `Locked` かつコードが一致するときだけ成功 |

テストは、**全ての（状態 × 操作）の組み合わせ**で、有効／無効を検証します。
`Locked` はコードを持つので、`enum` のバリアントにデータを持たせてください。

## Challenge

(B) の `self -> Result<Self, _>` を、エラー時に元の状態を返す形（`Err((Self, Error))`）で実装してみてください。
(A) と比べて、呼び出しコードはどう変わりますか。

## Review

- [ ] 状態機械では「遷移の規則」がコードの1箇所に集約されるべき理由を説明できる
- [ ] 無効な遷移の扱い（panic / Err / 無視）を、状況に応じて選べる
- [ ] 状態フィールドの公開範囲が、遷移の保証に関わることを説明できる

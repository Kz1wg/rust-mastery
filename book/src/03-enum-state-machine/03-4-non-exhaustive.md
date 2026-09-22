# Lesson 03-4: `#[non_exhaustive]` と将来の拡張

## Concept

公開している `enum` に、あとからバリアントを追加すると、**利用者のコードは壊れる**でしょうか。

## Why?

`enum` の網羅性検査は、自分のコードの中では強力な味方です（03-1）。
しかし、**ライブラリの利用者**にとっては、新しいバリアントの追加が**コンパイルエラー**を引き起こします。
これはバージョン管理（semver）の問題です。

## Bad Example

ライブラリが次のように公開しているとします。

```rust,ignore
// crate: mylib
pub enum Error {
    NotFound,
    PermissionDenied,
}
```

利用者は、網羅的に `match` します。

```rust,ignore
// 利用者のcrate
fn message(e: mylib::Error) -> &'static str {
    match e {
        mylib::Error::NotFound => "not found",
        mylib::Error::PermissionDenied => "denied",
    }
}
```

> 上のコードは、**別々の crate**（ライブラリと利用者）が必要なため `ignore` にしています。
> `mdbook test` では検証できませんが、`rustc` で2つのcrateを手元でビルドして、この節の挙動を確認しました。

## Problem

ライブラリが `Error::Timeout` を**追加すると、利用者の `match` がコンパイルエラーになります**。
「バリアントの追加」は、`enum` が `pub` である限り、**破壊的変更**です。

## Think

> **問い**:
> 1. ライブラリ側が、将来バリアントを追加できる自由を確保するには、どうしますか？
> 2. その代わり、利用者側は何を失いますか？
> 3. すべての公開 `enum` に付けるべきでしょうか？

<details>
<summary>Hint</summary>

属性 `#[non_exhaustive]` を `enum` に付けます。**利用者側の `match`** はどう変わるでしょうか。

</details>

<details>
<summary>Solution</summary>

```rust,ignore
// crate: mylib
#[non_exhaustive]
pub enum Error {
    NotFound,
    PermissionDenied,
}
```

こうすると、利用者は他の crate から `match` するとき、**必ずワイルドカードの腕（`_ =>`）を書く必要**があります。
書かなければ、次のエラーになります。

```text
error[E0004]: non-exhaustive patterns: `_` not covered
```

```rust,ignore
// 利用者のcrate
fn message(e: mylib::Error) -> &'static str {
    match e {
        mylib::Error::NotFound => "not found",
        mylib::Error::PermissionDenied => "denied",
        _ => "unknown error", // 必須
    }
}
```

これで、ライブラリは `Timeout` を**破壊的変更なしに**追加できます。

**代償**:

| 失うもの | 内容 |
| --- | --- |
| 網羅性検査（利用者側） | 新しいバリアントが追加されたことを、コンパイラが利用者に教えてくれなくなる |
| ライブラリ側の自由度 | なし（むしろ得る） |

つまり、`#[non_exhaustive]` は**「将来の拡張の自由」と「利用者の網羅性検査」の交換**です。

**同じ crate の中では、`#[non_exhaustive]` は効果がありません**。網羅性の検査は通常どおり行われます。
影響を受けるのは、**別の crate の利用者**だけです。

</details>

## 使い分けの基準

| `enum` の性質 | 選ぶ | 例 |
| --- | --- | --- |
| **意味が固定された閉じた集合**。利用者に網羅的な処理を**してほしい** | 付けない | `Option`, `Ordering`（`Less` / `Equal` / `Greater`） |
| **将来増えうる集合**（エラーの種類、イベント、設定項目） | 付ける | ライブラリの `Error` 型 |
| アプリケーション内部（別crateの利用者がいない） | 付けない（効果がない） | 03-1 の `JobState` |
| 状態機械で、状態の追加時に**全ての処理を見直したい** | 付けない | 自分のcrate内の状態機械 |

`Ordering` に `Less` / `Equal` / `Greater` 以外が増えることは考えられないため、非網羅的にする意味がありません。
一方、ライブラリの `Error` は、機能追加のたびにバリアントが増えるのが自然です。

## `struct` と `variant` にも付けられる

```rust,ignore
// crate: mylib
#[non_exhaustive]
pub struct Options {
    pub level: u8,
}
```

利用者は `Options { level: 1 }` と直接構築できなくなります（`E0639`）。
利用者が構築できるよう、**構築用の関数**（`Options::new()` やビルダー）を提供します。
フィールドを後から追加できる自由が得られる一方、Lesson 02-4 で見たように、
「構築経路を限定する」設計とも整合します。

## Deep Dive: ワイルドカードの落とし穴

`_ =>` の腕は、書かれる状況によって意味が異なります。

- `non_exhaustive` な外部の `enum` に対する `_ =>` は**必須**で、「未知のバリアントをどう扱うか」を決める場所になる
- 自分の `enum` に対する `_ =>` は**選択**で、新しいバリアントを黙って飲み込む危険がある

`_ =>` を書くときは、「新しいバリアントが増えたとき、この腕で本当に良いか」を毎回考えてください。

## Exercise

**`ex012_non_exhaustive`** — `cd exercises/ex012_non_exhaustive && cargo test` で判定します
（`lib` と `app` の2crateからなる独立した workspace なので、`-p` ではなくディレクトリ内で実行します）。

| crate | 内容 |
| --- | --- |
| `lib`（`ex012_lib`） | `#[non_exhaustive]` 付きの `enum Event { Click, Key, Scroll }` は用意済み。`Scroll` は「`app` を書いた後から追加されたバリアント」という想定 |
| `app`（`ex012_app`） | `handle(Event) -> String` を実装する。`Click` と `Key` は説明文を、それ以外（`_ =>`）は汎用メッセージを返す |

実装できたら、`app` の `_ =>` の腕を一度消してみてください。別crateからの `match` なので、
`error[E0004]` になることを確認できます。

## Challenge

あなたが今持っているコードの公開 `enum` を1つ選び、`#[non_exhaustive]` を**付けるべきか**を、上の表に沿って判断してください。
「付けない」という判断にも、理由が必要です。

## Review

- [ ] `enum` へのバリアント追加が、なぜ破壊的変更になりうるか説明できる
- [ ] `#[non_exhaustive]` が利用者側に何を要求し、何を得るか説明できる
- [ ] 同じ crate 内では `#[non_exhaustive]` が効果を持たないことを知っている
- [ ] 公開 `enum` に付ける／付けないの判断基準を説明できる

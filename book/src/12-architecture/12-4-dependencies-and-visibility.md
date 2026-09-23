# Lesson 12-4: 依存の方向と公開範囲

## Concept

`pub` は「この項目を外に見せる」という宣言であると同時に、**利用者との約束**です。
一度公開したものを消したり変えたりすることは、破壊的変更になります（Lesson 03-4、Chapter 16）。

## Why?

「とりあえず `pub`」で書いたコードは、後から範囲を狭めるのが難しくなります。
最初から範囲を絞っておけば、内部の作り替えは自由です。

## 4段階の公開範囲

```rust
mod outer {
    pub mod inner {
        pub fn everyone() {}           // crate の外からも見える
        pub(crate) fn same_crate() {}  // この crate の中だけ
        pub(super) fn parent_only() {} // outer からだけ
        fn private() {}                // inner の中だけ

        pub fn use_all() {
            everyone();
            same_crate();
            parent_only();
            private();
        }
    }

    pub fn from_outer() {
        inner::parent_only(); // pub(super) なので見える
    }
}

fn main() {
    outer::inner::use_all();
    outer::from_outer();
    outer::inner::same_crate(); // 同じ crate なので見える
}
```

**既定は非公開**です。`pub` は、書いたときにだけ範囲が広がります。

| 記法 | 使いどころ |
| --- | --- |
| （なし） | 既定。まずここから始める |
| `pub(super)` | 親モジュールとだけ共有したい実装 |
| `pub(crate)` | crate 内の共通処理。**外部には出したくないが、複数モジュールで使う** |
| `pub` | 利用者に使ってほしいAPI。**semver の約束がかかる** |

## Bad Example: 内部構造がそのまま公開APIになる

```rust,compile_fail,E0603
mod store {
    mod internal {
        pub struct Db;
    }

    pub fn open() -> internal::Db {
        internal::Db
    }
}

fn main() {
    let _ = store::internal::Db; // internal は非公開モジュール
}
```

このコードは、`open()` が**非公開モジュールの型**を返しているので、
利用者は戻り値の型を名前で書けません（変数に束縛して使うことはできます）。
公開APIに出てくる型は、利用者が名前を書ける場所に置く必要があります。

## Think

> **問い**: 内部のモジュール構成（`store::internal::Db` のような深い階層）を利用者に見せずに、
> `Db` という型だけを公開するにはどうしますか？

<details>
<summary>Solution</summary>

**`pub use` による再エクスポート（facade パターン）**です。

```rust
mod store {
    mod internal {
        pub struct Db {
            pub(super) name: String,
        }

        impl Db {
            pub fn name(&self) -> &str {
                &self.name
            }
        }
    }

    // 内部の階層は隠したまま、型だけを store の直下に見せる
    pub use internal::Db;

    pub fn open(name: &str) -> Db {
        Db { name: name.to_string() }
    }
}

fn main() {
    let db: store::Db = store::open("main"); // 型名を書ける
    assert_eq!(db.name(), "main");
}
```

利用者から見える名前は `store::Db` と `store::open` だけです。
`internal` というモジュール名は公開APIに出てこないので、**後から内部構成を変えても利用者は壊れません**。

これは実際のライブラリでよく使われる形です。crate のルート（`lib.rs`）に

```rust,ignore
mod config;
mod parser;
mod error;

pub use config::Config;
pub use error::{Error, Result};
pub use parser::parse;
```

と書けば、利用者は `mycrate::Config` のようにフラットな名前で使えます。

</details>

## 依存の方向を保つ

モジュール間でも crate 間でも、判断は同じです。

```text
良い:  cli → service → domain        （一方向）
悪い:  cli → service ⇄ domain        （相互参照）
```

`domain`（中心のロジック）が外側を知らない状態を保てば、

- `domain` を単体でテストできる
- 外側（DB、HTTP、CLI）を差し替えても `domain` は変わらない
- 変更の影響が外側へしか波及しない

相互参照が必要に見えたときは、たいてい**共通の型をどちらに置くかを決めていない**ことが原因です。
共通の型を内側（`domain`）に置き、外側がそれを使う形にすると一方向に戻せます。

## Deep Dive: `pub` を付ける前の問い

新しく `pub` を書くときは、次を自問してください。

1. **利用者はこれを使う必要があるか**（今、具体的に）
2. この型・関数の**シグネチャを将来変えないと約束できるか**
3. 公開する型は、**内部の実装詳細を漏らしていないか**（`pub` フィールド、内部の enum、依存crateの型）

3つ目が見落とされがちです。例えば公開関数が `serde_json::Value` を返すと、
**serde_json のバージョンが利用者の依存に固定されます**。自分の型で包むか、
その依存を公開APIから外す（feature にする）ことを検討してください。

## Exercise

**`ex049_visibility_and_facade`** — `cargo test -p ex049_visibility_and_facade` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `pub use` による facade | 内部モジュール構成を隠し、crate のルートに型と関数を再エクスポートする |
| `pub(crate)` のヘルパー | crate 内の複数モジュールから使うが、外部には見せない |

`compile_fail` doctest で、内部モジュールと `pub(crate)` の項目が外部 crate から見えないことを確認します。

## Challenge

自分のコードの `lib.rs` を開き、`pub` が付いている項目を数えてください。
そのうち、実際に外部から使われているものはいくつありますか。

## Review

- [ ] 4段階の公開範囲を使い分けられる
- [ ] `pub use` で内部構成を隠す facade パターンを説明できる
- [ ] 依存の方向を一方向に保つ利点を説明できる
- [ ] 公開APIに依存crateの型を出すことの危険を説明できる

# Lesson 12-3: workspaceの設計

## Concept

workspace は、複数の crate を1つのリポジトリでまとめて扱う仕組みです。
`target/` とロックファイルを共有するので、ビルドが速くなり、バージョンの食い違いも防げます。

問題は「どこで crate に分けるか」です。**モジュールで足りるなら、crate に分ける必要はありません。**

## Why?

crate を分けると、`Cargo.toml` が増え、`pub` の管理が増え、循環依存が禁止されます。
その代わりに得られるものがあるときだけ、分けるべきです。

## crate に分ける理由

| 理由 | 説明 |
| --- | --- |
| **別々に公開したい** | crates.io に別のパッケージとして出す |
| **コンパイル境界がほしい** | 片方だけ変更したときの再コンパイルを減らす（大規模なコードベースで効く） |
| **依存を分けたい** | 重い依存（Webフレームワーク等）を、コアのロジックから切り離す |
| **依存の方向を強制したい** | crate 間は循環依存できないので、設計上の一方向性をコンパイラに守らせられる |
| **バイナリが複数ある** | 共通ロジックを別 crate にして、複数のバイナリから使う |

最後から2番目が重要です。モジュールは相互参照できますが（12-1）、**crate はできません**。
「ドメインはインフラを知らない」といった規則を、規約ではなく**コンパイラに守らせる**ことができます。

## 典型的な構成

```text
my-app/
├── Cargo.toml          # [workspace] members = ["crates/*"]
└── crates/
    ├── core/           # ドメインのロジック。依存は最小
    ├── storage/        # DBアクセス。core に依存する
    └── cli/            # バイナリ。core と storage に依存する
```

依存の向きは `cli → storage → core` の一方向です。
`core` は `storage` を知らないので、`core` のテストにDBは要りません。

## この教材リポジトリの場合

```toml
[workspace]
resolver = "2"
members = ["exercises/*", "tools/check-exercises"]
exclude = ["exercises/ex012_non_exhaustive"]
default-members = ["tools/check-exercises"]
```

| 設定 | 理由 |
| --- | --- |
| `members = ["exercises/*"]` | 演習が増えても `Cargo.toml` を書き換えなくてよい（globパターン） |
| `exclude = [...]` | `ex012` は**それ自体が入れ子の workspace**（Lesson 03-4 で、別crateから見た `#[non_exhaustive]` の挙動を確かめるため）。親の glob に巻き込まれないよう除外する |
| `default-members` | ルートで `-p` なしの `cargo build` / `cargo test` が、未着手の演習（`todo!()` で失敗するのが正常）を巻き込まないようにする |
| `resolver = "2"` | 2021 edition 以降の機能解決。ビルド用依存とターゲット用依存の feature を混ぜない |

`default-members` は、「この workspace の**既定の作業対象**は何か」を宣言するものです。
**すべての crate をまとめてビルドすると困る構成**では、これを設定する意味があります。

## Think

> **問い**: 3つの crate（`core` / `storage` / `cli`）に分けた構成で、
> `core` のテストを実行するには、`storage` のコンパイルが必要でしょうか？

<details>
<summary>Solution</summary>

**必要ありません。** `cargo test -p core` は、`core` とその依存だけをビルドします。
`storage` は `core` に依存していますが、逆向きの依存は無いからです。

これが「依存の方向を一方向に保つ」ことの実利です。

- 下流（`core`）のテストが速い
- 下流を触っても、上流（`storage` / `cli`）のテストだけが再実行対象になる
- `core` に DB のモックが要らない

逆に、`core` が `storage` を（テストのためだけにでも）参照し始めると、この利点は消えます。
「テストのため」の依存は、`dev-dependencies` に入れても**循環していれば拒否されます**。
設計の問題を、コンパイラが教えてくれるわけです。

</details>

## Deep Dive: workspace で共通の設定を持つ

同じバージョン指定や lint 設定を各 crate に書くのは面倒です。workspace 全体で共有できます。

```toml
# ルートの Cargo.toml
[workspace.package]
edition = "2021"
license = "MIT"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
```

```toml
# crates/core/Cargo.toml
[package]
name = "core"
edition.workspace = true      # ルートから継承
license.workspace = true

[dependencies]
serde.workspace = true        # バージョンをルートで一元管理
```

依存のバージョンが crate ごとにずれる問題を防げます。
（この教材の演習は依存ゼロなので使っていませんが、実務の workspace では基本になります。）

## Exercise

**`ex048_workspace`** — `cd exercises/ex048_workspace && cargo test` で判定します
（`ex012` と同じく、入れ子の workspace なのでディレクトリ内で実行します）。

| crate | 役割 |
| --- | --- |
| `ex048_core` | ドメインのロジック。他の crate に依存しない |
| `ex048_storage` | `ex048_core` に依存する。メモリ上の簡易ストレージ |

`ex048_core` の crate から `ex048_storage` を使おうとすると、循環依存になってコンパイルできないことを、
`Cargo.toml` のコメントで確認します。

## Challenge

`ex048_core/Cargo.toml` の `[dependencies]` に `ex048_storage = { path = "../storage" }` を追加して、
`cargo build` を実行してみてください。どんなエラーになりますか。

## Review

- [ ] crate に分ける理由を複数挙げられる
- [ ] crate 間は循環依存できないことを、設計に利用できると説明できる
- [ ] `members` / `exclude` / `default-members` / `resolver` の役割を説明できる
- [ ] `workspace.dependencies` でバージョンを一元管理できることを知っている

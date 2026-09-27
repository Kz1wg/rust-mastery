# Lesson 02-4: フィールドの公開範囲とAPI

## Concept

**不変条件**（invariant）とは、「その型の値なら、いつでも必ず成り立っていてほしい条件」のことです。
例えば「範囲を表す `Range` なら、常に `min <= max`」のようなものです。

この条件が守られるのは、**その型の値を作ったり変えたりできる場所が限られている**ときだけです。
`pub` は「誰でも触ってよい」という宣言なので、`pub` なフィールドがあると、どこかで条件を破られてしまうかもしれません。

## Why?

02-3 で作った検証済みの型も、フィールドを `pub` にした瞬間、外から不正な値を書き込めます。
公開範囲は、「この型が何を保証するか」の一部です。

## Bad Example

```rust
pub struct Range {
    pub min: i32,
    pub max: i32, // 不変条件: min <= max
}

fn main() {
    let r = Range { min: 10, max: 1 }; // 不変条件が壊れている
    let _ = r;
}
```

## Problem

`Range` は「`min <= max`」を前提にしたメソッド（`contains` など）を持つかもしれません。
フィールドが `pub` だと、**その前提を守る責任が、すべての利用者に分散**します。

## Think

> **問い**:
> 1. `Range` に不変条件 `min <= max` を保証させるには、どうしますか？
> 2. 「値を読む」と「値を変更する」を、それぞれどう公開しますか？
> 3. `&mut` を返す getter を作ると、何が起きますか？

<details>
<summary>Hint</summary>

02-3 と同じく、フィールドを private にし、構築関数で検証します。
変更の手段を提供する場合、**変更の後にも不変条件が守られるか**を考えます。

</details>

<details>
<summary>Solution</summary>

```rust
mod range {
    #[derive(Debug, PartialEq)]
    pub struct RangeError;

    #[derive(Debug)]
    pub struct Range {
        min: i32,
        max: i32,
    }

    impl Range {
        pub fn new(min: i32, max: i32) -> Result<Range, RangeError> {
            if min <= max {
                Ok(Range { min, max })
            } else {
                Err(RangeError)
            }
        }

        pub fn min(&self) -> i32 { self.min }
        pub fn max(&self) -> i32 { self.max }

        pub fn contains(&self, x: i32) -> bool {
            self.min <= x && x <= self.max
        }

        // 変更も、不変条件を保つ形で提供する
        pub fn set_max(&mut self, max: i32) -> Result<(), RangeError> {
            if max < self.min {
                return Err(RangeError);
            }
            self.max = max;
            Ok(())
        }
    }
}

use range::{Range, RangeError};

fn main() {
    assert_eq!(Range::new(10, 1).err(), Some(RangeError));

    let mut r = Range::new(1, 10).unwrap();
    assert!(r.contains(5));
    assert_eq!(r.set_max(0), Err(RangeError));
    assert_eq!(r.max(), 10); // 失敗したので変更されていない
}
```

**`&mut` を返す getter は危険**です。`fn min_mut(&mut self) -> &mut i32` を公開すると、
呼び出し側が `*r.min_mut() = 100;` と書け、不変条件が壊れます。
可変な参照を外に出すことは、**そのフィールドを `pub` にするのとほぼ同じ**です。

| 公開の仕方 | 何を許すか | 不変条件は守れるか |
| --- | --- | --- |
| `pub` フィールド | 読み書き自由 | 守れない |
| `fn min(&self) -> i32` | 読み取りのみ | 守れる |
| `fn set_max(&mut self, ..) -> Result<..>` | 検証つきの変更 | 守れる |
| `fn min_mut(&mut self) -> &mut i32` | 読み書き自由 | 守れない |

</details>

## 公開範囲の道具

| 記法 | 見える範囲 | 使いどころ |
| --- | --- | --- |
| （なし） | 同じモジュール内 | フィールドの標準。不変条件を守る |
| `pub(super)` | 親モジュール | 兄弟モジュールで共有したい内部の型 |
| `pub(crate)` | 同じcrate内 | ライブラリ内部の共有。外部には出さない |
| `pub` | 外部に公開 | 安定させる約束をしたAPI |

`pub` にした時点で、それは**利用者との約束**になります。後から private に戻すことは破壊的変更です
（Chapter 12・16 で扱います）。

## Deep Dive: 何を「保証しない」かを書く

型のドキュメントには、**保証すること**だけでなく**保証しないこと**も書きます。

```rust
/// 検証済みの範囲。`min <= max` が常に成り立つ。
///
/// # 保証すること
/// - `min() <= max()`
///
/// # 保証しないこと
/// - 値の大きさの上限（オーバーフローは扱わない）
pub struct Range {
    min: i32,
    max: i32,
}

fn main() {
    let _ = Range { min: 0, max: 0 };
}
```

「何を保証するか」を書けない型は、設計がまだ曖昧なサインです。

## Exercise

**[`ex008_visibility`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex008_visibility)** — `cargo test -p ex008_visibility` で判定します。
（`cargo test` で自動判定するため、型定義や公開APIのシグネチャはあらかじめ用意してあります。本体の `todo!()` を実装してください。）

| 課題 | 仕様 |
| --- | --- |
| `Temperature` | 絶対零度（-273.15℃）未満を作れない型。`new` / `set_celsius` を実装（`celsius` は用意済み） |
| `Inventory` | 常に `reserved <= quantity` を保つ在庫。フィールドは private にしてあり、`new` / `available` / `reserve` / `release` を、**失敗時に状態を変えない**形で実装する |

外部から `Temperature` / `Inventory` を直接構築できないことは、`compile_fail` doctest で確認しています。
実装が終わったら、`Inventory` のフィールドを `pub` にしたと仮定して、どのテストや doctest が
意味を失うか考えてみてください。

## Challenge

`pub` フィールドのまま使いたい型（例: 座標 `Point { x, y }`）は、どういう性質を持っているでしょうか。
「不変条件がない型」の特徴を挙げてください。

## Review

- [ ] `pub` フィールドが不変条件を守れない理由を説明できる
- [ ] `&mut` を返す getter が危険な理由を説明できる
- [ ] `pub(crate)` などを、目的に合わせて選べる
- [ ] 型が「保証すること／しないこと」を文章にできる

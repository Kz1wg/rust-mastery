# Lesson 12-1: module分割の基準

## Concept

`mod` は名前空間と**可視性の境界**を作る仕組みです。
「ファイルが長くなったから分ける」だけでは、名前空間が増えるだけで、設計は何も良くなりません。

良い分割は、**モジュールの外に見せるものを減らします**。

## Why?

公開されている項目が多いほど、変更の影響範囲が読めなくなります。
「この関数を書き換えて大丈夫か」を判断するには、誰が使いうるかを知る必要があるからです。

## Bad Example: 分けたが、全部 `pub`

```rust
mod order {
    pub struct Order {
        pub id: u32,
        pub total: u64,
        pub tax: u64,
    }

    pub fn calculate_tax(total: u64) -> u64 {
        total / 10
    }

    pub fn apply_tax(order: &mut Order) {
        order.tax = calculate_tax(order.total);
    }
}

fn main() {
    let mut o = order::Order { id: 1, total: 1000, tax: 0 };
    order::apply_tax(&mut o);

    // 外から直接いじれてしまう
    o.tax = 0;
    let _ = order::calculate_tax(1000);
}
```

## Problem

ファイルは分かれましたが、`Order` のフィールドも内部計算の `calculate_tax` も全部公開されています。
外から `tax` を勝手に書き換えられるので、「`tax` は常に `total` の10%」という不変条件（02-4）が守れません。
**モジュールの内と外の区別が無いなら、分けた意味がありません。**

## Think

> **問い**: このモジュールが外に見せるべきものは何ですか？ 隠すべきものは何ですか？

<details>
<summary>Solution</summary>

```rust
mod order {
    pub struct Order {
        id: u32,
        total: u64,
        tax: u64,
    }

    impl Order {
        /// 構築時に税を計算する。以降、不変条件は常に保たれる。
        pub fn new(id: u32, total: u64) -> Self {
            Order { id, total, tax: calculate_tax(total) }
        }

        pub fn id(&self) -> u32 {
            self.id
        }

        pub fn total_with_tax(&self) -> u64 {
            self.total + self.tax
        }
    }

    /// 内部計算。モジュールの外からは見えない
    fn calculate_tax(total: u64) -> u64 {
        total / 10
    }
}

fn main() {
    let o = order::Order::new(1, 1000);
    assert_eq!(o.total_with_tax(), 1100);
    assert_eq!(o.id(), 1);
}
```

外に見えるのは `Order::new`・`id`・`total_with_tax` の3つだけになりました。
`calculate_tax` の実装を変えても、影響するのはこのモジュールの中だけです。

**分割の基準は、行数ではなく「外に見せる数を減らせるか」です。**
モジュールに分けても公開項目が減らないなら、その分割は名前空間を増やしただけです。

</details>

## Deep Dive: モジュール同士は相互参照できる

crate 同士は循環依存できませんが、**同じcrate内のモジュールは相互に参照できます**。

```rust
mod a {
    pub fn f() -> u32 {
        super::b::g()
    }
}

mod b {
    pub fn g() -> u32 {
        42
    }
    pub fn h() -> u32 {
        super::a::f()
    }
}

fn main() {
    assert_eq!(a::f(), 42);
    assert_eq!(b::h(), 42);
}
```

できるからといって、**すべきとは限りません**。相互参照するモジュールは、実質1つのまとまりです。
「a が b を呼び、b も a を呼ぶ」状態になったら、分け方を見直すか、共通部分を3つ目のモジュールへ切り出すことを検討してください。
依存の向きを一方向に保つことは、12-4 の主題でもあります。

## Exercise

**`ex046_module_boundaries`** — `cargo test -p ex046_module_boundaries` で判定します。

`order` モジュールを、公開項目を最小にした形で実装します。
`compile_fail` doctest で、内部の計算関数やフィールドが外から触れないことを確認します。

## Challenge

自分のコードで最も長いファイルを開き、「外に見せる数を減らせる分割」があるか探してください。
無いなら、分割しないほうが良いかもしれません。

## Review

- [ ] モジュール分割の価値が「外に見せる数を減らすこと」にあると説明できる
- [ ] 分けても全部 `pub` なら意味が無い理由を説明できる
- [ ] モジュールの相互参照が可能だが、設計の見直しのサインであることを説明できる

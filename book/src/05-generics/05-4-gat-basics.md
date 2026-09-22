# Lesson 05-4: GATの基本用途

## Concept

通常のassociated type（Chapter 04-3）は、「1つの型につき1つに決まる型」を表現します。
しかし、**「1つの型につき、呼び出しごとに（lifetimeによって）違う型」**を表現したい場面があります。
これがGAT（Generic Associated Types）が必要になる理由です。

このLessonの目標は「GATを使いこなす」ことではありません。**なぜ普通のassociated typeでは
足りないのか**を、最小限の例で理解することです。

## Why?

`&self` を借用するイテレータを返すメソッドを考えます。借用の生存期間は、
**そのメソッドを呼んだ場所ごとに違います**。普通のassociated typeには、
その「呼び出しごとに違うlifetime」を表現する方法がありません。

## Bad Example: 普通のassociated typeで挑戦する

```rust,compile_fail,E0637
trait Container {
    type Iter: Iterator<Item = i32>;
    fn iter(&self) -> Self::Iter;
}

struct Numbers(Vec<i32>);

impl Container for Numbers {
    type Iter = std::slice::Iter<'_, i32>; // 'a をどこにも書けない
    fn iter(&self) -> Self::Iter {
        self.0.iter()
    }
}

fn main() {}
```

## Problem

`std::slice::Iter<'a, i32>` は、借用元のスライスと同じだけ生きる参照のイテレータです。
`'a` は本来「`iter()` を呼んだときの `&self` の借用」に結びつくべきlifetimeですが、
`type Iter = ...;` という宣言のどこにも、そのlifetimeを受け取る場所がありません。

`'_` を書いてみても、「`type` 宣言の中で使える名前ではない」というエラー（`E0637`）になります。

## Think

> **問い**: `Self::Iter` という型そのものに、lifetimeパラメータを持たせることはできるでしょうか？
> （`Vec<T>` が `T` という型パラメータを持てるように、`Iter` に `'a` を持たせる。）

<details>
<summary>Hint</summary>

`type Iter<'a>: ...;` のように、associated type自体に**パラメータ**を持たせられないか考えてください。

</details>

<details>
<summary>Solution</summary>

```rust
trait Container {
    type Iter<'a>: Iterator<Item = &'a i32>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}

struct Numbers(Vec<i32>);

impl Container for Numbers {
    type Iter<'a> = std::slice::Iter<'a, i32>;

    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter()
    }
}

fn main() {
    let n = Numbers(vec![1, 2, 3]);
    let sum: i32 = n.iter().sum();
    println!("{sum}");
}
```

`type Iter<'a>` は、**associated type自身がgenericパラメータ（ここではlifetime）を持てる**、
という宣言です。これがGeneric Associated Type（GAT）です。
`fn iter(&self) -> Self::Iter<'_>` の `'_` は、「呼ばれるたびに、その呼び出しでの `&self` の
借用に対応するlifetimeが入る」ことを表しています。

普通のassociated type（`type Iter: ...;`）は、実装ごとに**1つの型**に決まるだけで、
その型が「呼び出しごとに違うlifetimeを持つ」ことを表現できません。GATは、
associated type自体をgeneric（ここではlifetimeに関して）にすることで、この制約を外します。

</details>

## Deep Dive: なぜ「基本用途」だけでよいのか

GATは、次のような場面で必要になります。

- **borrowするイテレータ**（今回の例。`Self::Item` が `&self` のlifetimeに依存する）
- 非同期トレイトの一部の高度な設計（`async fn` をtraitに書く際の内部的な仕組みの一部）
- 型レベルでより精密な制約を表現したいライブラリ設計

これらはいずれも**ライブラリ設計の高度な場面**であり、
アプリケーションコードを書く上で自分でGATを定義する機会は多くありません。
このLessonの到達目標は、「GATを自在に書けるようになる」ことではなく、
**「借用を返すメソッドで、なぜ普通のassociated typeが破綻するのか」を理解し、
必要になったときにGATという名前を思い出せる**ことです。

## Exercise

**`ex021_gat_basics`** — `cargo test -p ex021_gat_basics` で判定します。

`Container` trait をGATで定義し、`Numbers` に実装します。

## Challenge

標準ライブラリの `Iterator` trait には、なぜ `next(&mut self) -> Option<Self::Item>` の
`Self::Item` がGATになっていないのか（＝lifetimeパラメータを持たないのか）を考えてみてください。
（ヒント: `Item` が `&self` を借用する必要があるとは限りません。）

## Review

- [ ] 普通のassociated typeが「呼び出しごとに違うlifetime」を表現できない理由を説明できる
- [ ] GATが「associated type自体にパラメータを持たせる」仕組みであることを説明できる
- [ ] GATが必要になる典型的な場面（borrowするイテレータなど）を1つ挙げられる

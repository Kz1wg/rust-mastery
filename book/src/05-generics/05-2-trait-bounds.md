# Lesson 05-2: trait boundsの設計

## Concept

trait boundは、「この関数がTに対して実際に何をするか」の一覧であるべきです。
「念のため」で付け足したboundは、呼び出し側に不要な制約を強います。

## Why?

bound が多すぎると、その型を渡せる呼び出し側の範囲が狭まります。
関数の中身が使っていないboundは、読む人にとって「なぜこれが必要なのか」が説明できません。

## Bad Example

```rust
fn print_and_clone<T>(x: T) -> T
where
    T: std::fmt::Display + Clone + Default + PartialEq + std::fmt::Debug,
{
    println!("{x}");
    x.clone()
}

fn main() {
    let s = print_and_clone("hello".to_string());
    println!("{s}");
}
```

## Problem

関数の中身は `println!("{x}")`（`Display`が必要）と `x.clone()`（`Clone`が必要）しかしていません。
`Default`・`PartialEq`・`Debug` は**一度も使われていません**。

これらのboundのせいで、`Display + Clone` は満たすが `Default` は満たさない型
（例えば「既定値の概念が無い」型）は、この関数に渡せなくなります。

## Think

> **問い**: 関数の中身を1行ずつ見て、実際に必要なtrait boundだけを残してください。

<details>
<summary>Solution</summary>

```rust
fn print_and_clone<T: std::fmt::Display + Clone>(x: T) -> T {
    println!("{x}");
    x.clone()
}

fn main() {
    let s = print_and_clone("hello".to_string());
    println!("{s}");
}
```

**判断の仕方は機械的です**: 関数本体を1行ずつ見て、その行がどのtraitのメソッドを呼んでいるかを確認する。
使われていないboundは削除する。これだけで、呼び出し側が渡せる型の範囲が広がります。

</details>

## `<T: Trait>` と `where` の使い分け

```rust
// (A) インライン
fn f<T: std::fmt::Display + Clone>(x: T) -> T { x }

// (B) where句
fn f2<T>(x: T) -> T
where
    T: std::fmt::Display + Clone,
{
    x
}
```

boundが1つか2つで、行に収まるなら (A) で構いません。
**`where` が必要になるのは、次のような場合です。**

- boundが増えて1行に収まらない、または複数の型パラメータそれぞれに条件がある
- **型パラメータ自体ではなく、型パラメータを使った式にboundを課したい**とき

後者は `<T: Trait>` の構文では書けません。例えば:

```rust
fn to_owned_string<T>(x: T) -> String
where
    String: From<T>,
{
    String::from(x)
}

fn main() {
    assert_eq!(to_owned_string("abc"), "abc"); // String: From<&str>
    assert_eq!(to_owned_string('x'), "x"); // String: From<char>
}
```

`String: From<T>` は、**左辺が型パラメータ `T` ではなく具体的な型 `String`** のboundです。
「`T` から `String` を作れる」という条件を、`T` 側のtraitとしてではなく
`String` 側の実装として表現しています。この形は `<T: ...>` の位置には書けず、
`where` 句だけが持つ表現力です。

## Deep Dive: boundを絞ると「呼び出し側の型」が広がる

```rust
fn find_max_ord<T: Ord + Clone>(items: &[T]) -> Option<T> {
    items.iter().max().cloned()
}
```

`Ord` は「常に全順序で比較できる」ことを要求します。しかし `f64` は `NaN` があるため
`Ord` を実装できません（`PartialOrd` のみ）。もし`Ord`ではなく`PartialOrd`ベースの比較で
書き直せれば、`f64` のスライスにもこの関数を使えるようになります
（本Lessonでは深入りしませんが、`items.iter().reduce(|a, b| if a > b { a } else { b })` の
ような書き方で `PartialOrd` だけに緩められます）。

**boundを絞ることは、単なる整理整頓ではありません。呼び出し側が渡せる型の範囲を
実際に広げる、具体的な効果があります。**

## Exercise

**`ex019_trait_bounds`** — `cargo test -p ex019_trait_bounds` で判定します。

不要なboundが付いた関数から、実際に使われているboundだけを残すように直します。

## Challenge

自分のコードのgeneric関数を1つ選び、trait boundを1つずつ「本当に使っているか」確認してください。

## Review

- [ ] 関数本体を読んで、実際に使われているtrait boundだけを残せる
- [ ] `<T: Trait>` と `where` の使い分け（特に `where` でしか書けない場合）を説明できる
- [ ] boundを絞ることが、呼び出し側の型の範囲を広げる具体的な効果を持つことを説明できる

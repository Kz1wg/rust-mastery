# Lesson 08-4: HRTB入門

## Concept

「関数の**中で作った**値への参照を、引数で受け取ったクロージャに渡す」とき、
そのクロージャは「**どんな lifetime の参照でも**受け取れる」必要があります。
これを表すのが `for<'a>`（Higher-Ranked Trait Bounds, HRTB）です。

このLessonの目標は、HRTBを自在に書くことではありません。
**なぜ普通の lifetime パラメータでは書けないのか**を理解し、`Fn(&str)` と書いたときに
裏で何が起きているかを知ることです。

## Why?

`for<'a>` は普段目にしませんが、実はクロージャを受け取る関数の多くで、**暗黙のうちに使われています**。
それを知らないと、lifetimeを「丁寧に」書いたつもりでエラーに遭遇します。

## Bad Example

```rust,compile_fail,E0597
fn apply<'a, F: Fn(&'a str) -> usize>(f: F) -> usize {
    let s = String::from("hello");
    f(&s)
}

fn main() {}
```

## Problem

`'a` は `apply` の**型パラメータ**なので、**呼び出し側が**決めます。呼び出し側から見て `'a` は
`apply` の呼び出しより長く生きる期間です。しかし `s` は `apply` の中で作られ、中で破棄されます。
`&s` が呼び出し側の決めた `'a` の間生きることはあり得ません。

本当に言いたいのは、「`f` は `'a` という**特定の**期間の参照を受け取る」ではなく、
「`f` は**どんな期間の参照でも**受け取れる」ということです。

## Think

> **問い**: 「どんな lifetime でも」という条件は、どう書けばよいでしょうか？

<details>
<summary>Solution</summary>

```rust
fn apply<F>(f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    let s = String::from("hello");
    f(&s)
}

fn main() {
    println!("{}", apply(|s| s.len()));
}
```

`for<'a> Fn(&'a str) -> usize` は、「**すべての** `'a` について、`Fn(&'a str) -> usize` を満たす」という意味です。
`'a` を決めるのは呼び出し側ではなく、`f` を**呼ぶたび**に決まります。
そのため、関数の中で作った一時的な値への参照も渡せます。

そして重要なのは、これは**普段書いている形と同じ意味**だということです。

```rust
fn apply<F: Fn(&str) -> usize>(f: F) -> usize {
    let s = String::from("hello");
    f(&s)
}

fn main() {
    println!("{}", apply(str::len));
}
```

`Fn(&str) -> usize` と lifetime を省略して書くと、コンパイラは自動的に
`for<'a> Fn(&'a str) -> usize` として扱います。**Bad Example は、省略すれば動いたものを、
lifetime を「明示」したことで壊していた**のです。

</details>

## Deep Dive: いつ `for<'a>` を自分で書くのか

`Fn(&str)` のような形では省略で済むため、`for<'a>` を明示的に書く場面は多くありません。
書く必要が出てくるのは、主に次のような場合です。

- trait bound の中で、クロージャ以外の trait に「任意の lifetime の参照について」という条件を付けたいとき
  （例: `where for<'a> &'a T: IntoIterator`）
- ライブラリの内部で、型レベルの精密な制約を書くとき

アプリケーションコードで出会うのは、主に**エラーメッセージの中**です。
`for<'a>` を見かけたら、「どんな lifetime でも受け取れることが要求されている」と読んでください。

## Exercise

**`ex033_hrtb`** — `cargo test -p ex033_hrtb` で判定します。

| 関数 | シグネチャ（用意済み） | 実装すること |
| --- | --- | --- |
| `apply_to_local` | `fn apply_to_local<F>(f: F) -> usize where F: for<'a> Fn(&'a str) -> usize` | 関数の中で `String` を作り、その参照を `f` に渡す |
| `count_matching` | `fn count_matching<F: Fn(&str) -> bool>(items: &[String], pred: F) -> usize` | `pred` が真になる要素を数える |

2つの関数の bound は、書き方は違っても同じ種類の条件です。なぜそう言えるのか説明してください。

## Challenge

`apply_to_local` の bound を `fn apply_to_local<'a, F: Fn(&'a str) -> usize>(f: F) -> usize` に書き換え、
どんなエラーになるか確かめてください。

## Review

- [ ] 関数の lifetime パラメータは「呼び出し側が決める」ことを説明できる
- [ ] `for<'a>` が「すべての lifetime について」を意味することを説明できる
- [ ] `Fn(&str)` の省略形が、暗黙に `for<'a>` になっていることを説明できる

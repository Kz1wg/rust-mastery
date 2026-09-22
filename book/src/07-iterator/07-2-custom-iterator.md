# Lesson 07-2: `Iterator` trait

## Concept

`Iterator` を自作する必要があるのは、**既存のadapterの組み合わせでは表現できない、
独自の「次の値の計算方法」がある**ときです。

## Why?

「必要な値を毎回全部計算してVecに貯めてから返す」設計は、
呼び出し側が最初の数個しか使わない場合や、そもそも値が無限に続く場合に無駄が生じます。

## Bad Example: 全部を前もって計算する

```rust
fn fibonacci_up_to(count: usize) -> Vec<u64> {
    let mut result = Vec::with_capacity(count);
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..count {
        result.push(a);
        let next = a + b;
        a = b;
        b = next;
    }
    result
}

fn main() {
    // 最初の3つしか要らないのに、10個分計算してVecに詰めている
    let first_three: Vec<u64> = fibonacci_up_to(10).into_iter().take(3).collect();
    println!("{first_three:?}");
}
```

## Problem

`take(3)` で最初の3つしか使わないのに、`fibonacci_up_to` は指定された `count` の分だけ
**必ず計算してVecに詰めます**。呼び出し側が実際に必要とする数と、計算する数が一致していません。

## Think

> **問い**: 「必要になった分だけ、その場でフィボナッチ数を計算する」ように書き直すには、
> どうすればよいですか？

<details>
<summary>Hint</summary>

`Iterator` トレイトは、`fn next(&mut self) -> Option<Self::Item>` の1つのメソッドだけを
要求します（あとは全て、`next` を使ったデフォルト実装です）。**「次の1つを計算する方法」**さえ
書けば、`take`・`map`・`filter` などのadapterが全て自動的に使えるようになります。

</details>

<details>
<summary>Solution</summary>

```rust
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let current = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(current)
    }
}

fn main() {
    let first_three: Vec<u64> = Fibonacci::new().take(3).collect();
    println!("{first_three:?}");
}
```

`Fibonacci` は**概念上**無限に値を返します（`next` が常に `Some` を返すため）。

> **注意（訂正）**: 実際には `u64` の上限があります。fib(94) が `u64` を超え、この実装は
> 「2つ先」を計算するので、**93個目を取り出す時点で溢れます**（debugビルドではpanic、
> releaseビルドでは黙って値が回り込みます）。本当に安全な無限列にするなら、
> `checked_add` が `None` を返したら `None` を返して列を終える、といった設計が必要です。
> 「無限列」を型で表しても、値の範囲の問題は消えません。
`take(3)` は、**実際に3回 `next` を呼んだ時点で止まります**。
呼び出し側が必要とする数だけ計算され、無駄がありません。

`Iterator` を実装したことで、`take` 以外にも `map`・`filter`・`zip` など、
標準ライブラリのadapterが**すべて自動的に使えるように**なっています。
自分で実装したのは `next` だけです。

</details>

## Deep Dive: `Iterator` は1つのメソッドから多くを引き出す

`Iterator` トレイトのメソッドの大半（`map`, `filter`, `take`, `fold`, `sum`, ...）は
**デフォルト実装**です。実装する側が用意する必要があるのは、原則として `next` だけです
（`size_hint` などは任意で、最適化のために上書きできます）。

これは、Chapter 04で見た「traitは呼び出し側への保証」という考え方の良い例です。
`Iterator` は「`next` さえあれば、値の列を順番に取り出せる」ことを保証し、
その保証の上に大量の便利なメソッドを築いています。

## Exercise

**`ex027_custom_iterator`** — `cargo test -p ex027_custom_iterator` で判定します。

`Fibonacci` の `Iterator` 実装を書きます。

## Challenge

自分のコードで、「必要な分だけ計算する」のではなく「全部計算してから一部だけ使う」
書き方をしている箇所がないか探してください。

## Review

- [ ] 自作の `Iterator` が必要になる場面（無限列、遅延計算）を説明できる
- [ ] `Iterator` の実装が `next` だけで、他のメソッドがデフォルト実装で使えることを説明できる

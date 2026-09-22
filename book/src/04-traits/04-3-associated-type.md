# Lesson 04-3: associated type

## Concept

`trait Container<T>` と `trait Container { type Item; }` は、どちらも「中身の型」を扱う方法です。
違いは、**1つの型が、そのtraitを何通りの型引数で実装できるか**です。

## Why?

正しい方を選ばないと、呼び出し側が毎回型注釈を書かされたり、逆に本当は必要な柔軟性を失ったりします。

## Bad Example

```rust
trait Stack<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
}

struct IntStack {
    items: Vec<i32>,
}

impl Stack<i32> for IntStack {
    fn push(&mut self, item: i32) {
        self.items.push(item);
    }
    fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }
}

// IntStack は i32 専用なのに、関数側は T という型パラメータを別に持たされる
fn drain_all<T>(s: &mut impl Stack<T>) -> Vec<T> {
    let mut out = Vec::new();
    while let Some(x) = s.pop() {
        out.push(x);
    }
    out
}

fn main() {
    let mut s = IntStack { items: vec![1, 2, 3] };
    // 今は impl が1つしかないので、T = i32 と推論できる
    let all = drain_all(&mut s);
    println!("{all:?}");
}
```

## Problem

`IntStack` は `i32` 以外の型で `Stack` を実装することは決してありません（`items: Vec<i32>` に固定されているため）。
それなのに `Stack<T>` は「1つの型が複数の `T` で実装できる」余地を持たせた設計になっています。

上の `drain_all(&mut s)` が型注釈なしで通るのは、**今たまたま実装が1つしかないから**です。
誰かが `impl Stack<String> for IntStack` を追加した瞬間、コンパイラは `T` を決められなくなり、
**既存の呼び出し側がすべて壊れます**:

```rust,compile_fail,E0283
trait Stack<T> {
    fn pop(&mut self) -> Option<T>;
}

struct IntStack {
    items: Vec<i32>,
}

impl Stack<i32> for IntStack {
    fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }
}

// 後から誰かが追加した2つ目の実装
impl Stack<String> for IntStack {
    fn pop(&mut self) -> Option<String> {
        None
    }
}

fn drain_all<T>(s: &mut impl Stack<T>) -> Vec<T> {
    let mut out = Vec::new();
    while let Some(x) = s.pop() {
        out.push(x);
    }
    out
}

fn main() {
    let mut s = IntStack { items: vec![1] };
    let all = drain_all(&mut s); // T が i32 か String か決まらない
    let _ = all;
}
```

「1つの型につき `Item` は1つ」という事実が型で表現されていないため、
こういう壊れ方を防げません。

## Think

> **問い**:
> 1. `IntStack` にとって、`Item` の型は何通りありますか？
> 2. 「1つの型につき`Item`は1つに決まる」という関係を、Rustの型システムでどう表現しますか？
> 3. `associated type` に変えると、`drain_all` の呼び出し方はどう変わりますか？

<details>
<summary>Hint</summary>

`Iterator` トレイトの `type Item;` を思い出してください。
`Vec<i32>` の `IntoIter` は `i32` の `Iterator` であって、他の型の `Iterator` には**なれません**。

</details>

<details>
<summary>Solution</summary>

```rust
trait Stack {
    type Item;
    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
}

struct IntStack {
    items: Vec<i32>,
}

impl Stack for IntStack {
    type Item = i32;
    fn push(&mut self, item: i32) {
        self.items.push(item);
    }
    fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }
}

fn drain_all<S: Stack>(s: &mut S) -> Vec<S::Item> {
    let mut out = Vec::new();
    while let Some(x) = s.pop() {
        out.push(x);
    }
    out
}

fn main() {
    let mut s = IntStack { items: vec![1, 2, 3] };
    let all = drain_all(&mut s); // Item は IntStack から一意に決まる（実装を増やして曖昧になる余地が無い）
    println!("{all:?}");
}
```

`drain_all` のシグネチャから `T` が消え、`S::Item` に置き換わりました。
呼び出し側は `Item` の型を書く必要がありません——`IntStack` を渡した時点で `i32` に決まるからです。

</details>

## 使い分けの基準

| 状況 | 選ぶ |
| --- | --- |
| ある型について、`Item`（や`T`）が**1つに決まる** | associated type |
| 1つの型が、**複数の`T`で同じtraitを実装しうる**（例: `From<i32> for MyNum` と `From<f64> for MyNum` の両方が欲しい） | generic parameter |
| 標準ライブラリとの整合性（`Iterator`, `IntoIterator` は associated type） | 倣うなら associated type |

`From<T>` が generic parameter を使っているのは、まさに「1つの型が複数の `T` から変換できてよい」
という性質を表すためです。`MyNum` は `From<i32>` も `From<f64>` も実装できます——
これは `Stack` のケースとは逆で、**複数の実装が意味を持つ**からです。

## Deep Dive: 呼び出し側から見た違い

associated type を使うと、trait bound の書き方も変わります。

```rust,ignore
// generic parameter 版: T を書く必要がある
fn use_stack_generic<T, S: Stack<T>>(s: &mut S) { /* ... */ }

// associated type 版: S::Item として参照する。T という別のパラメータは不要
fn use_stack_assoc<S: Stack>(s: &mut S) -> S::Item where S::Item: Default {
    s.pop().unwrap_or_default()
}
```

associated type 版は、trait boundに登場する型パラメータが減り、シグネチャが読みやすくなります。
これは「1つの型につき1通りに決まる」関係を型システムで正しく表現した、当然の結果です。

## Exercise

**`ex015_associated_type`** — `cargo test -p ex015_associated_type` で判定します。

`Stack` trait を associated type で定義し、`IntStack` に実装します。
`S::Item` を使う generic 関数 `drain_all` も実装します。

## Challenge

`From<T>` が generic parameter を使っている理由を、このLessonの基準に沿って自分の言葉で説明してください。

## Review

- [ ] associated type と generic parameter の違い（1:1 か 1:多 か）を説明できる
- [ ] `Iterator` の `type Item` が associated type である理由を説明できる
- [ ] `From<T>` が generic parameter である理由を説明できる

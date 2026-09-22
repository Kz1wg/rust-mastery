# Lesson 05-3: `impl Trait`

## Concept

`impl Trait` は、引数位置と戻り値位置で**違う意味**を持ちます。
どちらも「具体的な型名を書かずに済む」という見た目は同じですが、コンパイラの扱いは別物です。

## Why?

この違いを知らないと、「戻り値位置の `impl Trait` は複数の型を返せる」と誤解し、
コンパイルエラーの理由が分からなくなります。

## 引数位置: genericの省略記法

```rust
fn show(x: impl std::fmt::Display) {
    println!("{x}");
}

// 上と全く同じ意味
fn show_generic<T: std::fmt::Display>(x: T) {
    println!("{x}");
}
```

引数位置の `impl Trait` は、**generic parameterの匿名版**です。呼び出し側が渡した具体的な型に、
コンパイルの時点で置き換わります（単相化）。

## Think

> **問い**: 次のコードはコンパイルできるでしょうか？

```rust,compile_fail,E0308
fn same_type_pair(a: impl PartialEq, b: impl PartialEq) -> bool {
    a == b
}

fn main() {
    same_type_pair(1, 2);
}
```

<details>
<summary>Hint</summary>

`a: impl PartialEq` と `b: impl PartialEq` は、**別々の匿名の型パラメータ**です。
`a` が `i32`、`b` も `i32` だとしても、コンパイラにとっては「たまたま同じ型が渡された」だけで、
シグネチャ上は**別の型でもよい**ことになっています。

</details>

<details>
<summary>Solution</summary>

`a == b` は、`a` と `b` が**同じ型**であることを要求します
（`PartialEq<Rhs>` はデフォルトで `Rhs = Self`）。しかし `impl Trait` の引数は、
それぞれ独立した型パラメータとして扱われるため、コンパイラは「型が違うかもしれない」と判断します。

**同じ型であることを要求したいなら、名前付きのgenericパラメータを使う必要があります。**

```rust
fn same_type_pair<T: PartialEq>(a: T, b: T) -> bool {
    a == b
}

fn main() {
    assert!(same_type_pair(1, 2) == false);
    assert!(same_type_pair(3, 3));
}
```

**使い分けの基準**: 型パラメータが1箇所にしか現れず、他の引数や戻り値と型を揃える必要がないなら
`impl Trait`。**2つ以上の引数を同じ型に揃えたい、あるいは戻り値の型を引数の型と揃えたいなら、
名前付きのgenericパラメータが必要**です。

</details>

## 戻り値位置: 「1つの具体的な型」を隠す

```rust
fn make_numbers() -> impl Iterator<Item = i32> {
    (1..5).map(|x| x * 2)
}

fn main() {
    let sum: i32 = make_numbers().sum();
    println!("{sum}");
}
```

戻り値位置の `impl Trait` は、「呼び出し側には具体的な型名を教えないが、
関数の中では**1つの具体的な型に決まっている**」ことを意味します。
`(1..5).map(...)` の型は `std::iter::Map<std::ops::Range<i32>, ...>` という長い名前を持ちますが、
`impl Iterator<Item = i32>` と書けば、それを書かずに済みます。

## Think

> **問い**: 次のコードは、なぜコンパイルできないのでしょうか？

```rust,compile_fail,E0308
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}
struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side
    }
}

fn make_shape(round: bool) -> impl Shape {
    if round {
        Circle { radius: 1.0 }
    } else {
        Square { side: 1.0 }
    }
}

fn main() {}
```

<details>
<summary>Solution</summary>

`impl Shape` という戻り値の型は、**コンパイル時に1つの具体的な型に確定していなければなりません**。
`if` の分岐によって `Circle` にも `Square` にもなりうる、というのは矛盾します
（コンパイラが実際に「この関数は `Circle` 型の値を返す」とコードを生成しようとした瞬間、
`else` 節で `Square` が出てくるため、単一の型に定まりません）。

異なる具象型を条件によって返し分けたいなら、**戻り値の型自体を実行時に決める必要があり**、
それは `dyn Trait`（Chapter 04-2）の仕事です。

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}
struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side
    }
}

fn make_shape(round: bool) -> Box<dyn Shape> {
    if round {
        Box::new(Circle { radius: 1.0 })
    } else {
        Box::new(Square { side: 1.0 })
    }
}

fn main() {
    let s = make_shape(true);
    println!("{}", s.area());
}
```

`Box<dyn Shape>` は、実行時に「中身が何であるか」を持ち運べる型なので、
分岐によって異なる具象型を返しても構いません（Chapter 04-2の静的/動的ディスパッチの違いが、
そのままこの制約の理由になっています）。

</details>

## Deep Dive: 戻り値位置の `impl Trait` は「呼び出し側から見えない型」

戻り値位置の `impl Trait` は、関数の実装者にとっては具体的な型が1つに決まっていますが、
**呼び出し側はその型の名前を知ることができません**。これは意図的な設計で、
「戻り値の型を変更しても、呼び出し側のコードは壊れない」という自由を実装者に与えます
（trait boundを満たす限り、内部で使う具体的なイテレータの実装を変えても、呼び出し側には影響しません）。

## Exercise

**`ex020_impl_trait`** — `cargo test -p ex020_impl_trait` で判定します。

引数位置・戻り値位置それぞれの `impl Trait` を使った関数を実装します。

## Challenge

自分のコードで `impl Trait` を戻り値に使っている関数を探し（無ければ標準ライブラリの
`str::chars()` などを調べてください）、なぜ `dyn Trait` ではなく `impl Trait` で
書けているのか（＝常に1つの具体的な型を返しているか）を確認してください。

## Review

- [ ] 引数位置の `impl Trait` が、名前を持たないgenericパラメータであることを説明できる
- [ ] 2つの引数を同じ型に揃えたいときは、名前付きgenericが必要な理由を説明できる
- [ ] 戻り値位置の `impl Trait` が「1つの具体的な型」を要求する理由を説明できる
- [ ] 条件によって異なる具象型を返したいときは `Box<dyn Trait>` を使う、と判断できる

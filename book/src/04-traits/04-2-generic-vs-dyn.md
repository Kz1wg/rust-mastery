# Lesson 04-2: generic vs trait object

## Concept

同じtraitでも、`fn f<T: Shape>(x: &T)` と `fn f(x: &dyn Shape)` はコンパイル後、全く違うコードになります。
どちらを使うべきかは、**「呼び出し側が異なる型を混在させたいかどうか」**でほぼ決まります。

## Why?

「動くから」で選ぶと、無駄なコストを払ったり、逆に必要な柔軟性を得られなかったりします。

## Bad Example

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
        std::f64::consts::PI * self.radius * self.radius
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    // Circle と Square は別の型なので、同じ Vec に入れられない
    let shapes = vec![Circle { radius: 1.0 }, Square { side: 2.0 }];
}
```

## Problem

`Circle` と `Square` は別の型なので、`Vec<T>` の `T` を一意に決められません。
`total_area<T: Shape>(shapes: &[T])` のようなgeneric関数を書いても、
呼び出せるのは「`Circle` だけのスライス」か「`Square` だけのスライス」のどちらかです。

## Think

> **問い**:
> 1. `Circle` と `Square` を同じコレクションに混在させるには、どうすればよいですか？
> 2. そのとき、実行時のコストは generic 版と比べてどう変わりますか？
> 3. 逆に、`Circle` だけを大量に扱うホットループでは、どちらを選ぶべきですか？

<details>
<summary>Hint</summary>

「異なる具象型を、同じtraitを実装しているというだけの理由でひとまとめに扱いたい」
——これが `dyn Trait` の出番です。

</details>

<details>
<summary>Solution</summary>

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
        std::f64::consts::PI * self.radius * self.radius
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// (A) 静的ディスパッチ: T は呼び出しごとに1つの具象型に固定される
fn total_area_generic<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(Shape::area).sum()
}

// (B) 動的ディスパッチ: 異なる具象型を混在できる
fn total_area_dyn(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    let circles = vec![Circle { radius: 1.0 }, Circle { radius: 2.0 }];
    println!("{}", total_area_generic(&circles));

    let mixed: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    println!("{}", total_area_dyn(&mixed));
}
```

表に出てくる言葉を先に説明しておきます。

- **静的ディスパッチ**: どの関数を呼ぶかが**コンパイル時に決まる**呼び方。generic はこちら。
  コンパイラは、使われた型ごとに関数のコピーを作ります（`total_area_generic::<Circle>` と `total_area_generic::<Square>` が別々に作られる）。
  これを**単相化**（monomorphization）と言います。
- **動的ディスパッチ**: どの関数を呼ぶかが**実行時に決まる**呼び方。`dyn Trait` はこちら。
  `Box<dyn Shape>` は、中身のデータへのポインタと一緒に、「この型の `area` はここにある」という
  **関数の住所録**へのポインタを持っています。この住所録を **vtable** と言います。

| 観点 | (A) generic（静的ディスパッチ） | (B) `dyn Trait`（動的ディスパッチ） |
| --- | --- | --- |
| 型の決まる時機 | コンパイル時（`T` ごとにコードが複製される＝単相化） | 実行時（vtable経由で呼び出し先を決める） |
| 呼び出しコスト | 直接呼び出し。インライン化もされうる | vtable経由の間接呼び出し（わずかなオーバーヘッド） |
| 異なる型の混在 | できない（1回の呼び出しにつき`T`は1つ） | できる（`Box<dyn Shape>` はどれも同じ型として扱える） |
| バイナリサイズ | `T` の種類だけコードが複製される（コードサイズ増） | 実装は1つだけ（コードサイズは小さい） |

**「混在させたい」なら (B)、そうでなければ (A) がまず選ぶべき既定**です。
(A) の方がコストが低く、コンパイラの最適化も効きやすいので、
「どちらでもよい」ときは (A) を選んでください。

</details>

## Deep Dive: object safety（trait objectにできないtrait）

すべてのtraitが `dyn Trait` にできるわけではありません。

```rust,compile_fail,E0038
trait Cloneable {
    fn duplicate(&self) -> Self;
}

fn use_dyn(x: &dyn Cloneable) {
    let _ = x;
}

fn main() {}
```

`duplicate` の戻り値が `Self` なので、コンパイラは「`dyn Cloneable` として呼ばれたとき、
戻り値の型（実際のサイズ）が分からない」という問題に直面します。vtableは
「同じシグネチャの関数ポインタの集まり」であり、呼び出し先ごとにサイズが変わる `Self` を
戻り値にできません。

trait がobject safeであるための条件（主なもの）:

- `Self` はレシーバ（`&self`, `&mut self`, `self: Box<Self>` など）以外の位置に現れない。
  **戻り値に `Box<Self>` を書くのもNG**です（`Self` を値で返すのと同じく E0038 になります）
- メソッドにgenericパラメータが無い
- trait自体が `Sized` を前提にしていない（`trait Foo: Sized` のように、「この trait を実装するなら `Sized` も必要」と書いていない。
  このように trait に前提として付ける trait を **supertrait** と言います）

ただし、個々のメソッドに `where Self: Sized` を付けると、そのメソッドは
`dyn Trait` からは呼べなくなる代わりに、trait全体のobject safetyを壊さなくなります。
「一部のメソッドだけ具象型専用にする」逃げ道として使われます。

このため、`Clone` トレイト自体も `dyn Clone` にはできません
（`fn clone(&self) -> Self` を持つため）。`dyn Trait` として使いたいコレクションに
複製可能性を持たせたい場合は、`dyn_clone` のような別パターンを使います
（このLessonの範囲を超えるため、ここでは深追いしません）。

## Exercise

**[`ex014_generic_vs_dyn`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex014_generic_vs_dyn)** — `cargo test -p ex014_generic_vs_dyn` で判定します。

`Shape` trait と、(A) generic版・(B) dyn版の `total_area` を両方実装します。

## Challenge

このLessonの `total_area_generic` と `total_area_dyn` を、
「呼び出し回数が非常に多いホットパスかどうか」「型の種類が事前に分かっているか」
という観点で使い分けるとしたら、それぞれどんな場面が向いているか考えてください。

## Review

- [ ] generic（静的ディスパッチ）とdyn Trait（動的ディスパッチ）の違いを説明できる
- [ ] 「異なる型を混在させたいか」で使い分けを判断できる
- [ ] object safetyの基本的な条件（`Self` を返さない、genericメソッドを持たない）を説明できる

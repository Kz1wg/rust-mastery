# Lesson 09-1: `PhantomData`

## Concept

Lesson 02-2 では、`UserId(String)` と `Email(String)` のように newtype で取り違えを防ぎました。
では、ユーザー・注文・商品……と ID の種類が増えたとき、`UserId`・`OrderId`・`ProductId` を
全部手書きするのでしょうか。**「何の ID か」を型パラメータで表す** `Id<User>`・`Id<Order>` という設計があります。

## Why?

`Id<T>` の `T` は、値としてはどこにも保存されません（中身はただの `u64` です）。
しかし Rust は「使われていない型パラメータ」を許しません。ここで `PhantomData` が必要になります。

## Bad Example

```rust,compile_fail,E0392
struct Id<T> {
    value: u64,
}

fn main() {}
```

## Problem

`T` がどのフィールドにも現れないため、コンパイラは「`T` は使われていない」と拒否します。
これは意地悪ではありません。`T` が型の中でどう使われているか（所有しているのか、参照しているのか）によって、
その型が `Send` かどうか、lifetime の扱い（09-2 の variance）などが決まるため、
**使われ方が分からない型パラメータは扱えない**のです。

## Think

> **問い**: 値としては持たないが、型としては `T` と関係がある——これをどう伝えますか？

<details>
<summary>Solution</summary>

```rust
use std::marker::PhantomData;

struct User;
struct Order;

struct Id<T> {
    value: u64,
    _marker: PhantomData<T>, // サイズ0。「T と関係がある」ことだけを伝える
}

impl<T> Id<T> {
    fn new(value: u64) -> Self {
        Id { value, _marker: PhantomData }
    }
}

fn find_user(id: Id<User>) -> u64 {
    id.value
}

fn main() {
    let user_id: Id<User> = Id::new(1);
    let _order_id: Id<Order> = Id::new(1);
    println!("{}", find_user(user_id));
}
```

`PhantomData<T>` はサイズ 0 の型で、実行時には何も持ちません。
「この構造体は `T` を持っているかのように扱ってほしい」とコンパイラに伝えるだけです。

これで、同じ `u64` でも `Id<User>` と `Id<Order>` は別の型になります。

```rust,compile_fail,E0308
use std::marker::PhantomData;

struct User;
struct Order;

struct Id<T> {
    value: u64,
    _marker: PhantomData<T>,
}

fn find_user(id: Id<User>) -> u64 {
    id.value
}

fn main() {
    let order_id: Id<Order> = Id { value: 1, _marker: PhantomData };
    find_user(order_id); // 注文の ID をユーザー検索に渡してしまった
}
```

newtype を種類ごとに書く代わりに、**1つの `Id<T>` で全ての種類をまかなえます**。

</details>

## 罠: `#[derive]` は `T` にも条件を付ける

`Id` は中身が `u64` だけなので、`Copy` にしたくなります。素直に `derive` すると……

```rust,compile_fail,E0382
use std::marker::PhantomData;

struct User; // User 自体は Copy ではない

#[derive(Clone, Copy)]
struct Id<T> {
    value: u64,
    _marker: PhantomData<T>,
}

fn main() {
    let a: Id<User> = Id { value: 1, _marker: PhantomData };
    let b = a;  // move されてしまう
    let c = a;  // Id<User> は Copy になっていない
    let _ = (b.value, c.value);
}
```

`#[derive(Clone, Copy)]` は、`impl<T: Clone> Clone for Id<T>` のように**すべての型パラメータに同じ trait を要求する**実装を生成します。
`User` が `Copy` でないので、`Id<User>` も `Copy` になりません。`T` は値として持っていないのに、です。

**手で実装すれば、`T` に条件を付けずに済みます。**

```rust
use std::marker::PhantomData;

struct User;

struct Id<T> {
    value: u64,
    _marker: PhantomData<T>,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Id<T> {}

fn main() {
    let a: Id<User> = Id { value: 1, _marker: PhantomData };
    let b = a;
    let c = a; // OK: T に関係なく Copy
    println!("{} {}", b.value, c.value);
}
```

`PhantomData` を使う型で `derive` するときは、「この derive は `T` に不要な条件を付けていないか」を確認してください。

## Deep Dive: `PhantomData<T>` の書き方で性質が変わる

`PhantomData<T>` は「`T` を所有している」ように振る舞うので、`T` が `Send` でなければ型全体も `Send` ではなくなります。

```rust,compile_fail,E0277
use std::marker::PhantomData;
use std::rc::Rc;

struct Id<T>(PhantomData<T>);

fn is_send<X: Send>() {}

fn main() {
    is_send::<Id<Rc<i32>>>(); // Rc は Send ではないので、Id<Rc<i32>> も Send ではない
}
```

ID が単なる番号で、`T` の値を持つわけではないなら、この振る舞いは過剰です。
その場合は `PhantomData<fn() -> T>` と書くと、「`T` を**返す関数**と関係がある」という意味になり、
`T` の `Send` / `Sync` に引きずられなくなります。

```rust
use std::marker::PhantomData;
use std::rc::Rc;

struct Id<T>(PhantomData<fn() -> T>);

fn is_send<X: Send>() {}

fn main() {
    is_send::<Id<Rc<i32>>>(); // OK
}
```

どちらを選ぶかは、「この型は `T` の値を（論理的に）所有しているのか」で決めます。
（`Send` / `Sync` は Chapter 10 で詳しく扱います。）

## Exercise

**`ex034_phantom_data`** — `cargo test -p ex034_phantom_data` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `Id<T>` | `new` / `value` を実装。`Clone` は `T` に条件を付けない手書き実装にする（`Copy` は用意済み） |
| `find_user` | `Id<User>` を受け取り、該当する `User` を探す。`Id<Order>` を渡すとコンパイルエラーになることは `compile_fail` doctest で確認 |

テストには、`User` が `Copy` でないのに `Id<User>` をコピーできることを確かめるものが含まれています。

## Challenge

`Id<T>` の `Clone` / `Copy` を `#[derive(Clone, Copy)]` に戻し、どのテストがコンパイルできなくなるか確かめてください。

## Review

- [ ] 値として持たない型パラメータに `PhantomData` が必要な理由を説明できる
- [ ] `#[derive]` が型パラメータ全体に条件を付けることと、その回避方法を説明できる
- [ ] `PhantomData<T>` と `PhantomData<fn() -> T>` の違いを説明できる

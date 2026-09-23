# Lesson 14-1: マクロが必要な瞬間

## Concept

マクロが必要になるのは、**関数の仕組みでは表現できないこと**をしたいときだけです。
代表的なのは次の3つです。

| 関数ではできないこと | 例 |
| --- | --- |
| **引数の数を自由にする**（可変長引数） | `println!("{} {}", a, b)`、`vec![1, 2, 3]` |
| **関数ではなく「項目」を作る**（struct、impl、関数そのもの） | 複数の型に同じ `impl` をまとめて書く |
| **コードをそのまま文字列にする・受け取る** | `stringify!(a + b)` → `"a + b"`、`assert!(x > 0)` の失敗メッセージ |

逆に言えば、**これら以外の目的でマクロを使っているなら、関数で書けないか先に考えてください**。

## Why?

マクロは、書いた本人以外には中身が読みにくいものです。
展開されるまで何が起きるか分からず、エラーは展開後のコードの位置で報告されることもあります。
関数で済むなら、関数のほうが**型チェックも、エラーメッセージも、IDEの補完も**素直に効きます。

## Bad Example: 関数で書けるものをマクロにした

```rust
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn next(counter: &mut i32) -> i32 {
    *counter += 1;
    *counter
}

fn main() {
    let mut c = 0;
    let r = square!(next(&mut c));
    // 1 * 1 = 1 になるはず……？
    println!("r = {r}, c = {c}"); // r = 2, c = 2
}
```

## Problem

`square!(next(&mut c))` は、展開されると次のコードになります。

```rust,ignore
next(&mut c) * next(&mut c)
```

マクロは**式を「値」としてではなく、「コードの断片」としてそのまま貼り付けます**。
そのため `next` が2回呼ばれ、1回目は 1、2回目は 2 を返して、結果は `1 * 2 = 2` になります。
関数なら、引数は呼び出し前に1回だけ計算されるので、こうはなりません。

## Think

> **問い**: この `square!` は、マクロである必要がありますか？ 上の表のどれかに当てはまりますか？

<details>
<summary>Solution</summary>

当てはまりません。引数は1つで、作るのは値で、コードを文字列にする必要もありません。
**関数で書くべき**です。

```rust
fn square(x: i64) -> i64 {
    x * x
}

fn next(counter: &mut i64) -> i64 {
    *counter += 1;
    *counter
}

fn main() {
    let mut c = 0;
    let r = square(next(&mut c)); // next は1回だけ呼ばれる
    assert_eq!((r, c), (1, 1));
}
```

いろいろな型で使いたければ、ジェネリクスにすれば済みます（Chapter 05）。

```rust
use std::ops::Mul;

fn square<T: Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}

fn main() {
    assert_eq!(square(3), 9);
    assert_eq!(square(1.5), 2.25);
}
```

「いろいろな型で使いたいからマクロにする」は、ほとんどの場合ジェネリクスで解決します。

</details>

## マクロでなければ書けない例

**可変長引数**: 関数は引数の数が決まっていますが、マクロは繰り返しを受け取れます。

```rust
macro_rules! max_of {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => {{
        let a = $x;
        let b = max_of!($($rest),+);
        if a > b { a } else { b }
    }};
}

fn main() {
    assert_eq!(max_of!(3), 3);
    assert_eq!(max_of!(3, 9, 2, 7), 9);
}
```

（ただし、要素の数が実行時に決まるならスライスを受け取る関数で十分です。
`max_of!` が役に立つのは、`max_of!(a, b, c)` のように**書く場所で個数が決まっている**場合です。
それでも `[a, b, c].iter().max()` で済むことも多い、ということは覚えておいてください。）

**複数の型に同じ `impl` を作る**: 関数は `impl` ブロックを作れません。

```rust
trait Describe {
    fn describe() -> &'static str;
}

macro_rules! impl_describe {
    ($($t:ty),*) => {
        $(
            impl Describe for $t {
                fn describe() -> &'static str {
                    stringify!($t)
                }
            }
        )*
    };
}

impl_describe!(i32, u8, String);

fn main() {
    assert_eq!(i32::describe(), "i32");
    assert_eq!(String::describe(), "String");
}
```

`stringify!($t)` は、受け取った型をそのまま文字列にします。これも関数ではできないことです。
標準ライブラリでも、数値型ごとの trait 実装などに、この形のマクロが使われています。

## Deep Dive: マクロの中の変数は外から見えない

```rust,compile_fail,E0425
macro_rules! make_x {
    () => {
        let x = 1;
    };
}

fn main() {
    make_x!();
    println!("{x}"); // マクロの中で作った x は、ここからは見えない
}
```

`macro_rules!` の中で作った変数は、呼び出し側の変数と**名前がぶつからないよう、別物として扱われます**。
これを**衛生性（hygiene）**と言います。うっかり呼び出し側の変数を上書きしてしまう事故を防ぐ仕組みです。
C 言語のマクロ（単純な文字列置換）との大きな違いです。

## Exercise

**`ex053_when_to_use_macros`** — `cargo test -p ex053_when_to_use_macros` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `square`（**関数**） | マクロではなく関数で書く。引数は1回だけ評価される |
| `max_of!`（マクロ） | 可変長の引数から最大値を返す |

テストには、`square(next(&mut c))` で `next` が1回しか呼ばれないことを確かめるものが含まれています。

## Challenge

自分のコードや、よく使うcrateの中から `macro_rules!` を1つ探してください。
それは上の表の3つのどれに当たりますか？ 関数で書き直せませんか？

## Review

- [ ] マクロが必要になる3つの場面（可変長引数、項目の生成、コードの文字列化）を挙げられる
- [ ] 関数で書けるものをマクロにすると、式が2回評価される危険があることを説明できる
- [ ] マクロの衛生性（中の変数が外とぶつからない）を説明できる

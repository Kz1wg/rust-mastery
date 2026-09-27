# Lesson 07-4: 借用するiteratorを返す

## Concept

`&self`（や引数の参照）を借用するイテレータを返す関数では、
**戻り値の型が、借用元と同じだけ生きる**ことを、コンパイラに伝える必要があります。

## Why?

参照を2つ以上受け取る関数で「どちらの参照を借りているか」が曖昧だと、
コンパイラは推測してくれず、明示的なlifetime注釈が必要になります。

## Bad Example: 曖昧な借用元

```rust,compile_fail,E0621
fn merged<'a>(a: &'a [i32], b: &[i32]) -> impl Iterator<Item = &'a i32> {
    a.iter().chain(b.iter())
}

fn main() {}
```

## Problem

戻り値の型 `impl Iterator<Item = &'a i32>` は「`'a` の間だけ生きる参照」を約束していますが、
本体は `a` だけでなく `b` からの参照も返します。`b` には明示的なlifetimeが無く、
コンパイラは「`b` が `'a` と同じだけ生きる」とは推測してくれません
（複数の参照があると、lifetime省略規則は自動適用されません）。

## Think

> **問い**: `a` と `b` の両方から借用した値を返すには、シグネチャをどう直しますか？

<details>
<summary>Hint</summary>

`a` と `b` に**同じlifetime**を明示的に与えるか、それぞれ別のlifetimeを与えたうえで
戻り値の型を「どちらか短い方」に合わせる必要があります。今回は単純化のため、
両方に同じ `'a` を与えます。

</details>

<details>
<summary>Solution</summary>

```rust
fn merged<'a>(a: &'a [i32], b: &'a [i32]) -> impl Iterator<Item = &'a i32> {
    a.iter().chain(b.iter())
}

fn main() {
    let x = vec![1, 2, 3];
    let y = vec![4, 5, 6];
    let all: Vec<&i32> = merged(&x, &y).collect();
    println!("{all:?}");
}
```

`b: &'a [i32]` と明示することで、「`a` と `b` はどちらも、少なくとも `'a` の間は生きている」
という約束になります。戻り値の `impl Iterator<Item = &'a i32>` は、
そのどちらの参照を返しても矛盾しなくなりました。

</details>

## 引数が1つだけなら、省略できる

```rust
fn evens(nums: &[i32]) -> impl Iterator<Item = &i32> {
    nums.iter().filter(|&&n| n % 2 == 0)
}

fn main() {
    let v = vec![1, 2, 3, 4];
    let e: Vec<&i32> = evens(&v).collect();
    println!("{e:?}");
}
```

参照を受け取る引数が1つしかない場合、lifetime省略規則によって、戻り値の型に現れる
`&i32` にはその引数と同じlifetimeが割り当てられます。**そのlifetimeが戻り値の型
（`Item = &i32`）に現れている**ので、`impl Iterator` の中身がその引数を借用してもよい、と判断されます。

07-3 の `doubled` では `Item = i32` にlifetimeが現れないため、`+ '_` を明示する必要がありました
（edition 2021 の場合）。**「戻り値の型にlifetimeが書かれているか」**が分かれ目です。
**曖昧さが生まれるのは、参照が複数あるときだけです。**

## Deep Dive: 08章でさらに詳しく

lifetime注釈が「何を主張しているのか」、省略規則がどこまで効くのかは、
Chapter 08（Lifetimes）で改めて扱います。このLessonで持ち帰ってほしいのは、
**「借用するイテレータを返す関数では、借用元が複数ある場合にコンパイラの助けを借りられない」**
という1点です。

## Exercise

**[`ex029_borrowing_iterators`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex029_borrowing_iterators)** — `cargo test -p ex029_borrowing_iterators` で判定します。

2つのスライスを受け取り、両方から借用したイテレータを返す関数を実装します。

## Challenge

08章に進む前に、自分のコードで `impl Iterator<Item = &T>` を返す関数を探し、
引数がいくつあるか、lifetime注釈が省略されているか明示されているかを確認してください。

## Review

- [ ] 借用するイテレータを返す関数で、lifetime注釈が必要になる理由を説明できる
- [ ] 引数が1つだけなら省略でき、複数あると曖昧になることを説明できる

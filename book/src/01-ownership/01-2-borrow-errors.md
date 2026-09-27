# Lesson 01-2: 借用のエラーを読む

## Concept

借用エラーは「Rustが厳しい」のではなく、**コンパイラが何かを守ろうとして止めている**サインです。
エラーを直す前に、「何を守ろうとしているのか」を言葉にできるようになりましょう。

## Why?

借用エラーを「とにかく `clone` で消す」癖がつくと、設計の問題（所有者が曖昧・参照の寿命が長すぎる）を見逃します。
エラーを読む力は、設計を見直す力に直結します。

## エラーを読む5段階

どの借用エラーでも、次の順に考えます。

1. **何が** borrow（またはmove）されているか
2. **なぜ** その操作ができないのか
3. コンパイラは **何を保証しようとしている** か
4. **どう修正** できるか（候補を複数）
5. **どの修正が設計として望ましい** か

## Bad Example 1: 参照を持ったまま `Vec` に追加する

```rust,compile_fail,E0502
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("{first}");
}
```

## Problem

エラーは「`v` を可変で借用できない。すでに不変で借用されている」という趣旨です。

## Think

> **問い**: 上の5段階に当てはめて答えてください。特に3（コンパイラが守っていること）を、
> `Vec` の内部で `push` が何をするかに着目して説明してください。

<details>
<summary>Hint</summary>

`Vec` は容量が足りなくなると、より大きなヒープ領域を確保して**要素を引っ越し**させます。
そのとき、`first` はどこを指していることになるでしょうか。

</details>

<details>
<summary>Solution</summary>

| 段階 | 答え |
| --- | --- |
| 1. 何がborrowされているか | `let first = &v[0];` により、`v` の要素が不変借用されている |
| 2. なぜできないか | `v.push(4)` は `&mut v` を必要とするが、不変借用（`first`）が `println!` まで生きている |
| 3. 何を保証しようとしているか | **`push` による再確保で、`first` が解放済みメモリを指す（dangling）ことを防ぐ** |
| 4. 修正案 | (a) 値をコピーして取り出す (b) `push` の後で参照を取る (c) 参照の使用を `push` の前に終わらせる |
| 5. 設計として望ましいのは | 下記 |

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    // (a) i32 は Copy なので、値を取り出せば借用は残らない
    let first = v[0];
    v.push(4);
    println!("{first}");

    // (b) push の後に参照を取る
    let first = &v[0];
    println!("{first}");
}
```

(a) は要素が `Copy` のときだけ使えます。`String` などでは (b) か (c) です。
**「参照を長く持ち歩かない」**のが基本方針です。参照を持ったまま変更したくなったら、
「本当にその参照は必要か」「インデックスや値のコピーで足りないか」を先に考えてください。

</details>

## Bad Example 2: 可変参照が2つ同時に存在する

```rust,compile_fail,E0499
fn main() {
    let mut s = String::new();
    let a = &mut s;
    let b = &mut s;
    a.push('x');
    b.push('y');
}
```

### Think

> **問い**: なぜ「同時に2つの可変参照」が危険なのでしょうか。
> 単一スレッドで、しかも `a` と `b` は別々の操作しかしていないのに。

<details>
<summary>Solution</summary>

コンパイラが守っているのは、次の規則です。

> **ある時点で、同じ値に対して「複数の読み手」か「1人の書き手」のどちらかしか存在しない**（aliasing XOR mutability）

`a.push` が文字列を再確保した瞬間、`b` が見ている領域は無効になります。
スレッドの有無に関係なく、**書き換えの最中に他の誰かが同じ場所を覚えている**こと自体が問題です。
（この規則はデータ競合の防止（Chapter 10）にも同じ形で現れます。）

</details>

## Deep Dive: 借用は「最後に使われるまで」

借用の寿命は、スコープの終わりではなく**最後に使われる場所まで**です（NLL: Non-Lexical Lifetimes）。

```rust
fn main() {
    let mut s = String::from("hi");
    let r = &s;
    println!("{r}"); // r の最後の使用
    s.push('!');     // ここでは r の借用はもう終わっているので OK
    println!("{s}");
}
```

00-1 の `drop(x)` の例を思い出してください。`println!("{y}")` を `drop(x)` の**前**に移せば、同じ理由でコンパイルが通ります。
エラーの本質は「順序」であり、「借用と破棄が同時に存在してはならない」ということです。

## Exercise

**[`ex002_borrow_errors`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex002_borrow_errors)** — `cargo test -p ex002_borrow_errors` で判定します。

`src/lib.rs` の先頭コメントに、借用エラーになる元のコード（Bad Example 1 と同じ形）が示されています。
それを**設計を変えて**書き直す2つの関数を実装します。

| 関数 | 修正の観点 |
| --- | --- |
| `first_then_push(v: &mut Vec<i32>, extra: i32) -> i32` | 参照を持ち続けず、`Copy` な値を先に取り出す |
| `bump_two(v: &mut [i32], i: usize, j: usize)` | 2要素を同時に可変で触る。`i == j` の場合を分け、`split_at_mut` を使う |

任意で、それぞれの修正理由を `NOTES.md` に5段階で書いてください
（自己チェック用で、`cargo test` の判定対象ではありません）。

## Challenge

「借用エラーを `clone()` で解消した」コードを、AIに書かせてみてください。
そのコードについて、`clone` が**本当に必要か**、必要でないならどう書き直せるかをレビューしてください。

## Review

- [ ] 借用エラーを5段階で説明できる
- [ ] 「aliasing XOR mutability」を、`Vec::push` の例で説明できる
- [ ] 借用の寿命は「最後に使われるまで」であることを説明できる

# Lesson 01-1: moveは何を守っているのか

## Concept

`let b = a;` で `a` が使えなくなるのはなぜでしょうか。
「Rustのルールだから」ではなく、**何を防ぐための仕組みか**を説明できることが目標です。

## Why?

ヒープを持つ値（`String` や `Vec`）には、必ず「誰がそのヒープを解放するか」という責任者が必要です。
責任者が0人なら**メモリリーク**、2人なら**二重解放**になります。
所有権は、この責任者を常に1人にするための仕組みです。

## Bad Example

次のコードは、多くの言語では問題なく動きます。Rustではどうでしょうか。

```rust,compile_fail,E0382
fn main() {
    let a = String::from("hello");
    let b = a;
    println!("{a} {b}");
}
```

## Problem

`String` は、スタック上に3つの値（ヒープへのポインタ・長さ・容量）を持ち、文字列本体はヒープにあります。

```text
let a = String::from("hello");

  スタック                  ヒープ
  a: [ptr | len=5 | cap=5] ──► "hello"

let b = a;   ← もし「中身をそのままコピー」するだけだったら？

  a: [ptr | len=5 | cap=5] ──┐
                             ├──► "hello"
  b: [ptr | len=5 | cap=5] ──┘

  スコープ終了時、a と b の両方が「自分がヒープの持ち主」として解放する → 二重解放
```

## Think

> **問い**: このコードを、意図どおりに動くように直す方法を**3つ以上**挙げてください。
> それぞれの方法は、メモリ上で何が起きるか（ヒープの確保・コピーの有無）が違います。

<details>
<summary>Hint 1</summary>

`a` と `b` の両方が**同じ文字列を読むだけ**でよいなら、ヒープを2つ持つ必要はあるでしょうか。

</details>

<details>
<summary>Hint 2</summary>

「`b` に渡した後、`a` はもう要らない」という設計なら、どう書けますか。

</details>

<details>
<summary>Solution</summary>

**案1: 参照を使う（借用）** — ヒープはそのまま。コピーなし。

```rust
fn main() {
    let a = String::from("hello");
    let b = &a;
    println!("{a} {b}");
}
```

**案2: `clone` する** — ヒープを新しく確保し、内容をコピーする。所有者が2人になるが、それぞれが自分のヒープを持つので安全。

```rust
fn main() {
    let a = String::from("hello");
    let b = a.clone();
    println!("{a} {b}");
}
```

**案3: `a` を使うのをやめる** — 所有権を `b` に渡し、`a` は以降使わない。

```rust
fn main() {
    let a = String::from("hello");
    let b = a;
    println!("{b}");
}
```

| 案 | ヒープ確保 | 所有者 | 向いている状況 |
| --- | --- | --- | --- |
| 借用 | なし | `a` のみ | 読むだけでよい。`b` は `a` より長生きしない |
| `clone` | あり（コピーのコスト） | `a` と `b` が別々に | 両方が独立して変更・保持したい |
| move | なし | `b` のみ | 渡した後は元の変数が不要 |

**設計上の望ましさ**: まず借用を検討し、それで足りなければmove、最後に `clone`。
`clone` は「コピーのコストを払う」という**明示的な意思表示**であり、なんとなく付けるものではありません。
（`clone` が悪いわけではありません。所有者が本当に2人必要なら、`clone` が正解です。）

</details>

## Deep Dive: なぜ `i32` はmoveされないのか

```rust
fn main() {
    let a = 5;
    let b = a;
    println!("{a} {b}"); // OK: i32 は Copy
}
```

`i32` はスタックだけで完結し、解放すべきヒープがありません。
二重解放の問題が起きないので、ビット単位のコピーで済み、`Copy` traitが実装されています。

では「ヒープを持たない型なら何でも `Copy` にできる」のでしょうか。
`Drop`（解放時に何かをする）を実装した型は、`Copy` にできません。

```rust,compile_fail,E0184
#[derive(Clone, Copy)]
struct Handle(i32);

impl Drop for Handle {
    fn drop(&mut self) {
        // 例: ファイルディスクリプタを閉じる
    }
}
```

コピーされた分だけ `drop` が呼ばれると、同じリソースを複数回閉じてしまうからです。
`Copy` は「コピーしても意味が変わらない」ことの宣言です。

## Exercise

**`ex001_move_semantics`** — `cargo test -p ex001_move_semantics` で判定します。

| 関数 | 仕様 |
| --- | --- |
| `fn longest_owned(a: String, b: String) -> String` | 長い方を返す。同じ長さなら `a` |
| `fn longest_ref<'a>(a: &'a str, b: &'a str) -> &'a str` | 同上。ただし新しい確保をしない |

判定テストでは、呼び出し元が引数を再利用できること（`longest_ref` の場合）を確認します。
それぞれ「呼び出し側から見て何が違うか」を説明してください。

## Challenge

`fn consume(s: String)` と `fn borrow(s: &str)` を呼び出す側のコードを書き、
**呼び出した後に元の変数が使えるか**を確かめてください。使えない場合、それは呼び出し側にとって「制約」でしょうか、「保証」でしょうか。

## Review

- [ ] moveが「二重解放を防ぐ」仕組みであることを、図を描いて説明できる
- [ ] 借用・`clone`・moveの3つを、コストと意図の違いで使い分けられる
- [ ] `Copy` と `Drop` が両立しない理由を説明できる

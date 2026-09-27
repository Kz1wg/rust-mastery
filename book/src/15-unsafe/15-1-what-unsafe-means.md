# Lesson 15-1: unsafe は何を宣言しているのか

## Concept

Rust のコンパイラは、所有権・借用・型のチェックによって「メモリを壊すバグ」を防いでいます。
しかし世の中には、**正しいのに、コンパイラには正しいと確認できないコード**があります。

`unsafe` は、そういうコードのために「ここは私が責任を持って確認した」と書く場所です。

## unsafe で**できるようになる**こと

`unsafe` ブロックの中でだけ許されるのは、次の5つだけです。

| できること | 例 |
| --- | --- |
| 生ポインタ（`*const T` / `*mut T`）の参照外し | `*ptr` で中身を読み書きする |
| `unsafe fn` の呼び出し | `std::slice::from_raw_parts` など |
| `unsafe trait` の実装 | `unsafe impl Send for MyType {}` |
| 可変の `static` 変数へのアクセス | `static mut COUNTER: u32` |
| `union` のフィールドの読み出し | C との互換用 |

## unsafe でも**できるようにならない**こと

ここが大事です。`unsafe` ブロックの中でも、**借用チェックや型チェックは普段どおり働きます**。

```rust,compile_fail,E0499
fn main() {
    let mut x = 5;
    unsafe {
        let a = &mut x;
        let b = &mut x; // unsafe の中でも、可変借用の二重取りは許されない
        *a += 1;
        *b += 1;
    }
}
```

`unsafe` は「チェックを全部オフにする」スイッチではなく、**上の5つの操作を追加で許可する**だけです。

## Bad Example: unsafe が要るのは「たどる」とき

```rust,compile_fail,E0133
fn main() {
    let x = 5;
    let p = &x as *const i32; // 生ポインタを「作る」のは安全
    println!("{}", *p);       // 生ポインタを「たどる」のは unsafe が必要
}
```

## Problem

生ポインタは、**どこを指しているか保証がないポインタ**です。

- 解放済みのメモリを指しているかもしれない
- 適当な数値から作られたかもしれない（`0x1234 as *const i32`）
- 他の誰かが同時に書き換えているかもしれない

作るだけなら何も起きませんが、**たどった瞬間に**これらの問題が表に出ます。
だから Rust は「作るのは自由、たどるなら unsafe」という線を引いています。

## Think

> **問い**: 上のコードを unsafe ブロックで直すとき、「このポインタをたどっても大丈夫」と言える根拠は何ですか？
> それをどこに書き残せばよいでしょうか。

<details>
<summary>Solution</summary>

```rust
fn main() {
    let x = 5;
    let p = &x as *const i32;

    // SAFETY: p は直前に &x から作ったもので、x はこのスコープの間ずっと生きている。
    // x は不変なので、同時に書き換えられることもない。
    let value = unsafe { *p };

    assert_eq!(value, 5);
}
```

根拠は `// SAFETY:` というコメントで、**unsafe ブロックのすぐ上に**書くのが Rust の慣習です。
（clippy の `undocumented_unsafe_blocks` という lint を有効にすると、このコメントが無い unsafe を警告してくれます。）

このコメントは、コードを読む人（未来の自分や、レビューするAI）への**証明の記録**です。
「なぜ安全なのか」を書けないなら、そのコードは本当に安全なのか疑うべきです。

</details>

## `unsafe fn` と `# Safety`

関数そのものを `unsafe fn` にすると、「**呼ぶ側が**条件を守る責任を持つ」という意味になります。
その条件は、ドキュメントの `# Safety` 節に書きます。

```rust
/// ポインタが指す値を読む。
///
/// # Safety
///
/// - `p` は null であってはならない
/// - `p` は、有効な `i32` を指していなければならない
pub unsafe fn read_value(p: *const i32) -> i32 {
    // SAFETY: 呼び出し側が上の条件を守っている前提。
    unsafe { *p }
}

fn main() {
    let x = 42;
    // SAFETY: &x から作ったポインタで、x は生きている。
    let v = unsafe { read_value(&x) };
    assert_eq!(v, 42);
}
```

clippy は、`pub unsafe fn` に `# Safety` 節が無いと警告します（`missing_safety_doc`）。
また、**生ポインタを受け取ってたどる `pub fn` を、`unsafe` にせずに公開する**とエラーにします（`not_unsafe_ptr_arg_deref`）。
呼ぶ側が安全性の条件を知らないまま使えてしまうからです。

## Deep Dive: 「unsafe な関数の中」も区切る

上の例では、`unsafe fn` の中でもわざわざ `unsafe { *p }` と書きました。
Rust 2024 edition からは、**`unsafe fn` の中でも、危険な操作ごとに `unsafe` ブロックを書く**ことが推奨されます
（`unsafe_op_in_unsafe_fn` という lint が既定で警告になります）。

理由は、`unsafe fn` 全体が unsafe だと、**どの行が本当に危険なのか分からなくなる**からです。
危険な操作を小さなブロックに区切り、それぞれに `// SAFETY:` を書くほうが、確認漏れを防げます。

## Exercise

**[`ex056_unsafe_basics`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex056_unsafe_basics)** — `cargo test -p ex056_unsafe_basics` で判定します。

| 関数 | 仕様 |
| --- | --- |
| `first_via_ptr(slice)` | スライスの先頭を、生ポインタ経由で読んで返す（空なら `None`）。`// SAFETY:` を書くこと |
| `read_value(p)` | `unsafe fn`。`# Safety` 節で条件を書く |
| `swap_values(a, b)` | 2つの `&mut i32` の中身を入れ替える。**unsafe を使わずに書ける**ので使わないこと |

最後の課題は引っかけです。`std::mem::swap` で安全に書けるものを、unsafe で書く理由はありません。

## Challenge

`first_via_ptr` の `// SAFETY:` コメントに書いた条件のうち、1つでも崩れるとどうなるでしょうか。
例えば「空のスライスのときもポインタをたどる」実装にすると、何が起きうるか考えてください。

## Review

- [ ] unsafe で追加で許可される5つの操作を挙げられる
- [ ] unsafe の中でも借用チェックは働くことを説明できる
- [ ] 生ポインタは「作るのは安全、たどるのは unsafe」である理由を説明できる
- [ ] `// SAFETY:` と `# Safety` を書き分けられる

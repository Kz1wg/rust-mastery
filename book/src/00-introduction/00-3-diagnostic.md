# Lesson 00-3: 診断問題

## Concept

この教材は、Rust の文法を一通り知っている人を対象にしています。
始める前に、前提となる知識があるかを確かめましょう。

問題は 8 問です。**答えを開く前に**、自分の答えを決めてください。
紙やメモに書いておくと、後で答え合わせがしやすくなります。

「コンパイルは通るか」という問題では、コンパイラを使わずに考えてください。
この教材で身につけたいのは、コンパイラに言われる**前に**、何が問題かに気づく力だからです。

## 問題1: 所有権

次のコードは、コンパイルが通るでしょうか。通らないなら、理由は何でしょうか。

```rust,compile_fail,E0382
fn main() {
    let s = String::from("hello");
    let t = s;
    println!("{s} {t}");
}
```

<details>
<summary>答え</summary>

**通りません**（E0382: borrow of moved value）。

`let t = s;` で、`String` の所有権が `s` から `t` に移ります（move）。
移った後の `s` は使えません。
`String` は中身をヒープに持っているので、単純にコピーすると、2つの変数が同じメモリを指すことになります。
両方が片付けのときにそのメモリを解放しようとすると、二重解放になります。move はそれを防ぐ仕組みです。

両方使いたいなら `let t = s.clone();`（中身ごと複製する）にします。

</details>

## 問題2: 借用

次のコードは、コンパイルが通るでしょうか。

```rust,compile_fail,E0502
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("{first}");
}
```

<details>
<summary>答え</summary>

**通りません**（E0502: cannot borrow `v` as mutable because it is also borrowed as immutable）。

`first` は `v` の中身を借りています。
その後の `v.push(4)` は、`Vec` の容量が足りなければ、中身を別の場所に引っ越します。
すると `first` は、もう使われていない古い場所を指すことになります。

「読むための借用（`&`）が生きている間は、書き換えるための借用（`&mut`）を作れない」という規則が、これを防いでいます。
`println!` を `push` の前に移せば、`first` の借用はそこで終わるので、通ります。

</details>

## 問題3: `Option` と iterator

次のコードの `total` はいくつでしょうか。

```rust
fn main() {
    let inputs = ["10", "x", "25", ""];
    let total: u32 = inputs.iter().filter_map(|s| s.parse::<u32>().ok()).sum();
    println!("{total}");
#   assert_eq!(total, 35);
}
```

<details>
<summary>答え</summary>

**35** です。

`s.parse::<u32>()` は `Result` を返し、`.ok()` はそれを `Option` にします（成功なら `Some`、失敗なら `None`）。
`filter_map` は、`Some` の中身だけを残します。
`"x"` と `""` は数として読めないので捨てられ、10 + 25 = 35 になります。

ただし、「読めない入力を黙って捨てる」のが正しいかは、場面によります。
この問いは 00-4 で扱います。

</details>

## 問題4: `?` 演算子

次の関数に `"42"` と `"4x"` を渡すと、それぞれ何が返るでしょうか。

```rust
use std::num::ParseIntError;

fn double(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

fn main() {
    println!("{:?}", double("42"));
    println!("{:?}", double("4x"));
#   assert_eq!(double("42"), Ok(84));
#   assert!(double("4x").is_err());
}
```

<details>
<summary>答え</summary>

`double("42")` は `Ok(84)`、`double("4x")` は `Err(…)`（`ParseIntError`）です。

`?` は、`Result` が `Ok` なら中身を取り出して続け、`Err` なら**その場で関数から `Err` を返します**。
`"4x"` の場合、`s.parse()?` の時点で関数を抜けるので、`Ok(n * 2)` には届きません。

</details>

## 問題5: `match`

次のコードは、コンパイルが通るでしょうか。

```rust,compile_fail,E0004
enum Signal {
    Red,
    Yellow,
    Green,
}

fn action(s: Signal) -> &'static str {
    match s {
        Signal::Red => "止まる",
        Signal::Green => "進む",
    }
}

fn main() {
    println!("{}", action(Signal::Yellow));
}
```

<details>
<summary>答え</summary>

**通りません**（E0004: non-exhaustive patterns: `Signal::Yellow` not covered）。

`match` は、全ての可能性を扱わなければなりません（網羅性）。
`Yellow` の場合に何を返すかが決まっていないので、コンパイラが止めます。

これは、あとで `enum` に variant を足したときにも効きます。
足した variant を扱い忘れた `match` が、全てコンパイルエラーとして見つかります。
この性質は、Chapter 03 で状態を設計するときの土台になります。

</details>

## 問題6: 引数の型

次の2つの呼び出しが、**両方とも**コンパイルできるようにしたいとします。
`greet` の引数の型は、何にすればよいでしょうか。

```rust,ignore
let name = String::from("Alice");
greet(&name);
greet("Bob");
```

<details>
<summary>答え</summary>

`&str` です。

```rust
fn greet(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let name = String::from("Alice");
    greet(&name); // &String は &str に自動で変換される
    greet("Bob"); // 文字列リテラルは、もともと &str
}
```

`&String` にすると、`"Bob"` を渡せません。`String` にすると、呼ぶ側が所有権を手放すか、`to_string()` でコピーを作る必要があります。
「読むだけなら、一番広く受け取れる借用の型にする」という考え方は、Lesson 01-3 と 01-4 で詳しく扱います。

</details>

## 問題7: trait の実装

次の `Point` を `println!("{p}")` で `(1, 2)` と表示できるようにしてください。

```rust,ignore
struct Point {
    x: i32,
    y: i32,
}
```

<details>
<summary>答え</summary>

`std::fmt::Display` を実装します。

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{p}");
#   assert_eq!(p.to_string(), "(1, 2)");
}
```

`{}` は `Display`、`{:?}` は `Debug` を使います。
`Display` を実装すると、`to_string()` も使えるようになります。

</details>

## 問題8: Cargo

次のうち、**正しくない**ものはどれでしょうか。

1. `src/lib.rs` があるとライブラリの crate、`src/main.rs` があるとバイナリの crate になる。両方あってもよい
2. `tests/` ディレクトリのテストは、crate の**公開された**関数だけを呼べる
3. `cargo test` は、`///` のドキュメントコメントに書いたコード例も実行する
4. `[dependencies]` に書いた crate は、`tests/` のテストからは使えない

<details>
<summary>答え</summary>

**4** が正しくありません。

`[dependencies]` の crate は、ライブラリ本体からもテストからも使えます。
逆に、`[dev-dependencies]` に書いた crate は、**テストとベンチマークからだけ**使えます（P6 で tokio のテスト用の機能をここに書いています）。

1〜3 は正しい内容です。2 と 3 は、Chapter 12 と 13 で詳しく扱います。

</details>

## 結果の目安

| 正解の数 | おすすめ |
| --- | --- |
| 7〜8 問 | このまま Chapter 01 へ進んでください |
| 4〜6 問 | 進めて大丈夫です。間違えた問題のテーマを、下の表で復習しておくと安心です |
| 0〜3 問 | 先に『The Rust Programming Language』（日本語訳「[The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/)」）の該当する章を読むことをおすすめします |

| 問題 | テーマ | 『The Rust Programming Language』の章 |
| --- | --- | --- |
| 1, 2, 6 | 所有権と借用 | 4 章 |
| 3 | iterator とクロージャ | 13 章 |
| 4 | エラー処理 | 9 章 |
| 5 | enum と match | 6 章 |
| 7 | trait | 10 章 |
| 8 | Cargo とテスト | 7 章、11 章、14 章 |

## Review

- [ ] 8 問のうち、間違えた問題の理由を説明できる
- [ ] 自分がどの章から始めるべきか決めた

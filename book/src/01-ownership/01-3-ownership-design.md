# Lesson 01-3: 所有権で設計する

## Concept

関数の引数を `T` にするか `&T` にするか `&mut T` にするかは、**その関数が値に対して何をするか**の宣言です。

## Why?

シグネチャは、呼び出し側への約束です。
「読むだけ」なのに所有権を要求すると、呼び出し側は不要な `clone` を強いられます。
逆に「保存する」のに参照で受けると、寿命の問題が呼び出し側に漏れます。

## Bad Example

```rust,compile_fail,E0382
fn len_of(s: String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("alice");
    let n = len_of(name);
    println!("{name} has {n} bytes");
}
```

## Problem

`len_of` は長さを読むだけなのに、所有権を奪ってしまっています。
呼び出し側は `len_of(name.clone())` と書くか、`len_of` を諦めることになります。

## Think

> **問い**: 次の3つの関数を、それぞれ「何をするための引数の受け方か」で説明してください。
> そして、どういうときに所有権 `T` を要求するのが**正当**か、考えてください。

```rust
struct User {
    name: String,
}

fn print_name(name: &str) { println!("{name}"); }                // (a)
fn rename(user: &mut User, name: String) { user.name = name; }   // (b)
fn into_name(user: User) -> String { user.name }                 // (c)
```

<details>
<summary>Hint</summary>

各関数が、呼び出しの**後**に呼び出し側の変数に何を期待できるかを考えてください。
(b) では、引数が2つありますが、受け方が違います。なぜでしょうか。

</details>

<details>
<summary>Solution</summary>

| 受け方 | 宣言していること | 使うとき |
| --- | --- | --- |
| `&T` / `&str` | 「読むだけ。返したら元のまま」 | 検査・表示・計算 |
| `&mut T` | 「その場で変更する。所有権は返す」 | 部分的な更新 |
| `T` | 「**受け取って、持つ／消費する**」 | 構造体に保存、別スレッドへ送る、変換して返す |

(b) で `name: String` を値で受けているのは、`user.name` に**保存する**からです。
もし `name: &str` で受けると、内部で `to_string()` する必要があり、呼び出し側が `String` を持っていた場合に無駄なコピーが発生します。

**所有権 `T` を要求すべき正当な理由**は、その関数の中に「値を保持・移動・消費する処理」があることです。
処理の中でそれが起きないなら、`&T` にするのが基本です。

</details>

## Solution: 修正版

```rust
fn len_of(s: &str) -> usize {
    s.len()
}

fn main() {
    let name = String::from("alice");
    let n = len_of(&name);
    println!("{name} has {n} bytes");
}
```

引数を `&String` ではなく `&str` にしている点にも注意してください。
`&String` は `&str` に自動で変換（deref coercion）されますが、逆はできません。
`&str` で受ければ、`String` からも文字列リテラルからも呼べます。

```rust
fn len_of(s: &str) -> usize { s.len() }

fn main() {
    let owned = String::from("alice");
    println!("{}", len_of(&owned));   // &String → &str
    println!("{}", len_of("bob"));    // &'static str
}
```

## Deep Dive: 「保存する」ときの受け方の選択肢

構造体に保存する場合、`String` で受けるのが素直ですが、呼び出し側の柔軟性を上げる書き方もあります。

```rust
struct User {
    name: String,
}

impl User {
    // (1) String を直接受ける
    fn new(name: String) -> Self { Self { name } }

    // (2) &str を受けて内部で確保する
    fn from_ref(name: &str) -> Self { Self { name: name.to_string() } }

    // (3) Into<String> を受ける
    fn with_into(name: impl Into<String>) -> Self { Self { name: name.into() } }
}

fn main() {
    let a = User::new(String::from("a"));      // (1) リテラルは to_string() が必要
    let b = User::from_ref("b");               // (2) String からだと余計なコピー
    let c = User::with_into("c");              // (3) どちらでも書ける
    let d = User::with_into(String::from("d"));
    let _ = (a, b, c, d);
}
```

| 案 | 呼び出しの書きやすさ | 余計なコピー | 読みやすさ |
| --- | --- | --- | --- |
| (1) `String` | リテラルだと手間 | なし（`String`を渡す場合） | 最も明快。確保のコストが呼び出し側に見える |
| (2) `&str` | 簡単 | `String` を持っている場合は無駄が出る | 簡単だがコストが隠れる |
| (3) `impl Into<String>` | 最も柔軟 | なし | ジェネリクスの分、少し複雑。エラーメッセージも読みにくい |

**どれが正解ではありません。** 公開APIで呼び出しの多様性が重要なら (3)、
内部用や単純さを優先するなら (1)。**「確保のコストを誰に見せたいか」**が判断軸です。

## Exercise

**`ex003_ownership_design`** — `cargo test -p ex003_ownership_design` で判定します。

この演習では、シグネチャはあらかじめ決めてあります（`cargo test` で自動判定するため）。
実装する前に、**なぜこの受け方なのか**を本Lessonの表と照らし合わせて説明してください。

| 関数 | 用意してあるシグネチャ | テストが確認すること |
| --- | --- | --- |
| `count_words` | `fn count_words(text: &str) -> usize` | 文字列リテラル・`&String`・`as_str()` のどれからでも呼べる |
| `Config::new` | `fn new(name: impl Into<String>) -> Self` | リテラルからも `String` からも構築できる |

「シグネチャを自分で決める」体験は、下の Challenge で行ってください。

## Challenge

`fn count_words(text: &str) -> usize` を、`&String` 版と比べて、呼び出せるパターンがどう違うか表にしてください。
さらに、`count_words` を `String` で受ける版・`&String` で受ける版を自分で書いてテストを走らせ、
どの呼び出しがコンパイルできなくなるかを確かめてください。

## Review

- [ ] `T` / `&T` / `&mut T` が呼び出し側へ何を約束するか説明できる
- [ ] 引数を `&String` ではなく `&str` にする理由を説明できる
- [ ] 「保存するときの受け方」の選択肢とトレードオフを説明できる

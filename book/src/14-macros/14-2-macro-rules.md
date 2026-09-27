# Lesson 14-2: declarative macro

## Concept

`macro_rules!` で作るマクロは、**「この形の入力が来たら、このコードに置き換える」という規則の集まり**です。
`match` 式に似ていますが、値ではなく**コードの形**にマッチします。

```rust
macro_rules! greet {
    // 規則1: 引数なし
    () => {
        "こんにちは"
    };
    // 規則2: 式を1つ受け取る
    ($name:expr) => {
        format!("こんにちは、{}さん", $name)
    };
}

fn main() {
    assert_eq!(greet!(), "こんにちは");
    assert_eq!(greet!("田中"), "こんにちは、田中さん");
}
```

上から順に規則を試し、最初に合ったものが使われます。

## 入力の「種類」を指定する

`$name:expr` の `expr` は、**どんな種類のコードを受け取るか**の指定です（フラグメント指定子と言います）。

| 指定子 | 受け取るもの | 例 |
| --- | --- | --- |
| `expr` | 式 | `1 + 2`、`foo()`、`"text"` |
| `ty` | 型 | `i32`、`Vec<String>` |
| `ident` | 名前（識別子） | `my_var`、`Point` |
| `tt` | トークン1つ（何でも） | 上級者向け。自由度が高い分、扱いが難しい |
| `literal` | リテラル | `42`、`"abc"` |

## 繰り返し

`$( ... ),*` は「`...` をカンマ区切りで0回以上」、`$( ... ),+` は「1回以上」です。
受け取った側でも同じ書き方で、繰り返しを展開できます。

```rust
macro_rules! sum {
    ($($x:expr),*) => {
        0 $(+ $x)*   // 1, 2, 3 を受け取ると 0 + 1 + 2 + 3 に展開される
    };
}

fn main() {
    assert_eq!(sum!(), 0);
    assert_eq!(sum!(1, 2, 3), 6);
}
```

## Bad Example: 規則に合わない入力

```rust,compile_fail
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    let _ = square!(1, 2); // 引数は1つのはず
}
```

エラーは `no rules expected the token ','`（どの規則も `,` を想定していない）となります。
関数なら「引数は1つですが2つ渡されました」と言ってくれるところですが、
マクロは**形が合わない**としか言えません。規則が増えるほど、どこが合わなかったのか分かりにくくなります。

## Think

> **問い**: `HashMap` を `{"a" => 1, "b" => 2}` のような書き方で作るマクロ `hashmap!` を考えます。
> どんな規則を書けばよいでしょうか。

<details>
<summary>Hint</summary>

「式 `=>` 式」の組を、カンマ区切りで繰り返します。`$( $key:expr => $value:expr ),*` という形です。

</details>

<details>
<summary>Solution</summary>

```rust
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

fn main() {
    let m = hashmap! {
        "a" => 1,
        "b" => 2,  // 最後のカンマがあっても良い（$(,)? のおかげ）
    };
    assert_eq!(m["a"], 1);
    assert_eq!(m.len(), 2);
}
```

いくつかの細かい工夫があります。

| 書き方 | 理由 |
| --- | --- |
| `{{ ... }}`（波かっこ2重） | 外側はマクロの規則の区切り、内側は**ブロック式**。`let` を含む複数の文を、1つの式として返すため |
| `::std::collections::HashMap` | 呼び出し側で `HashMap` が `use` されていなくても動くよう、**完全なパス**で書く |
| `$(,)?` | 最後のカンマを「0回か1回」許す。Rust のコードでよく見る書き方に合わせる |

</details>

## Deep Dive: 他のcrateから使えるマクロにする

ライブラリでマクロを公開するときは、2つのことに気を付けます。

```rust,ignore
#[macro_export]           // crate の外から使えるようにする（crate のルートに置かれる）
macro_rules! max_of {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => {{
        let a = $x;
        let b = $crate::max_of!($($rest),+);  // 自分自身を呼ぶときは $crate:: を付ける
        if a > b { a } else { b }
    }};
}
```

- **`#[macro_export]`** を付けると、他の crate から `use my_crate::max_of;` で使えます。
- マクロの中から、自分の crate の関数や自分自身を呼ぶときは **`$crate::`** を付けます。
  `$crate` は「このマクロを定義した crate」に置き換わるので、利用者の側でどんな名前で読み込まれていても正しく参照できます。

## Exercise

**[`ex054_macro_rules`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex054_macro_rules)** — `cargo test -p ex054_macro_rules` で判定します。

| マクロ | 仕様 |
| --- | --- |
| `hashmap!` | `key => value` の組から `HashMap` を作る。最後のカンマを許す |
| `impl_unit!` | 単位の名前と記号の組から、newtype と `Display` 実装をまとめて作る |

どちらも `#[macro_export]` 付きで、テスト（別crate）から使えることを確かめています。

## Challenge

`hashmap!` の規則から `$(,)?` を消すと、どんな呼び出しがエラーになりますか。試してください。

## Review

- [ ] `macro_rules!` が「形にマッチする規則の集まり」だと説明できる
- [ ] `expr` / `ty` / `ident` などの指定子を使い分けられる
- [ ] `$( ... ),*` による繰り返しを書ける
- [ ] `#[macro_export]` と `$crate` の役割を説明できる

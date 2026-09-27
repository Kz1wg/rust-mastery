# Lesson 14-3: procedural macro 入門

## Concept

`#[derive(Debug)]` や `#[derive(Clone)]` は、procedural macro（手続き的マクロ）の一種です。
procedural macro は、仕組みとしては**とても単純**です。

> **Rust のコード（トークンの列）を受け取って、Rust のコードを返す関数。**

この関数は、コンパイラがあなたのコードをコンパイルする**途中で**呼び出します。

## 3種類の procedural macro

| 種類 | 使い方 | 例 | できること |
| --- | --- | --- | --- |
| derive マクロ | `#[derive(Name)]` | `#[derive(Debug, Serialize)]` | struct / enum を見て、**追加の実装を生成する**（元のコードは変えない） |
| attribute マクロ | `#[name(...)]` を項目に付ける | `#[tokio::main]`、`#[test]` 風のもの | 付けた項目を**丸ごと書き換える** |
| function-like マクロ | `name!(...)` | `sql!(SELECT ...)` のようなもの | `macro_rules!` と同じ見た目だが、中身を自由に解析できる |

このLessonでは、最もよく使われる **derive マクロ**を自分で1つ作ります。

## Why?

serde の `#[derive(Serialize)]` や、tokio の `#[tokio::main]` を使っていると、
「これは何をしているのか」が見えません。1つ自分で作ると、
**「struct の定義を読んで、impl を書き足しているだけ」**だと分かり、魔法ではなくなります。

## 作るもの

```rust,ignore
use describe_derive::Describe;

#[derive(Describe)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    assert_eq!(User::type_name(), "User");
    assert_eq!(User::field_names(), vec!["name", "age"]);
}
```

`#[derive(Describe)]` を付けると、型の名前とフィールド名の一覧を返すメソッドが自動で生えます。

## 仕組み

procedural macro は、**専用の crate** に書く必要があります（`Cargo.toml` で `proc-macro = true` にした crate）。
コンパイラがその crate を先にビルドし、あなたのコードをコンパイルする途中で呼び出すためです。

```toml
# describe_derive/Cargo.toml
[lib]
proc-macro = true
```

```rust,ignore
// describe_derive/src/lib.rs
use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    // 1. input は struct の定義そのもの（トークンの列）
    //    struct User { name : String , age : u32 }

    // 2. トークンを読んで、型の名前を探す
    let mut name = None;
    let mut tokens = input.into_iter();
    while let Some(token) = tokens.next() {
        if let TokenTree::Ident(ident) = &token {
            if ident.to_string() == "struct" {
                if let Some(TokenTree::Ident(n)) = tokens.next() {
                    name = Some(n.to_string());
                }
                break;
            }
        }
    }
    let name = name.expect("Describe は struct にだけ使えます");

    // 3. 生成したいコードを文字列で組み立て、トークンの列に変換して返す
    let code = format!(
        "impl {name} {{ pub fn type_name() -> &'static str {{ \"{name}\" }} }}"
    );
    code.parse().unwrap()
}
```

やっていることは3つだけです。

1. **受け取る**: struct の定義が、トークン（`struct`、`User`、`{ ... }` などの部品）の列として渡される
2. **読む**: トークンを順に見て、必要な情報（型の名前、フィールド名）を取り出す
3. **返す**: 生成したいコードを作り、トークンの列として返す。コンパイラがそれを元のコードの後ろに付け足す

`format!` の中の `{{` と `}}` は、文字としての `{` と `}` を出力するための書き方です
（`format!` では `{}` が埋め込みの印なので、波かっこそのものは2重にします）。

## Think

> **問い**: この derive マクロは、生成するコードを文字列で組み立てています。
> この方法の弱点は何でしょうか。実務のマクロはどうしているでしょうか。

<details>
<summary>Solution</summary>

**弱点**:

- 生成したコードに文法の誤りがあっても、`parse()` するまで分からない（エラーの場所も分かりにくい）
- 入力を読む処理（トークンを1つずつ見る）が、ジェネリクスや属性などの複雑な定義に対応しにくい
  （例えば `struct Wrapper<T> { ... }` の `<T>` を無視して `impl Wrapper { ... }` を作ると、コンパイルエラーになる）

**実務では**、次の2つの crate を使うのが定番です。

| crate | 役割 |
| --- | --- |
| `syn` | 入力のトークン列を、「struct の名前」「フィールドの一覧」「ジェネリクス」などを持った**構造化されたデータ**に変換する |
| `quote` | `quote! { impl #name { ... } }` のように、**Rust のコードに近い見た目**で出力を組み立てる |

この教材では依存crateを増やさない方針なので、標準の `proc_macro` だけで作りました。
そのぶん、**マクロが本当は何を受け取り、何を返しているのか**が、そのまま見えています。
実務で書くときは `syn` と `quote` を使ってください。

</details>

## Deep Dive: マクロを書く前に

procedural macro は、使う側から見るととても便利ですが、書く側・保守する側のコストは高めです。

- 専用の crate が必要になる（workspace の構成が増える）
- マクロの中のバグは、利用者のコードのコンパイルエラーとして現れ、原因が追いにくい
- コンパイル時間が伸びる（特に `syn` は重い）

「同じ `impl` を何十もの型に書いている」「ボイラープレートが利用者にとって大きな負担になっている」
といった、**はっきりした理由があるときだけ**作るのがよいでしょう。
多くの場合は、trait のデフォルト実装や、14-2 の `macro_rules!` で足ります。

## Exercise

**[`ex055_derive_macro`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex055_derive_macro)** — `cd exercises/ex055_derive_macro && cargo test` で判定します
（derive マクロ用の crate と、それを使う crate からなる入れ子の workspace です）。

| crate | 役割 |
| --- | --- |
| `describe_derive` | derive マクロ本体。**型の名前とフィールド名を読む部分は用意済み**。生成するコードを組み立てる部分を実装する |
| `describe_user` | `#[derive(Describe)]` を使う側。テストはここにある |

生成する2つのメソッド:

| メソッド | 返すもの |
| --- | --- |
| `type_name() -> &'static str` | 型の名前 |
| `field_names() -> Vec<&'static str>` | フィールド名の一覧（定義の順） |

## Challenge

`cargo expand`（別途インストールが必要なツール）を使うと、マクロが展開された後のコードを見られます。
インストールできる環境なら、`#[derive(Debug)]` を付けた struct を展開して、標準の derive が何を生成しているか覗いてみてください。

## Review

- [ ] procedural macro が「トークンの列を受け取ってトークンの列を返す関数」だと説明できる
- [ ] derive / attribute / function-like の3種類の違いを説明できる
- [ ] procedural macro が専用の crate（`proc-macro = true`）に置かれる理由を説明できる
- [ ] 実務では `syn` と `quote` を使う理由を説明できる

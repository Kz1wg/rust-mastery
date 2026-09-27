# Lesson 06-2: 独自Error型

## Concept

複数の原因（ファイルI/O、パース失敗など）から失敗しうる関数を書くとき、
`enum Error`（具体的な列挙）と `Box<dyn std::error::Error>`（型消去）のどちらを使うかは、
**呼び出し側がエラーの種類ごとに違う対応をする必要があるか**で決まります。

## Why?

安易に `Box<dyn Error>` に頼ると、呼び出し側は表示することしかできなくなります。
逆に、表示するだけでよい場面で `enum` を手作りすると、無駄なボイラープレートが増えます。

## Bad Example: 型情報を早々に手放す

```rust
fn read_number(path: &str) -> Result<i32, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    content.trim().parse::<i32>().map_err(|e| e.to_string())
}

fn main() {
    match read_number("/nonexistent") {
        Ok(n) => println!("{n}"),
        Err(msg) => eprintln!("{msg}"),
    }
}
```

## Problem

`io::Error` も `ParseIntError` も、`.to_string()` で文字列にしてしまった時点で、
呼び出し側は「ファイルが無かったのか」「中身が数値でなかったのか」を、
**文字列の中身を見て判定するしかなくなります**（脆く、ロケールにも依存しえます）。

## Think

> **問い**: 呼び出し側が「ファイルが見つからない」場合だけ別の処理をしたい
>（例えばデフォルト値を使う）としたら、`Err` 型をどう設計しますか？

<details>
<summary>Solution</summary>

```rust
#[derive(Debug)]
enum ReadNumberError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl From<std::io::Error> for ReadNumberError {
    fn from(e: std::io::Error) -> Self {
        ReadNumberError::Io(e)
    }
}

impl From<std::num::ParseIntError> for ReadNumberError {
    fn from(e: std::num::ParseIntError) -> Self {
        ReadNumberError::Parse(e)
    }
}

fn read_number(path: &str) -> Result<i32, ReadNumberError> {
    let content = std::fs::read_to_string(path)?; // io::Error は自動でReadNumberError::Ioへ
    let n = content.trim().parse::<i32>()?; // ParseIntError も同様
    Ok(n)
}

fn main() {
    match read_number("/nonexistent") {
        Ok(n) => println!("{n}"),
        Err(ReadNumberError::Io(_)) => println!("ファイルが無いのでデフォルト値の 0 を使います"),
        Err(ReadNumberError::Parse(e)) => println!("数値として読めませんでした: {e}"),
    }
}
```

呼び出し側は `match` で `Io` と `Parse` を区別できます。
`?` が `From::from` を自動で呼んでくれる仕組みは、次のLesson（06-3）で扱います。

</details>

## `Box<dyn Error>` が向いている場面

呼び出し側が「表示する・ログに出す」以外の対応をしない場合、
`enum` を手作りするコストは見合いません。

```rust
use std::error::Error;

fn read_number(path: &str) -> Result<i32, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;
    let n = content.trim().parse::<i32>()?;
    Ok(n)
}

fn main() {
    match read_number("/nonexistent") {
        Ok(n) => println!("{n}"),
        Err(e) => eprintln!("failed: {e}"), // 表示するだけ
    }
}
```

`std::error::Error` を実装している型はすべて、`?` によって自動的に
`Box<dyn Error>` へ変換されます（`From<E> for Box<dyn Error>` が標準ライブラリに
用意されているため）。**独自の `From` 実装を書かずに済みます。**

## 使い分けの基準

| 状況 | 選ぶ |
| --- | --- |
| 呼び出し側がエラーの種類によって違う処理をする | `enum Error`（具体的な列挙） |
| 呼び出し側は表示・ログ出力しかしない（`main` の末端、CLIツールなど） | `Box<dyn Error>` |
| ライブラリとして公開し、利用者に判断を委ねたい | `enum Error`（06-4で詳しく） |

## Deep Dive: `std::error::Error` トレイトの役割

`std::error::Error` は、`Display`（人間向けメッセージ）と `Debug` の実装を要求し、
オプションで `source()`（この失敗を引き起こした、一段階下の原因のエラー）を提供できます。
`enum Error` を書くときも、可能なら `std::error::Error` を実装しておくと、
その `enum` 自体を `Box<dyn Error>` に変換できるようになり、
エラーを扱うエコシステム（`anyhow` のような外部クレートなど）とも噛み合いやすくなります。

```rust
use std::fmt;

#[derive(Debug)]
enum ReadNumberError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl fmt::Display for ReadNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadNumberError::Io(e) => write!(f, "input error: {e}"),
            ReadNumberError::Parse(e) => write!(f, "invalid number: {e}"),
        }
    }
}

impl std::error::Error for ReadNumberError {}

fn main() {
    let e = ReadNumberError::Parse("abc".parse::<i32>().unwrap_err());
    println!("{e}");
}
```

## Exercise

**[`ex023_custom_error_types`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex023_custom_error_types)** — `cargo test -p ex023_custom_error_types` で判定します。

`enum ReadNumberError` を、`Display` と `std::error::Error` の実装込みで作ります。

## Challenge

自分のコードで `.map_err(|e| e.to_string())` のような書き方をしている箇所を探し、
呼び出し側がエラーの種類で分岐する必要があるかどうか確認してください。

## Review

- [ ] `enum Error` と `Box<dyn Error>` を、呼び出し側の要求に応じて選べる
- [ ] `std::error::Error` トレイトが `Display` + `Debug` + オプションの `source()` を要求することを説明できる

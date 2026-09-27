# Lesson 06-3: error propagation

## Concept

`?` は、「`Err` だったらその場で `return` する」ための短い書き方です。
（このように、長い書き方を短く書けるようにした文法を**糖衣構文**と言います。）

ただし `?` は、`match` で書いた早期リターンを短くしただけではありません。
**`Err` の中身を、関数の戻り値のエラー型へ `From::from` で変換してから**返します。
この「変換してくれる」部分を知っていると、`?` が急にコンパイルエラーになったときの理由が分かります。

## Why?

この変換が自動で行われることを知らないと、`?` が急にコンパイルエラーになったとき
（`From` 実装が無いとき）に、何が起きているのか分からなくなります。

## Bad Example: `?` を使わない手書きの伝播

```rust
#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

fn read_number(path: &str) -> Result<i32, MyError> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(MyError::Io(e)),
    };
    let n = match content.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(MyError::Parse(e)),
    };
    Ok(n)
}

fn main() {}
```

## Problem

`match` で「成功なら続行、失敗なら包んで即座にreturn」というパターンが、
呼び出しの数だけ繰り返されています。本質的なロジック（読んでパースする）が、
このボイラープレートに埋もれています。

## Think

> **問い**: `?` を使って書き直すと、何が必要になりますか？

<details>
<summary>Hint</summary>

`?` は `Err(e)` を見つけたら、`e` を `From::from(e)` で関数の戻り値のエラー型に変換してから
`return Err(...)` します。この変換ができるように、何を用意する必要がありますか？

</details>

<details>
<summary>Solution</summary>

```rust
#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::Io(e)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(e: std::num::ParseIntError) -> Self {
        MyError::Parse(e)
    }
}

fn read_number(path: &str) -> Result<i32, MyError> {
    let content = std::fs::read_to_string(path)?;
    let n = content.trim().parse::<i32>()?;
    Ok(n)
}

fn main() {}
```

`?` は、`std::fs::read_to_string` が返す `Result<String, io::Error>` の `Err(io::Error)` を見つけたら、
**`MyError::from(io_error)` を呼んで `MyError` に変換してから** `return Err(...)` します。
これが `impl From<std::io::Error> for MyError` を用意しておく理由です。

</details>

## `From` が無いとどうなるか

```rust,compile_fail,E0277
#[derive(Debug)]
struct MyError;

fn read_number(path: &str) -> Result<i32, MyError> {
    let content = std::fs::read_to_string(path)?; // io::Error -> MyError の From が無い
    let n: i32 = content.trim().parse().unwrap();
    Ok(n)
}

fn main() {}
```

エラーメッセージは「`?` は `io::Error` を `MyError` に変換できなかった」という趣旨です。
`?` は変換方法を**推測しません**。`From` を実装するか、`.map_err(...)` で
明示的に変換してから `?` を使うか、どちらかが必要です。

## Deep Dive: `?` は「失敗を上に投げる」だけではない

`?` は `Result` だけでなく `Option` にも使えます（`None` なら即座に `None` を返す）。
また、`?` は「呼び出し元の関数の戻り値の型」に依存するため、
**`main` 関数自体が `Result` を返すようにする**と、`main` の中でも `?` が使えます。

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("/nonexistent")?;
    println!("{content}");
    Ok(())
}
```

`main` が `Err` を返すと、Rustは終了コード1でプロセスを終了し、
`Err` の `Debug` 表現を標準エラー出力に表示します。CLIツールの `main` でよく使われる形です。

## Exercise

**`ex024_error_propagation`** — `cargo test -p ex024_error_propagation` で判定します。

`?` と `From` 実装を使って、複数の原因から失敗しうる関数を実装します。

## Challenge

自分のコードで `match ... { Ok(x) => x, Err(e) => return Err(...) }` という形のボイラープレートを
書いている箇所があれば、`?` と `From` で置き換えられないか確認してください。

## Review

- [ ] `?` が「`Err` を `From::from` で変換してから早期returnする」ことを説明できる
- [ ] `?` を使うために `From` の実装が必要になる理由を説明できる
- [ ] `main` 自体が `Result` を返せることを知っている

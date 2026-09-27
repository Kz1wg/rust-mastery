# Lesson 02-3: 構築時検証 — 検証するのではなく、検証済みの型を作る

## Concept

「検証する関数」を用意することと、「検証済みであることを型が保証する」ことは違います。

## Why?

`fn is_valid_email(s: &str) -> bool` を用意しても、その関数を**呼ぶかどうか**は呼び出し側次第です。
呼び忘れても、コンパイルは通ります。

## Bad Example

```rust
fn is_valid_email(s: &str) -> bool {
    match s.split_once('@') {
        Some((local, domain)) => !local.is_empty() && !domain.is_empty(),
        None => false,
    }
}

fn send_welcome(email: &str) {
    // 検証済みだろうか？ この関数だけからは分からない
    println!("welcome mail to {email}");
}

fn main() {
    let input = "not-an-email";
    // 検証を呼び忘れても、コンパイルは通る
    send_welcome(input);

    if is_valid_email(input) {
        send_welcome(input);
    }
}
```

## Problem

- `send_welcome` は「検証済みかどうか」を**型では知らされていない**。呼ぶ前の検証は、慣習に頼っている。
- 安全のため、`send_welcome` の中で再検証すると、検証が**あちこちに重複**する。
- 呼び出しの連鎖が深くなるほど、どこで検証したか追えなくなる。

## Think

> **問い**:
> 1. 「検証済みのメールアドレス」だけを受け取る関数を、型でどう表現しますか？
> 2. 検証済みの `Email` を、**検証を通さずに作れない**ようにするには、何が必要ですか？
> 3. 検証が失敗したとき、呼び出し側にどんな情報を返すべきですか？

<details>
<summary>Hint</summary>

Lesson 02-2 の newtype に、**外部から直接作れない**という性質を加えます。
フィールドの公開範囲（`pub` の有無）と、構築用の関数に注目してください。

</details>

<details>
<summary>Solution</summary>

```rust
mod email {
    #[derive(Debug, PartialEq)]
    pub enum EmailError {
        MissingAt,
        EmptyLocal,
        EmptyDomain,
    }

    pub struct Email {
        value: String, // private: モジュール外からは直接作れない
    }

    impl Email {
        pub fn parse(s: &str) -> Result<Email, EmailError> {
            let (local, domain) = s.split_once('@').ok_or(EmailError::MissingAt)?;
            if local.is_empty() {
                return Err(EmailError::EmptyLocal);
            }
            if domain.is_empty() {
                return Err(EmailError::EmptyDomain);
            }
            Ok(Email { value: s.to_string() })
        }

        pub fn as_str(&self) -> &str {
            &self.value
        }
    }
}

use email::{Email, EmailError};

fn send_welcome(email: &Email) {
    // 検証済みであることが型で保証されている。再検証は不要。
    println!("welcome mail to {}", email.as_str());
}

fn main() {
    assert_eq!(Email::parse("nope").err(), Some(EmailError::MissingAt));

    let email = Email::parse("alice@example.com").unwrap();
    send_welcome(&email);
}
```

外部から `Email` を直接作ろうとすると、コンパイルエラーになります。

```rust,compile_fail,E0451
mod email {
    pub struct Email {
        value: String,
    }
}

fn main() {
    let e = email::Email { value: "not-an-email".to_string() };
}
```

**設計の要点**:

| 要点 | 内容 |
| --- | --- |
| フィールドは private | `Email::parse` を通らないと作れない＝**存在する `Email` は必ず検証済み** |
| 失敗を型で返す | `Result<Email, EmailError>`。失敗の理由も型（`enum`）で表す |
| 検証は境界で1回 | 入力を受けた場所（フォーム・API・ファイル読み込み）で `parse` し、内部は `Email` だけを扱う |

この考え方は "parse, don't validate" と呼ばれます。
**「`bool` を返す検証」は情報を捨て、「型を返すパース」は情報を残します。**

**注意**: ここでの検証は「`@` を含み、最初の `@` の前後が空でない」だけの簡易版です
（`split_once` は最初の `@` で分けるだけなので、`a@b@c` も通ります）。
実際のメールアドレスの仕様は複雑で、検証を厳密にしすぎると正当なアドレスを弾きます。
どこまで検証するかは**要件の問題**であり、型が保証するのは「この関数が検証した範囲」です。

</details>

## Deep Dive: `TryFrom` / `FromStr` にするか

`parse` という名前の独自メソッドの代わりに、標準のtraitを実装する選択肢があります。

```rust
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Port(u16);

#[derive(Debug, PartialEq)]
struct PortError;

impl FromStr for Port {
    type Err = PortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u16 = s.parse().map_err(|_| PortError)?;
        if n == 0 { Err(PortError) } else { Ok(Port(n)) }
    }
}

fn main() {
    assert_eq!("8080".parse::<Port>(), Ok(Port(8080)));
    assert_eq!("0".parse::<Port>(), Err(PortError));
}
```

| 方法 | 利点 | 欠点 |
| --- | --- | --- |
| 独自の `Email::parse(&str)` | 名前が明快。引数の型を自由に選べる | 標準のtraitと連携しない |
| `FromStr`（`"..".parse::<T>()`） | 標準的な書き方。エコシステムと連携 | 入力が `&str` に限られる |
| `TryFrom<U>` | `&str` 以外の型（`u8` など）からの変換にも使える。`.try_into()` が使える | `.parse()` 構文は使えない。エラー型は関連型 `Error` として定義する |

## Exercise

**[`ex007_parse_dont_validate`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex007_parse_dont_validate)** — `cargo test -p ex007_parse_dont_validate` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `Percentage` | 0〜100 の整数のみ。`Percentage::new(u8) -> Result<Percentage, PercentageError>`。private フィールド |
| `NonEmptyString` | 空でない文字列。`NonEmptyString::new(String) -> Option<NonEmptyString>` |
| 使う側の関数 | `fn describe(p: Percentage) -> String` は範囲を再検証しない。不正な `Percentage` を外部から直接作れないことは `compile_fail` doctest で確認 |

## Challenge

`Percentage::new` を `Option` で返す設計と `Result` で返す設計を比べてください。
呼び出し側は、どちらのほうがエラーの理由を活かせますか？（Chapter 06 で改めて扱います。）

## Review

- [ ] 「`bool` を返す検証」と「型を返すパース」の違いを説明できる
- [ ] private フィールド＋構築関数で、不正な値の存在を防げることを説明できる
- [ ] 検証を「境界で1回」にする理由を説明できる

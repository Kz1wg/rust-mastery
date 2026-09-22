# Lesson 06-1: `Result` の型設計

## Concept

`Err` の型をどう設計するかは、次の1つの問いに集約されます。

> **呼び出し側は、このエラーを見て、何をどう変える必要があるのか？**

## Why?

`Err` の型が粗すぎると、呼び出し側は「失敗した」以上のことが分かりません。
細かすぎると、呼び出し側が実際には区別しない情報にまで付き合わされます。

## Bad Example: 粗すぎる

```rust
fn parse_config(s: &str) -> Result<i32, String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err("設定が空です".to_string());
    }
    trimmed.parse::<i32>().map_err(|_| "数値として解釈できません".to_string())
}

fn main() {
    match parse_config("") {
        Ok(n) => println!("{n}"),
        Err(msg) => {
            // メッセージを表示することしかできない
            eprintln!("{msg}");
        }
    }
}
```

## Problem

呼び出し側は `Err(String)` を受け取っても、**表示する以外に何もできません**。
「空だったらデフォルト値を使う」「数値でなければ再入力を促す」のように、
理由に応じて違う対応をしたくても、文字列の中身を見て判定するしかありません
（文字列の変更に弱い、脆いコードになります）。

## Think

> **問い**: このケースで、呼び出し側が実際に区別したい失敗の種類は何通りありますか？
> それぞれの種類にどんな情報を持たせるべきですか？

<details>
<summary>Hint</summary>

「空だった」と「数値として不正だった」は、呼び出し側にとって明らかに違う対応をしたくなる状況です。
「数値として不正だった」場合、元の文字列（どう不正だったか分かる情報）を持たせると親切です。

</details>

<details>
<summary>Solution</summary>

```rust
#[derive(Debug, PartialEq)]
enum ConfigError {
    Empty,
    NotANumber(String),
}

fn parse_config(s: &str) -> Result<i32, ConfigError> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(ConfigError::Empty);
    }
    trimmed
        .parse::<i32>()
        .map_err(|_| ConfigError::NotANumber(trimmed.to_string()))
}

fn main() {
    match parse_config("") {
        Ok(n) => println!("{n}"),
        Err(ConfigError::Empty) => println!("デフォルト値の 0 を使います"),
        Err(ConfigError::NotANumber(s)) => println!("'{s}' は数値ではありません。入力し直してください"),
    }
}
```

呼び出し側は `match` で分岐でき、**それぞれの状況に応じた対応**を書けます。
`NotANumber(String)` に元の文字列を持たせているのは、
「呼び出し側がエラーメッセージを組み立てるときに使いたい情報」だからです
——「何が不正だったか」が分かることは、多くの場合、呼び出し側にとって意味があります。

</details>

## 粒度をどこで止めるか

「もっと細かくできるのでは？」という誘惑もあります。例えば `NotANumber` を
`TooManyDigits` / `ContainsLetter` / `ContainsSymbol` のように分けることもできますが、
**呼び出し側がそれぞれに対して違う対応をしないなら、分ける意味がありません**。

```rust
// 過剰な粒度: 呼び出し側はどうせ全部同じ扱いをする
enum ConfigError {
    Empty,
    TooManyDigits(String),
    ContainsLetter(String),
    ContainsSymbol(String),
}
```

02-1（`bool`が表現してしまう不正な状態）と同じ問いがここにも当てはまります。
**型の粒度は、「意味のある区別」の数に合わせます。** 呼び出し側の要求を超えて分けても、
`match` の腕が増えるだけで、誰の役にも立ちません。

## Deep Dive: 粒度は「今の呼び出し側」で決まるとは限らない

ライブラリとして公開する関数の場合、「今の呼び出し側」だけでなく
**「将来の、まだ見ぬ呼び出し側」**も考慮に入れる必要があります
（この視点は06-4で改めて扱います）。今のところは、
「自分のアプリケーション内で完結するコード」を前提に、
**今、実際に分岐が必要な種類**で判断してください。

## Exercise

**`ex022_result_design`** — `cargo test -p ex022_result_design` で判定します。

粗すぎる `Err = String` の関数を、呼び出し側が区別したい種類ごとの `enum` に設計し直します。

## Challenge

自分のコードから `Result<T, String>` を探してください。
呼び出し側が実際にメッセージの中身で分岐している（文字列比較している）箇所があれば、
それは `enum` に変えるべきサインです。

## Review

- [ ] `Err` 型の粒度を、「呼び出し側が実際に区別したいか」で判断できる
- [ ] `Result<T, String>` が呼び出し側の選択肢を狭めることを説明できる
- [ ] 過剰に細かい `enum` も、呼び出し側にとって無意味なら避けるべきだと説明できる

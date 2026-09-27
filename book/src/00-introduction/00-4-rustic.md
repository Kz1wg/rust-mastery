# Lesson 00-4: 「Rustらしさ」とは

## Concept

この教材では、コードを3つの段階で見分けます。

| 段階 | 意味 |
| --- | --- |
| 動くコード | 決まった入力に対して、期待した結果を返す |
| 短いコード | 少ない行数で書かれている |
| Rustらしいコード | Rust の型・所有権・エラー処理を使って、**間違いにくく、読み手に意図が伝わる**ように書かれている |

注意してほしいのは、**「Rustらしい」は「短い」ではない**ことです。
iterator を使った1行のコードが、`for` ループより Rust らしいとは限りません。

## Why?

AI にコードを書かせると、「動くコード」はすぐに手に入ります。
「短いコード」も、頼めば書いてくれます。
けれど、そのコードが**どんな入力で困ったことになるか**、**呼ぶ人に何を約束しているか**を判断するのは、
読む側の仕事です。この教材全体が、その判断力を鍛えるためにあります。

## Bad Example: 同じ仕事をする3つのコード

仕事の内容は「空白で区切られた点数の文字列（例: `"80 90 70"`）から、平均点を求める」です。

### A: 動くコード

```rust
fn average(input: String) -> f64 {
    let parts: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();
    let mut sum = 0;
    let mut i = 0;
    while i < parts.len() {
        sum += parts[i].parse::<i32>().unwrap();
        i += 1;
    }
    sum as f64 / parts.len() as f64
}

fn main() {
    assert_eq!(average("80 90 70".to_string()), 80.0);
}
```

### B: 短いコード

```rust
fn average(s: &str) -> f64 {
    let v: Vec<f64> = s.split_whitespace().filter_map(|x| x.parse().ok()).collect();
    v.iter().sum::<f64>() / v.len() as f64
}

fn main() {
    assert_eq!(average("80 90 70"), 80.0);
}
```

### C: もう1つのコード

```rust
#[derive(Debug, PartialEq)]
enum AverageError {
    /// 点数が1つもない。
    Empty,
    /// 点数として読めない値があった。
    InvalidScore(String),
}

fn average(input: &str) -> Result<f64, AverageError> {
    let scores = input
        .split_whitespace()
        .map(|s| {
            s.parse::<u32>()
                .map_err(|_| AverageError::InvalidScore(s.to_string()))
        })
        .collect::<Result<Vec<u32>, _>>()?;

    if scores.is_empty() {
        return Err(AverageError::Empty);
    }
    let sum: u32 = scores.iter().sum();
    Ok(f64::from(sum) / scores.len() as f64)
}

fn main() {
    assert_eq!(average("80 90 70"), Ok(80.0));
}
```

3つとも、`"80 90 70"` に対しては同じ 80.0 を返します。テストが1つだけなら、3つとも「合格」です。

## Problem

違いは、**想定していない入力**が来たときに現れます。

| 入力 | A | B | C |
| --- | --- | --- | --- |
| `"80 90 70"` | 80.0 | 80.0 | `Ok(80.0)` |
| `"80 x 70"`（読めない値） | **panic** | 75.0（`x` を黙って捨てる） | `Err(InvalidScore("x"))` |
| `"80  90"`（空白が2つ） | **panic**（空の文字列を数にしようとする） | 85.0 | `Ok(85.0)` |
| `""`（空） | **panic** | **NaN**（0 ÷ 0） | `Err(Empty)` |

実際に確かめてみましょう。A に `"80 x 70"` を渡すと、panic します。

```rust,should_panic
# fn average(input: String) -> f64 {
#     let parts: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();
#     let mut sum = 0;
#     let mut i = 0;
#     while i < parts.len() {
#         sum += parts[i].parse::<i32>().unwrap();
#         i += 1;
#     }
#     sum as f64 / parts.len() as f64
# }
fn main() {
    average("80 x 70".to_string()); // unwrap で panic する
}
```

B に空の文字列を渡すと、エラーにならずに NaN（「数ではない」を表す特別な値）が返ります。

```rust
# fn average(s: &str) -> f64 {
#     let v: Vec<f64> = s.split_whitespace().filter_map(|x| x.parse().ok()).collect();
#     v.iter().sum::<f64>() / v.len() as f64
# }
fn main() {
    let result = average("");
    assert!(result.is_nan()); // エラーにならず、NaN が返る
}
```

## Think

> **問い**: A・B・C を、次の観点で比べてください。どれが一番良いかではなく、**それぞれの観点で何が違うか**を考えてください。
>
> correctness（正しさ） / readability（読みやすさ） / error handling（エラー処理） / type safety（型による安全性） /
> ownership（所有権） / performance（性能） / API design（呼ぶ人から見た使いやすさ）

<details>
<summary>Hint</summary>

関数のシグネチャだけを見比べてください。

```rust,ignore
fn average(input: String) -> f64                       // A
fn average(s: &str) -> f64                             // B
fn average(input: &str) -> Result<f64, AverageError>   // C
```

呼ぶ人は、シグネチャを見て「この関数は失敗することがあるか」を判断できるでしょうか。

</details>

<details>
<summary>Solution</summary>

| 観点 | A | B | C |
| --- | --- | --- | --- |
| correctness | 空白の入り方次第で panic | 読めない値を黙って捨てる。空なら NaN | 想定外の入力を、種類を分けて知らせる |
| error handling | `unwrap` で panic。呼ぶ人は回復できない | 失敗が**見えない**。NaN は後の計算にも広がる | `Result` で返す。どうするかは呼ぶ人が決める |
| type safety | 点数を `i32` で読むので、負の点数も通る | `f64` で読むので `"1e3"` や `"NaN"` も点数として通る | `u32` なので負の数は読めない |
| ownership | 読むだけなのに `String` の所有権を要求する | `&str` で借りる | `&str` で借りる |
| performance | 区切った部分ごとに `String` を作る（不要なコピー） | コピーなし | コピーなし（エラーのときだけ `String` を作る） |
| readability | 手続きが長く、意図（平均を求める）が見えにくい | 短いが、「捨てる」という重要な判断が `filter_map` の中に埋もれている | 一番長いが、失敗の種類が型の名前として読める |
| API design | シグネチャからは失敗しうることが分からない | 同じく分からない | シグネチャに「失敗しうる」「どんな失敗か」が書いてある |

C は一番長いコードです。それでも、この教材が「Rustらしい」と呼ぶのは C です。
理由は、**失敗の可能性を型で表し、呼ぶ人に判断を任せている**からです。

ただし、C がいつでも正解というわけではありません。
「ログから点数を集計する。壊れた行は無視してよい」という**要件**なら、B の「読めない値を捨てる」は正しい判断です。
その場合でも、空の入力で NaN を返すのは避けて、`Option<f64>` を返すなどの工夫をしたくなります。

**何が「良いコード」かは、要件によって決まります。**
Rustらしさとは、その要件を、型・所有権・エラー処理を使って**コードの形そのもので**表すことです。

</details>

## Deep Dive: 「動く」「短い」「Rustらしい」の関係

3つは、順番に積み上がる段階ではありません。

- 「動く」は最低条件です。動かないコードは、どれだけ型が美しくても意味がありません
- 「短い」は、それ自体は目標ではありません。短くしたことで判断が見えなくなるなら（B の `filter_map`）、長いほうが良いこともあります
- 「Rustらしい」は、次の観点を**まとめて**考えた結果です

```text
correctness / readability / maintainability / API design / type safety
ownership / performance / abstraction / error handling
```

この教材の各章は、この観点のどれかを深く掘り下げます。
たとえば、A の `String` を受け取る問題は Chapter 01、C の `AverageError` は Chapter 06、
B の NaN のような「型が許してしまう変な値」は Chapter 02 のテーマです。

## Challenge

C の `average` を、AI に「もっと短く書き直して」と頼んでみてください。
返ってきたコードについて、次を確かめてください。

1. 上の表の4つの入力に、同じ結果を返すか
2. 返さないなら、それは要件として許されるか
3. シグネチャ（とくに戻り値の型）は変わったか。変わったなら、呼ぶ人にとって何が変わるか

## Review

- [ ] 「動く」「短い」「Rustらしい」の違いを、例を挙げて説明できる
- [ ] シグネチャを見て、関数が失敗を知らせる方法（panic / 特別な値 / `Result`）を見分けられる
- [ ] 「どのコードが良いか」は要件によって変わることを、B と C の例で説明できる

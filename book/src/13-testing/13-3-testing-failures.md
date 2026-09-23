# Lesson 13-3: 失敗ケースのテスト

## Concept

テストというと「正しく動くこと」を確かめがちですが、同じくらい大事なのは
**「間違った入力のときに、ちゃんと失敗すること」**を確かめることです。

- `Result` を返す関数なら、**どの `Err` が返るか**まで確かめる
- panic する関数なら、**どんなメッセージで panic するか**まで確かめる

## Why?

失敗ケースは、普段の動作確認ではなかなか通りません。
その分、壊れても気づきにくく、**本番で初めて変な失敗の仕方をする**ことがよくあります。

## Bad Example: 「失敗したこと」しか見ていない

```rust
#[derive(Debug, PartialEq)]
enum AgeError {
    Empty,
    NotANumber,
    TooOld,
}

fn parse_age(s: &str) -> Result<u8, AgeError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(AgeError::Empty);
    }
    let n: u8 = s.parse().map_err(|_| AgeError::NotANumber)?;
    if n > 150 {
        return Err(AgeError::TooOld);
    }
    Ok(n)
}

fn main() {
    // どのテストも「Err であること」しか確かめていない
    assert!(parse_age("").is_err());
    assert!(parse_age("abc").is_err());
    assert!(parse_age("200").is_err());
}
```

## Problem

3つとも `is_err()` しか見ていません。
もし実装を間違えて、空文字列のときに `NotANumber` を返すようになっても、**このテストは通り続けます**。
Chapter 06 で「呼び出し側が区別したいから `Err` を enum にした」のに、テストがその区別を確かめていないのです。

もう1つ気づいてほしい点があります。`"200"` は `u8` の範囲（0〜255）に収まるので `TooOld` になりますが、
`"300"` は `u8` に入らないので `parse` の時点で失敗し、`NotANumber` になります。
**「どの `Err` になるか」を確かめていないと、こういう意外な動きにも気づけません。**

## Think

> **問い**: このテストを、「どの種類の失敗か」まで確かめるように書き直してください。

<details>
<summary>Solution</summary>

`Err` の中身まで比べます。`AgeError` が `PartialEq` を derive しているので、`assert_eq!` がそのまま使えます。

```rust
# #[derive(Debug, PartialEq)]
# enum AgeError { Empty, NotANumber, TooOld }
# fn parse_age(s: &str) -> Result<u8, AgeError> {
#     let s = s.trim();
#     if s.is_empty() { return Err(AgeError::Empty); }
#     let n: u8 = s.parse().map_err(|_| AgeError::NotANumber)?;
#     if n > 150 { return Err(AgeError::TooOld); }
#     Ok(n)
# }
fn main() {
    assert_eq!(parse_age(""), Err(AgeError::Empty));
    assert_eq!(parse_age("abc"), Err(AgeError::NotANumber));
    assert_eq!(parse_age("200"), Err(AgeError::TooOld));

    // u8 に収まらない数は、parse の時点で失敗する（意外な動きもテストで固定しておく）
    assert_eq!(parse_age("300"), Err(AgeError::NotANumber));

    // 成功側も、境目を確かめる
    assert_eq!(parse_age("150"), Ok(150));
    assert_eq!(parse_age("  42 "), Ok(42));
}
```

エラー型が `PartialEq` を実装していない場合（`std::io::Error` を持っている、など）は、
`matches!` マクロで**バリアントの種類だけ**を確かめられます。

```rust,ignore
assert!(matches!(result, Err(MyError::Io(_))));
```

</details>

## 境界値を確かめる

失敗ケースを探すときは、**境目（境界値）**を狙うのが効果的です。

| 入力の種類 | 例（`parse_age` の場合） |
| --- | --- |
| ちょうど境目 | `"150"`（OK の最大）と `"151"`（NG の最小） |
| 空・ゼロ | `""`、`"0"` |
| 型の限界 | `"255"`（`u8` の最大）、`"256"`（`u8` に入らない） |
| 余計な文字 | `" 42 "`（前後の空白）、`"4 2"`（間の空白） |

バグは「普通の値」より「境目」に集まりやすいので、境目を並べるだけでテストの価値が大きく上がります。

## panic をテストする: `#[should_panic]`

関数が panic すること自体が正しい動き、という場合もあります（例: 前提条件を破った呼び出し）。
そのときは `#[should_panic]` を使います。

```rust,ignore
fn get_item(items: &[i32], index: usize) -> i32 {
    if index >= items.len() {
        panic!("index out of range: {index}");
    }
    items[index]
}

#[test]
#[should_panic(expected = "out of range")]
fn panics_on_bad_index() {
    get_item(&[1, 2, 3], 10);
}
```

**`expected = "..."` を必ず付けてください。** 付けないと、「何でもいいから panic すれば合格」になり、
全く別の理由（例えば配列の範囲外アクセスや `unwrap` の失敗）で panic しても、テストが通ってしまいます。
`expected` を付けると、panic のメッセージにその文字列が**含まれていなければ失敗**します。

## Deep Dive: panic させるか、`Result` を返すか

テストの書き方を考えると、設計の問いに戻ってきます。

| 失敗の性質 | 選ぶもの | テストの書き方 |
| --- | --- | --- |
| 呼び出し側が回復できる（ユーザー入力の誤り、ファイルが無い） | `Result` | `assert_eq!(f(x), Err(...))` |
| 呼び出し側のバグ（前提条件を破った） | panic | `#[should_panic(expected = "...")]` |

「ユーザーが空文字列を入力した」は回復できる失敗なので `Result` です。
「プログラマが範囲外のインデックスを渡した」はバグなので panic でも構いません。
**テストしにくいと感じたら**（例えば、panic をテストするのが面倒）、そもそも `Result` にすべきではないか、
考え直すきっかけにしてください。

## Exercise

**`ex052_testing_failures`** — `cargo test -p ex052_testing_failures` で判定します。

| 関数 | 仕様 |
| --- | --- |
| `parse_age(s)` | 空なら `Empty`、数値でなければ `NotANumber`、150 を超えたら `TooOld` |
| `get_item(items, index)` | 範囲外なら `"index out of range"` を含むメッセージで panic する |

テストは、`Err` の**種類**と、panic の**メッセージ**まで確かめています。

## Challenge

`parse_age` の境界値テストを、上の表を参考に自分で5つ追加してください。
その中に、実装を読む前には予想していなかった結果になるものはありましたか？

## Review

- [ ] `Err` の種類まで確かめるテストを書ける（`assert_eq!` / `matches!`）
- [ ] 境界値を狙ってテストケースを選べる
- [ ] `#[should_panic]` に `expected` を付ける理由を説明できる
- [ ] panic と `Result` を、失敗の性質で使い分けられる

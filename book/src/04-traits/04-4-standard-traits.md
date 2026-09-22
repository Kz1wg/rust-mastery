# Lesson 04-4: 標準trait実装の設計

## Concept

`From`, `Display`, `Default` のような標準traitには、明文化されていない**文化的な約束**があります。
実装すること自体は自由ですが、約束を破ると、呼び出し側は気づかないまま危険なコードを書きます。

## Why?

標準traitには、名前だけで「こう振る舞うはず」という期待がついてきます。
その期待を裏切る実装は、バグではなくコンパイルが通る**トラップ**になります。

## Bad Example: `From` は失敗してはいけない

```rust
struct Percentage(u8);

impl From<&str> for Percentage {
    fn from(s: &str) -> Self {
        Percentage(s.parse().unwrap()) // parseに失敗するとpanicする
    }
}

fn main() {
    let ok: Percentage = "50".into();
    println!("{}", ok.0);

    // 一見ふつうの変換に見えるが、"abc" を渡すと panic する
    // let bad: Percentage = "abc".into();
}
```

## Problem

`From::from` は**失敗しない変換**のためのtraitです。`.into()` を呼ぶ人は、
「変換が失敗するかもしれない」とは想定していません——`From` という名前自体が
「これは常に成功する」という約束を運んでいるからです。

失敗しうる変換に `From` を実装すると、呼び出し側は失敗の可能性を全く意識せずに
`.into()` を書き、本番で初めてpanicに遭遇します。

## Think

> **問い**:
> 1. 「失敗しうる変換」のためにRustが用意しているtraitは何ですか？
> 2. `Percentage::from("abc")` を書き直すとしたら、どういうシグネチャにしますか？

<details>
<summary>Hint</summary>

Chapter 02-3 で見た「検証して `Result` を返す」設計を思い出してください。
標準ライブラリには `From` の「失敗してもよい版」があります。

</details>

<details>
<summary>Solution</summary>

```rust
struct Percentage(u8);

#[derive(Debug, PartialEq)]
struct PercentageError;

impl TryFrom<&str> for Percentage {
    type Error = PercentageError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let n: u8 = s.parse().map_err(|_| PercentageError)?;
        if n <= 100 {
            Ok(Percentage(n))
        } else {
            Err(PercentageError)
        }
    }
}

fn main() {
    let ok: Result<Percentage, _> = "50".try_into();
    assert!(ok.is_ok());

    let bad: Result<Percentage, _> = "abc".try_into();
    assert_eq!(bad.err(), Some(PercentageError));
}
```

`TryFrom` を実装すると、`.try_into()` が使えるようになり、
呼び出し側は `Result` を受け取ることになります。失敗の可能性が**シグネチャに現れます**。

**判断基準**: 変換の実装に `unwrap`・`expect`・`panic!` が出てくる時点で、
それは `From` ではなく `TryFrom` の仕事です。

</details>

## Bad Example: `Display` が秘密の値を漏らす

```rust
struct Password(String);

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0) // 生のパスワードがそのまま出力される
    }
}

fn log_action(action: &str, p: &Password) {
    // 悪気なく書いたログ出力が、パスワードを漏らしてしまう
    println!("{action}: {p}");
}

fn main() {
    let p = Password("hunter2".to_string());
    log_action("login attempt", &p);
}
```

## Think

> **問い**:
> 1. `Display` は「誰に向けた」文字列表現を保証するtraitですか？
> 2. `Password` に `Display` を実装すべきでしょうか？
> 3. デバッグ用にログへ出力したい場合、どうしますか？

<details>
<summary>Solution</summary>

`Display` は「**エンドユーザーに見せてよい**文字列」を保証するtraitです。
`Password` のような機密情報には、そもそも「ユーザーに見せてよい表現」は存在しません。
**`Display` を実装しないのが正しい選択**です。

デバッグ・ログ用途には `Debug` を使いますが、そのまま `#[derive(Debug)]` すると
やはり生の値が出力されてしまいます。**手動で実装し、値を隠します**。

```rust
struct Password(String);

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Password(***)")
    }
}

fn main() {
    let p = Password("hunter2".to_string());
    println!("{p:?}"); // Password(***)
}
```

`Debug` は本来「開発者向けの内部表現」を保証するtraitですが、
機密情報についてはその `Debug` ですら中身を隠す判断が必要になる、という例です。

</details>

## `AsRef` の約束: 安く、副作用がないこと

```rust
struct Email(String);

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
```

`AsRef<T>` は「ほぼ無料で `&T` に変換できる」ことを約束します。
参照を返すだけなので、これは守れています。もし `as_ref` の中でネットワーク通信をしたり、
新しい `String` を毎回確保したりすると、呼び出し側は「軽い操作のはず」と思って
ループの中で気軽に呼び、意図しないコストを払うことになります。

**`AsRef` を実装してよいのは、実質的に「すでにある値への参照を返すだけ」のときだけです。**

## Deep Dive: `Default` は「意味のある既定値」があるときだけ

```rust
struct Percentage(u8);

impl Default for Percentage {
    fn default() -> Self {
        Percentage(0) // 0% はPercentageにとって自然な既定値と言えるか？
    }
}
```

`Default` を実装するかどうかは、「**その型にとって、利用者が驚かない既定値が存在するか**」で判断します。
`Vec::default()` が空の `Vec` を返すのは自然です。しかし `Percentage` にとって `0` が
「妥当な既定値」なのか、それとも「たまたま選んだ数字」なのかは、ドメインによります。
**既定値の意味を説明できないなら、`Default` を実装しないのも正しい選択です。**

## Exercise

**`ex016_standard_traits`** — `cargo test -p ex016_standard_traits` で判定します。

`Percentage` に `TryFrom<&str>` を実装し、`Password` に「中身を隠す」独自の `Debug` を実装します。

## Challenge

自分のコードに `#[derive(Debug)]` を付けた型の中に、ログに出てはいけない値
（パスワード、APIキー、個人情報）を持つものがないか確認してください。

## Review

- [ ] `From` と `TryFrom` を、失敗しうるかどうかで使い分けられる
- [ ] `Display` が「誰に向けた文字列か」を保証するtraitであることを説明できる
- [ ] `AsRef` が「安価であること」を約束することを説明できる
- [ ] `Default` を実装するかどうかを、「意味のある既定値があるか」で判断できる

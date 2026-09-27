# Lesson 08-1: lifetime annotationは何を主張しているのか

## Concept

**lifetime（ライフタイム）**は、「参照が安全に使える期間」のことです。

参照は、何か別の値を指しています。指している先の値が消えた（drop された）後に参照を使うと、
もう存在しないものを読むことになってしまいます。Rust はそれを防ぐため、
コンパイル時に「この参照は、指している先より長く使われていないか」を確かめています。

ほとんどの場合、この確認はコンパイラが自動でやってくれます。
しかし、**コンパイラが自分では判断できない場面**があり、そのときだけ私たちが `'a` のような印（lifetime 注釈）を書いて伝えます。

```rust,ignore
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
```

この `'a` が伝えているのは、次のことです。

> 「戻り値は、`a` と `b` の**両方がまだ生きている間だけ**使える」

大事なのは、**注釈を書いても値が長生きするわけではない**ことです。
注釈は、参照どうしの関係（どの参照が、どの参照に頼っているか）をコンパイラに説明しているだけです。

## Why?

lifetime注釈を「コンパイラを黙らせるおまじない」として付けていると、
間違った関係を宣言してしまい、呼び出し側に不要な制約を課すことになります。

## Bad Example

```rust,compile_fail,E0106
fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {}
```

## Problem

戻り値の `&str` は、`a` を借りているのでしょうか、`b` を借りているのでしょうか。
**実行してみるまで分かりません**（長さで決まるため）。コンパイラは関数の**本体を見て**推測することはせず、
シグネチャだけで判断します。入力の参照が2つあると、どちらに結びつけるかを決められません。

## Think

> **問い**:
> 1. 戻り値の参照を安全に使える期間は、`a` と `b` の生存期間とどういう関係にありますか？
> 2. それを注釈でどう書きますか？
> 3. その注釈は、呼び出し側に**どんな制約**を課しますか？

<details>
<summary>Hint</summary>

どちらが返るか分からない以上、戻り値は「`a` と `b` の**短い方**」の間しか安全に使えません。

</details>

<details>
<summary>Solution</summary>

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let a = String::from("long string");
    let b = String::from("xyz");
    println!("{}", longest(&a, &b));
}
```

`'a` という1つの名前を3箇所に付けることで、「`a` も `b` も少なくとも `'a` の間は生きている。
戻り値も `'a` の間だけ有効」と宣言しています。呼び出し時、コンパイラは `'a` を
**`a` と `b` の生存期間が重なる部分**として選びます。

その結果、呼び出し側には次の制約がかかります。

```rust,compile_fail,E0597
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let a = String::from("long string");
    let result;
    {
        let b = String::from("xyz");
        result = longest(&a, &b);
    } // b はここで破棄される
    println!("{result}");
}
```

実行すれば `result` は `a`（長い方）を指しているので、本当は安全です。
しかしシグネチャが「戻り値は `b` を借りているかもしれない」と宣言しているので、コンパイラは拒否します。
**コンパイラが信じるのは本体ではなくシグネチャ**です。

</details>

## Deep Dive: 本当に必要な関係だけを宣言する

もし関数が**常に `a` を返す**なら、`b` を戻り値と結びつける必要はありません。

```rust
fn first<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str {
    a
}

fn main() {
    let a = String::from("keep me");
    let result;
    {
        let b = String::from("temporary");
        result = first(&a, &b);
    } // b が破棄されても、result は a だけを借りているので問題ない
    println!("{result}");
}
```

`'a` と `'b` を別々にしたことで、「戻り値は `b` とは無関係」という**より正確な約束**になり、
呼び出し側の自由度が上がりました。

実は、`'b` は書かなくても同じ意味になります。省略規則（Lesson 08-3）の1つ目
「引数の各参照には、それぞれ別の lifetime が割り当てられる」により、`_b: &str` と書けば
自動的に `a` とは別の lifetime になるからです。clippy も `'b` は不要（`needless_lifetimes`）と指摘します。
慣用的には、**関係を宣言する必要がある lifetime（ここでは `'a`）だけに名前を付けます**。

```rust
fn first<'a>(a: &'a str, _b: &str) -> &'a str {
    a
}

fn main() {
    println!("{}", first("keep", "other"));
}
```

lifetime注釈を書くときの問いは、**「この戻り値は、どの引数を借りているのか」**です。
全部に同じ `'a` を付けるのは簡単ですが、それは「全部を借りているかもしれない」という
最も制約の強い宣言です。

## Exercise

**`ex030_lifetime_annotations`** — `cargo test -p ex030_lifetime_annotations` で判定します。

| 関数 | シグネチャ（用意済み） | 実装すること |
| --- | --- | --- |
| `longest` | `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` | 長い方を返す（同じ長さなら `a`） |
| `first` | `fn first<'a>(a: &'a str, _b: &str) -> &'a str` | 常に `a` を返す |

テストには、`first` の結果が `b` の破棄後も使えることを確かめるものが含まれています。
2つのシグネチャの違いが、呼び出し側にとって何を意味するか説明してください。

## Challenge

`first` のシグネチャを `longest` と同じ `fn first<'a>(a: &'a str, _b: &'a str) -> &'a str` に変えてテストを走らせ、
どのテストがコンパイルできなくなるか確かめてください。

## Review

- [ ] lifetime注釈が値の生存期間を変えず、参照どうしの関係を宣言するものだと説明できる
- [ ] コンパイラが関数の本体ではなくシグネチャを信じることを説明できる
- [ ] 「戻り値がどの引数を借りているか」に応じて、lifetimeを分けて書ける

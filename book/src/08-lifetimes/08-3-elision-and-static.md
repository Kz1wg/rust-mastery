# Lesson 08-3: elisionと `'static`

## Concept

普段 Rust を書いていて、`'a` をほとんど書かずに済んでいるのはなぜでしょうか。
それは、コンパイラが**よくある形なら、lifetime を自動で補ってくれる**からです。これを**省略（elision）**と言います。

このLessonでは2つのことを扱います。

1. コンパイラが自動で補ってくれる**3つの規則**と、補ってくれないとき
2. よく誤解される `'static` の本当の意味（「永遠に生きる」とは限らない）

## 省略規則（lifetime elision）

コンパイラは、次の3つの規則だけを機械的に当てはめます。関数の中身を見て推測することはしません。

| 規則 | 内容 |
| --- | --- |
| 1 | 引数の各参照には、それぞれ別の lifetime が割り当てられる |
| 2 | 入力の lifetime が**ちょうど1つ**なら、出力の参照はすべてそれになる |
| 3 | メソッドで `&self` / `&mut self` があるなら、出力の参照は `self` の lifetime になる |

08-1 の `longest(a: &str, b: &str) -> &str` は、入力が2つで `self` も無いので、規則2・3のどちらも使えず、省略できませんでした。
08-2 の `next_word(&mut self) -> Option<&str>` は、規則3で `self` に結びつきました。

## Bad Example: 規則3が意図と合わない

```rust,compile_fail
struct Cache {
    data: String,
}

impl Cache {
    // 空なら自分のデータ、そうでなければ key 自体を返したい
    fn get(&self, key: &str) -> &str {
        if key.is_empty() {
            &self.data
        } else {
            key
        }
    }
}

fn main() {}
```

## Problem

規則3により、戻り値は `&self` の lifetime に結びつきます。しかし本体は `key` を返すことがあり、
`key` が `self` と同じだけ生きる保証はありません。
（このエラーにはエラーコードが付かず、「lifetime may not live long enough」とだけ表示されます。）

**省略規則は「よくある形」を楽にする仕組みで、意図を推測してくれるわけではありません。**
意図と合わなければ、注釈を書いて関係を明示します。

## Think

> **問い**: `get` の戻り値は `self` と `key` のどちらも指しうります。シグネチャをどう直しますか？

<details>
<summary>Solution</summary>

```rust
struct Cache {
    data: String,
}

impl Cache {
    fn get<'a>(&'a self, key: &'a str) -> &'a str {
        if key.is_empty() {
            &self.data
        } else {
            key
        }
    }
}

fn main() {
    let c = Cache { data: String::from("default") };
    println!("{} {}", c.get(""), c.get("k"));
}
```

`self` と `key` の両方に `'a` を付け、「戻り値はどちらも借りうる」と宣言しました（08-1 の `longest` と同じ形です）。
ただしこの設計は「`self` と `key` の短い方の間しか使えない」という制約を呼び出し側に課します。
それが嫌なら、戻り値を `String` にして所有させる、というのも正当な選択肢です。

</details>

## `'static` の2つの意味

`'static` には、よく混同される2つの使われ方があります。

| 書き方 | 意味 |
| --- | --- |
| `&'static str` | **プログラム終了まで有効な参照**（文字列リテラル、`Box::leak` したもの） |
| `T: 'static` | **`T` が `'static` でない参照を含まない**。`String` や `i32` のような所有型は満たす |

`T: 'static` は「`T` の値が永遠に生きる」という意味では**ありません**。
`String` は `'static` を満たしますが、関数の終わりで普通に破棄されます。
意味は「**この値は、どこかの一時的な借用に縛られていない**」です。

```rust,compile_fail,E0597
fn keep<T: std::fmt::Debug + 'static>(item: T) -> Box<dyn std::fmt::Debug> {
    Box::new(item)
}

fn main() {
    let owned = String::from("ok");
    let _a = keep(owned); // OK: String は参照を含まない

    let local = String::from("x");
    let _b = keep(&local); // NG: &local は local の借用に縛られている
}
```

`T: 'static` が要求されるのは、値を**いつまで使うか関数側が決められない**場面です。
典型例は `std::thread::spawn`（別スレッドがいつ終わるか分からない）や、
`Box<dyn Trait>` として長く保持する場合です（Chapter 10 で再登場します）。

## Deep Dive: エラーを `'static` で黙らせない

lifetimeエラーに遭遇したとき、「`'static` を付けたら通った」という経験があるかもしれません。
多くの場合、それは**呼び出し側に「文字列リテラルしか渡せない」という強い制約を押し付けた**だけです。
`'static` を付ける前に、**「その参照は本当にプログラム終了まで必要か。所有させれば済むのではないか」**
を考えてください。

## Exercise

**`ex032_static_bound`** — `cargo test -p ex032_static_bound` で判定します。

`fn describe_later<T: std::fmt::Display + 'static>(item: T) -> Box<dyn Fn() -> String>` を実装します
（`item` を後で文字列化するクロージャを返す）。テストでは `String` や `i32` のような所有型を渡せること、
`compile_fail` doctest では、ローカル変数への参照を渡せないことを確認します。

## Challenge

`T: 'static` を外して `fn describe_later<T: std::fmt::Display>(item: T) -> Box<dyn Fn() -> String>` にすると、
どんなエラーになるか確かめてください。なぜ `Box<dyn Fn() -> String>` は `'static` を要求するのでしょうか。

## Review

- [ ] lifetime省略の3つの規則を説明できる
- [ ] 省略規則が意図と合わないとき、注釈で関係を明示できる
- [ ] `&'static T` と `T: 'static` の違いを説明できる
- [ ] lifetimeエラーを `'static` で安易に黙らせない

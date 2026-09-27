# Lesson 08-2: 構造体が参照を持つ設計

## Concept

構造体のフィールドに参照を持たせると、その構造体は**参照先より長く生きられなくなります**。
これは制約ですが、同時に「コピーせずに元のデータを読む」という利点でもあります。
問いは2つあります。

1. そもそも参照を持つべきか、所有すべきか
2. 参照を持つなら、メソッドの戻り値を `&self` と構造体の `'a` のどちらに結びつけるか

## Why?

参照を持つ構造体は、使い方を間違えると「作った関数から返せない」「2回目の呼び出しでエラー」
といった、一見不可解なエラーを生みます。原因はたいてい、lifetimeの結びつけ方の設計にあります。

## Bad Example 1: 作った関数から返せない

```rust,compile_fail,E0515
struct Parser<'a> {
    input: &'a str,
}

fn make_parser() -> Parser<'static> {
    let text = String::from("hello world");
    Parser { input: &text }
}

fn main() {}
```

## Problem 1

`text` は `make_parser` の終わりで破棄されます。`Parser` はその `text` を借りているので、
関数の外へ出ることはできません。**参照を持つ構造体は、参照先を作った場所より外へは出られません。**

## Think 1

> **問い**: 構造体に参照を持たせるか、所有させるか。どういう基準で決めますか？

<details>
<summary>Solution</summary>

| 持たせ方 | 利点 | 制約 | 向いている場面 |
| --- | --- | --- | --- |
| 参照（`&'a str`） | コピーしない。大きな入力でも安い | 参照先より長く生きられない。型に `'a` が付く | 呼び出し側が持っているデータを**一時的に読む**（パーサ、ビュー、イテレータ） |
| 所有（`String`） | どこへでも持ち運べる。`'a` が不要 | 作るときにコピー（確保）が必要 | 構造体が**長く生きる**、別スレッドへ送る、関数から返す |

迷ったら、**まず所有**で書いてください。所有する構造体はlifetimeを気にせず使え、
設計の自由度が高いからです。プロファイルでコピーが問題になったとき、あるいは
「一時的に読むだけ」が明らかなときに、参照へ切り替えます。

```rust
struct OwnedParser {
    input: String,
}

fn make_parser() -> OwnedParser {
    let text = String::from("hello world");
    OwnedParser { input: text } // 所有権ごと渡すので、返せる
}

fn main() {
    let p = make_parser();
    println!("{}", p.input);
}
```

</details>

## Bad Example 2: 2回目の呼び出しでエラーになる

参照を持つパーサで、単語を1つずつ取り出すメソッドを書きます。

```rust,compile_fail,E0499
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn next_word(&mut self) -> Option<&str> {
        let rest = self.input[self.pos..].trim_start();
        if rest.is_empty() {
            return None;
        }
        let start = self.input.len() - rest.len();
        let end = rest.find(' ').map(|i| start + i).unwrap_or(self.input.len());
        self.pos = end;
        Some(&self.input[start..end])
    }
}

fn main() {
    let mut p = Parser { input: "hello world", pos: 0 };
    let w1 = p.next_word();
    let w2 = p.next_word(); // 1つ目の単語を持ったまま、2回目を呼べない
    println!("{w1:?} {w2:?}");
}
```

## Problem 2

`fn next_word(&mut self) -> Option<&str>` の戻り値には lifetime を書いていません。
このとき、コンパイラは「戻り値は `self` を借りている」と自動で決めます（省略規則。詳しくは 08-3）。
つまり、戻り値は **`&mut self` の借用**に結びつきます。つまり「返した単語が生きている間、`p` は可変借用されたまま」
という宣言になり、`w1` を持ったまま2回目の `next_word` を呼べません。

しかし実際には、返している単語は `self.input` の一部であり、**パーサではなく元の文字列を指しています**。
パーサを借り続ける必要はありません。

## Think 2

> **問い**: 戻り値の単語が「パーサ」ではなく「元の文字列」を借りていることを、シグネチャでどう表しますか？

<details>
<summary>Hint</summary>

構造体にはすでに `'a`（元の文字列の lifetime）という名前があります。

</details>

<details>
<summary>Solution</summary>

```rust
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn next_word(&mut self) -> Option<&'a str> {
        let rest = self.input[self.pos..].trim_start();
        if rest.is_empty() {
            return None;
        }
        let start = self.input.len() - rest.len();
        let end = rest.find(' ').map(|i| start + i).unwrap_or(self.input.len());
        self.pos = end;
        Some(&self.input[start..end])
    }
}

fn main() {
    let mut p = Parser { input: "hello world", pos: 0 };
    let w1 = p.next_word();
    let w2 = p.next_word(); // OK: w1 は p ではなく元の文字列を借りている
    println!("{w1:?} {w2:?}");
}
```

戻り値を `Option<&'a str>` と書くことで、「返す単語は、**パーサ自身ではなく、元の入力と同じだけ生きる**」
という正確な関係を宣言しました。これで単語を集めながらパーサを使い続けられます。

**判断の問い**: メソッドが返す参照は、何を指していますか？

| 返す参照が指すもの | 戻り値に付ける lifetime |
| --- | --- |
| 構造体自身が所有するデータ（`String` フィールドなど） | `&self` の lifetime（省略でよい） |
| 構造体が借りている外部のデータ（`&'a str` フィールドの中身） | 構造体の `'a` |

</details>

## Deep Dive: 自己参照構造体は作れない

「`String` を所有しつつ、その一部を指す `&str` も同じ構造体に持つ」という設計は、
安全なRustでは素直には書けません。理由は2つあります。

- Rustの型システムには、「このフィールドは、同じ構造体のあのフィールドを借りている」という関係を表す手段が無い
- 仮に書けたとしても、`String` に `push` すればヒープが再確保され、`&str` が解放済みの領域を指しうる

このような場合は、`&str` の代わりに**位置（`Range<usize>`）を持つ**のが定石です。

```rust
struct Document {
    text: String,
    title: std::ops::Range<usize>, // text の中での位置
}

impl Document {
    fn title(&self) -> &str {
        &self.text[self.title.clone()]
    }
}

fn main() {
    let d = Document { text: String::from("Title\nbody"), title: 0..5 };
    println!("{}", d.title());
}
```

移動の問題は Chapter 11 の `Pin` でも再び登場します。

## Exercise

**`ex031_struct_with_reference`** — `cargo test -p ex031_struct_with_reference` で判定します。

`Parser<'a>` の `next_word(&mut self) -> Option<&'a str>` を実装します（シグネチャは用意済み）。
テストには、**取り出した単語を持ったまま次の単語を取り出す**ものが含まれています。
戻り値が `Option<&str>`（`&self` に結びつく）だったら、そのテストがなぜコンパイルできないか説明してください。

## Challenge

`Parser` を所有版（`input: String`）に書き換えるとしたら、`next_word` の戻り値はどうなりますか？
`String` を返す（毎回コピー）か、`&str` を返す（`&self` に結びつく）か、それぞれの使い勝手を比べてください。

## Review

- [ ] 構造体に参照を持たせるか所有させるかを、使い方から判断できる
- [ ] メソッドの戻り値を `&self` と構造体の `'a` のどちらに結びつけるか判断できる
- [ ] 自己参照構造体の代わりに、位置（`Range`）を持つ設計を選べる

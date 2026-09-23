# Lesson 16-1: 公開APIのレビュー

## Concept

APIの良し悪しは、**使う側の立場に立つ**と見えてきます。
自分で書いた関数を、「このライブラリを初めて使う人」になったつもりで呼んでみてください。

- 呼び出しのコードを読んで、何をしているか分かるか
- 間違った使い方をしたとき、コンパイラが止めてくれるか
- 失敗したとき、何が起きたか分かるか

## Why?

公開した後でAPIを変えると、利用者のコードを壊します（16-2）。
だから公開する**前**に、利用者の目でレビューすることが大切です。

## Bad Example: ありがちな問題がいくつも入ったAPI

テキストを解析するライブラリの公開関数だとします。

```rust
pub fn analyze(text: String, ignore_case: bool, skip_numbers: bool, mode: &str) -> Vec<(String, usize)> {
    let _ = (ignore_case, skip_numbers);
    if mode != "words" && mode != "chars" {
        panic!("unknown mode");
    }
    let _ = text;
    Vec::new()
}

fn main() {
    // 利用者のコード
    let result = analyze("Hello hello".to_string(), true, false, "words");
    let _ = result;
}
```

## Think

> **問い**: 利用者の立場で、このAPIの問題点をできるだけ挙げてください。
> これまでの章で学んだことと結びつけて考えてみてください。

<details>
<summary>Hint</summary>

呼び出し側の `analyze("Hello hello".to_string(), true, false, "words")` だけを見て、
`true` と `false` が何を意味するか分かりますか？ `"word"` と打ち間違えたら、いつ気づけますか？

</details>

<details>
<summary>Solution</summary>

| 問題 | 利用者にとって困ること | 関連する章 |
| --- | --- | --- |
| `text: String` | 読むだけなのに所有権を要求している。`&str` を持っている利用者は `.to_string()` でコピーさせられる | 01-3 |
| `bool` が2つ並んでいる | 呼び出しを読んでも `true, false` の意味が分からない。順番を間違えても気づけない | 02-1 |
| `mode: &str` | `"word"` と打ち間違えても、コンパイルが通る。実行して初めて panic する | 02-1, 03-1 |
| 不正な `mode` で `panic!` | 利用者は panic を回復できない。本当は「呼び出し側のバグ」ではなく、型で防げる間違い | 06-1, 13-3 |
| 戻り値が `Vec<(String, usize)>` | タプルの2番目が何の数か分からない。後から情報を足すと戻り値の型が変わり、利用者が壊れる | 16-2 |

改善した形は、例えばこうなります。

```rust
/// 解析の単位。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Words,
    Chars,
}

/// 解析の設定。既定値から、必要なものだけを変える。
#[derive(Debug, Clone)]
pub struct Options {
    unit: Unit,
    ignore_case: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { unit: Unit::Words, ignore_case: false }
    }
}

impl Options {
    pub fn unit(mut self, unit: Unit) -> Self {
        self.unit = unit;
        self
    }

    pub fn ignore_case(mut self, yes: bool) -> Self {
        self.ignore_case = yes;
        self
    }
}

/// 1つの項目と、その出現回数。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Count {
    pub item: String,
    pub occurrences: usize,
}

pub fn analyze(text: &str, options: &Options) -> Vec<Count> {
    let _ = (text, options);
    Vec::new()
}

fn main() {
    // 呼び出しを読むだけで、何をしているか分かる
    let options = Options::default().unit(Unit::Words).ignore_case(true);
    let result = analyze("Hello hello", &options);
    let _ = result;
}
```

| 変えたところ | 効果 |
| --- | --- |
| `&str` で受ける | 利用者はコピーせずに渡せる |
| `bool` の代わりに、名前付きのメソッド | `ignore_case(true)` と読めば意味が分かる |
| `mode: &str` を `enum Unit` に | 打ち間違いはコンパイルエラーになる。panic が不要になった |
| 設定を `Options` にまとめ、`Default` を実装 | 必要な設定だけを書けばよい。**後から設定を増やしても、利用者のコードは壊れない** |
| 戻り値を名前付きの `Count` 構造体に | `occurrences` という名前で意味が分かる |

この形は、`Options::default()` から始めて、メソッドをつなげて設定していくので、
**ビルダーパターン**と呼ばれることがあります。

</details>

## レビューの観点チェックリスト

公開APIを書いたら、次の観点で見直してください。多くは、これまでの章で学んだことです。

| 観点 | 問い |
| --- | --- |
| 引数の受け方 | 読むだけなのに所有権を要求していないか（01-3） |
| 不正な値 | 間違った値を、型で書けなくできないか（02, 03） |
| 意味の分からない引数 | `bool` や数値が並んでいないか（02-1） |
| 失敗の伝え方 | 回復できる失敗を panic にしていないか。`Err` の種類は区別できるか（06） |
| 公開範囲 | 内部の型や依存crateの型が、APIに漏れていないか（12-4） |
| 拡張性 | 後から情報や設定を足したとき、利用者が壊れないか（16-2） |

## Deep Dive: 自分で最初の利用者になる

APIの使いにくさに一番早く気づく方法は、**自分でそのAPIを使うコードを書いてみる**ことです。
Chapter 12-2 で `main.rs` から自分のライブラリを `use` したのも、13-2 で統合テストが「利用者と同じ立場」だったのも、
同じ理由です。**ドキュメントのコード例（16-3）を書くことも、利用者として使ってみる一番手軽な方法**です。
例を書いていて「長いな」「分かりにくいな」と感じたら、それはAPIを直すサインです。

## Exercise

**`ex059_api_review`** — `cargo test -p ex059_api_review` で判定します。

上の改善版（`Unit`・`Options`・`Count`）を使って、`analyze` を実装します。

| 設定 | 動作 |
| --- | --- |
| `Unit::Words` | 空白区切りの単語ごとに数える |
| `Unit::Chars` | 空白以外の文字ごとに数える |
| `ignore_case(true)` | 大文字・小文字を区別しない（小文字にそろえる） |

結果は、出現回数の多い順（同じ回数なら項目の辞書順）に並べます。

## Challenge

自分が最近書いた（またはAIに書かせた）公開関数を1つ選び、上のチェックリストで見直してください。
いくつ当てはまりましたか。

## Review

- [ ] 利用者の立場で、APIの問題点を指摘できる
- [ ] `bool` の並んだ引数を、名前付きの設定（`Options` など）に置き換えられる
- [ ] 文字列で表していた選択肢を、`enum` に置き換えられる
- [ ] レビューの観点を、これまでの章と結びつけて説明できる

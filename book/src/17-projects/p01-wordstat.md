# P1: wordstat — 単語を数える CLI ツール

## 何を作るか

テキストファイルを読み、よく出てくる単語を多い順に表示するコマンドです。

```text
$ wordstat --ignore-case --top 3 notes.txt
the	3
brown	1
dog	1
```

| 引数 | 意味 |
| --- | --- |
| `<file>` | 読み込むファイル（必須、1つだけ） |
| `--top N` | 上位 N 件を表示する（既定は 10） |
| `--ignore-case` | 大文字・小文字を区別しない |

うまくいかなかったときは、理由を表示して、**終了コード**で失敗の種類を伝えます
（終了コードは、プログラムが終わるときに OS に返す数字です。0 が成功で、それ以外が失敗を表します）。

| 状況 | 終了コード |
| --- | --- |
| 成功 | 0 |
| 引数が間違っている（知らないオプションなど） | 2 |
| ファイルが読めない | 1 |

## 使う章

| 章 | このプロジェクトでの使いどころ |
| --- | --- |
| 06 Error Handling | 失敗の種類を `CliError` enum で区別する |
| 12-2 lib / bin の分離 | ロジックを `lib.rs` に置き、`main.rs` を薄くする |
| 13-1 テスト可能な設計 | 入力を `impl Read` で受け取り、ファイルなしでテストする |
| 16-1 公開APIのレビュー | 結果をタプルではなく名前付きの `WordCount` で返す |

## 設計の問い

手を動かす前に、次のことを考えてみてください。

> **問い1**: `run` 関数は、ファイルのパスではなく `impl Read`（読み込めるもの）を受け取ります。なぜでしょうか。

<details>
<summary>考え方の例</summary>

ファイルのパスを受け取ると、`run` のテストのたびに**ファイルを用意する**必要があります。
`impl Read` を受け取れば、テストでは `"The cat".as_bytes()` のように**文字列をそのまま渡せます**。

ファイルを開くのは `main.rs` の仕事にしておけば、`run` は「読み込めるものから単語を数える」ことだけに集中できます。
Lesson 13-1 の「調べる部分と決める部分を分ける」と同じ考え方です。

</details>

> **問い2**: 終了コードを 1 と 2 に分けているのはなぜでしょうか。`main.rs` のどこでそれが決まっていますか。

<details>
<summary>考え方の例</summary>

このコマンドを**別のプログラムやシェルスクリプトから呼ぶ**場面を考えてください。
呼び出し側は、表示されたメッセージではなく、終了コードを見て「引数の間違い（直すのは呼び出し側）」か
「ファイルの問題（ファイルを確認すべき）」かを判断できます。

`lib.rs` は具体的な失敗の種類（`CliError`）を返し、`main.rs` がそれを見て終了コードを決めています。
Lesson 06-4 の「ライブラリは情報を保存し、アプリケーションの末端が表示して終わる」の形です。

</details>

> **問い3**: 実務では、引数の解析に `clap` という crate を使うことが多いです。このプロジェクトで自分で書くことに、どんな意味がありますか。

<details>
<summary>考え方の例</summary>

`clap` を使うと、ヘルプの表示・型への変換・エラーメッセージを自動で作ってくれるので、実務では使ったほうがよいです。

一方で、自分で `parse_args` を書くと、「知らないオプション」「値の無い `--top`」「2つ目のファイル名」など、
**引数の解析で起こりうる失敗の種類**を自分で数え上げることになります。
これは `clap` を使うときにも、「どんな間違いを想定すべきか」を判断する力になります。

</details>

## 進め方

テストを上から順に通していくと、無理なく進められます。

1. **`parse_args`**: 引数を1つずつ見て、`match` で分ける。`--top` のときは次の値も読む
2. **`normalize`**: 単語の前後の記号を取り除く（`trim_matches`）
3. **`count_words`**: 数える部分はできているので、**並べ替え**だけを書く（多い順、同じなら辞書順）
4. **`run`**: `count_words` の結果の上位 N 件を、`単語<TAB>回数` の行にする

最後に、CLI として動かしてみてください。

```bash
cd projects
echo "The cat and the dog. The END." > /tmp/sample.txt
cargo run -p p01_wordstat -- --ignore-case --top 2 /tmp/sample.txt
```

## ヒント: clippy の指摘から学ぶ

`run` で、各行を `format!` で作って `collect` すると、clippy が `format_collect` という指摘をします。
行ごとに小さな `String` を作ってから連結するより、**1つの `String` に `writeln!` で直接書き足す**ほうが無駄がない、
という指摘です。

```rust
use std::fmt::Write as _;

fn main() {
    let mut output = String::new();
    for (word, count) in [("the", 3), ("and", 1)] {
        // String への書き込みは失敗しないが、writeln! は Result を返すので扱っておく
        writeln!(output, "{word}\t{count}").expect("String への書き込みは失敗しない");
    }
    assert_eq!(output, "the\t3\nand\t1\n");
}
```

clippy は「動くけれど、もっと良い書き方がある」ことを教えてくれる道具です。
指摘を読んで理由を確かめるのは、Rust らしい書き方を身につける近道です。

## 判定

```bash
cd projects && cargo test -p p01_wordstat
```

コードは [projects/p01_wordstat](https://github.com/Kz1wg/rust-mastery/tree/main/projects/p01_wordstat) にあります。書き換えるのは `src/` の中で、判定に使うテストは `tests/tests.rs` です。

## 振り返り

- [ ] `lib.rs` と `main.rs` に何を置いたか、理由を説明できる
- [ ] 入力を `impl Read` で受け取ったことで、テストがどう楽になったか説明できる
- [ ] `CliError` の種類と終了コードの対応を説明できる
- [ ] 自分で書いた `parse_args` と `clap` を比べて、それぞれの利点を説明できる

# P7: ライブラリを作る（INI 設定ファイル）

## 何を作るか

INI 形式の設定ファイルを読む、小さな**ライブラリ**です。

```ini
; アプリ全体の設定
name = demo

[server]
host = 127.0.0.1
port = 8080
```

```rust,ignore
use p07_iniconf::Document;

let doc: Document = text.parse()?;
doc.get("server", "host");                            // Some("127.0.0.1")
let port: u16 = doc.get_parsed("server", "port")?;   // 8080（型を指定して取り出す）
doc.get("", "name");                                  // Some("demo")（見出しより前のキーは "" に入る）

for section in doc.sections() {                       // 書かれた順に
    println!("[{}] キーは {} 個", section.name(), section.len());
}
println!("{doc}");                                    // INI 形式で書き出す
```

これまでのプロジェクトとの違いは、**使う人が自分以外**だという前提で作ることです。
アルゴリズムは P2（CSV）より簡単ですが、そのぶん Chapter 16 の問い
「このAPIは利用者に何を保証しているか？」にじっくり向き合います。

## 使う章

| 章 | このプロジェクトでの使いどころ |
| --- | --- |
| 16-1 公開APIのレビュー | 利用者の立場で API を読む（下の「まずレビューから」） |
| 16-2 semver と拡張性 | `#[non_exhaustive]`、フィールドを非公開にする、内部の表現を隠す |
| 16-3 ドキュメントと例 | crate のドキュメントに書いた例が、そのままテストになる |
| 12-4 依存の方向と公開範囲 | `mod` を非公開にし、`pub use` で入口を1つにする（facade） |
| 08-2 構造体が参照を持つ設計 | `Section<'a>` は `Document` の中身を借りて見せる型 |
| 07-4 借用する iterator を返す | `sections()` と `entries()` が、コピーせずに中身を返す |
| 06-2 独自 Error 型 | `ParseError`（何行目で何が起きたか）と `GetError` |
| 05-1 generic にするメリット | `get_parsed::<T>` は `FromStr` を実装した型なら何でも受け取る |

## まずレビューから

手を動かす前に、Chapter 16 のやり方で「悪い API」をレビューしてみましょう。
AI に「INI を読む Rust のライブラリを書いて」と頼むと、次のようなコードが返ってくることがあります。

```rust,ignore
use std::collections::HashMap;

pub struct Ini {
    pub sections: HashMap<String, HashMap<String, String>>,
}

/// INI を解析する。形がおかしい行があれば panic する。
pub fn parse_ini(text: String) -> Ini { /* … */ }

/// 値を取り出す。見つからなければ空文字列を返す。
pub fn get(ini: &Ini, section: &str, key: &str) -> String { /* … */ }
```

> **問い0**: 利用者の立場で、この API の問題点をできるだけ挙げてください。

<details>
<summary>考え方の例</summary>

| 問題 | 利用者にとって困ること | このプロジェクトでの形 |
| --- | --- | --- |
| 形がおかしいと `panic!` | 設定ファイルは**利用者が書く**もの。書き間違いでアプリごと落ちる。何行目が悪いかも分からない | `Result<Document, ParseError>`。`line()` と `kind()` で原因が分かる |
| 見つからないと `""` | 「キーが無い」と「値が空」を区別できない | `Option<&str>` を返す |
| `text: String` | 読むだけなのに所有権を要求する | `&str` を受け取る（`FromStr` も実装） |
| 戻り値が `String` | 取り出すたびにコピーが起きる | `&str` を返す（`Document` から借りる） |
| `pub sections: HashMap<…>` | 内部の表現が API の一部になる。後で `Vec` に変えると利用者が壊れる。書かれた順番も失われる | フィールドは非公開。`Section` 型を通して見せる |
| 数値への変換が無い | 利用者が毎回 `parse` とエラー処理を書く | `get_parsed::<T>` |

この表は、Chapter 01・02・06・16 で学んだことの組み合わせです。
AI の出したコードが「動く」ことと、「ライブラリとして良い」ことは別だ、という感覚を持ってください。

</details>

## ファイルの構成

```text
p07_iniconf/src/
├── lib.rs        入口。mod は非公開にし、使ってほしい型だけを pub use する
├── document.rs   Document / Section と、内部の表現（SectionData）
├── error.rs      ParseError / ParseErrorKind / GetError（実装済み）
└── parse.rs      文字列から Document を組み立てる
```

利用者から見えるのは、crate の直下に並べた5つの型だけです。

```rust,ignore
use p07_iniconf::{Document, Section, ParseError, ParseErrorKind, GetError};
```

`document.rs` や `parse.rs` という分け方は、利用者には見えません。
だから、後でファイルを分け直しても、利用者の `use` は壊れません（Lesson 12-4）。

## 解析の規則

| 行の形（前後の空白を除いて） | 意味 | 失敗 |
| --- | --- | --- |
| 空行、`;` か `#` で始まる | コメント。無視する | |
| `[name]` | セクションの見出し。名前の前後の空白は除く | `]` が無い → `UnclosedSection`、名前が空 → `EmptySectionName`、同じ名前が2回目 → `DuplicateSection` |
| `key = value` | 最初の `=` で分ける。キーと値の前後の空白は除く。値は空でもよい | `=` が無い → `MissingEquals`、キーが空 → `EmptyKey`、同じセクションで同じキーが2回目 → `DuplicateKey` |

最初の見出しより前に書かれたキーは、名前が `""` の**ルートセクション**に入ります。

## 設計の問い

> **問い1**: `doc.section("server")` は、内部の `SectionData` への参照ではなく、`Section<'_>` という別の型を返します。なぜでしょうか。

<details>
<summary>考え方の例</summary>

`SectionData` は「キーと値の `Vec`」という**内部の表現**そのものです。
これを `&SectionData` として公開すると、利用者は `.entries` の `Vec` を直接触れるようになり、
表現を変えた瞬間に利用者のコードが壊れます。

`Section<'a>` は、`&'a SectionData` を1つ持つだけの小さな型です。
利用者には `name()` / `get()` / `entries()` / `len()` しか見えないので、
中身が `Vec` から `HashMap` に変わっても、`Section` のメソッドが同じ結果を返す限り、誰も壊れません。

もう1つのポイントは、`Section::get` の戻り値が `Option<&'a str>` になっていることです。

```rust,ignore
pub fn get(&self, key: &str) -> Option<&'a str>
//          ^^^^^                         ^^
//          Section 自体を借りる期間      Document を借りている期間
```

省略の規則（Lesson 08-3）に任せると、戻り値は `&self`（`Section` の値）の寿命に結びつきます。
`'a` と明示すると、「`Section` の値が消えても、`Document` が生きている限り使える」と言えます。
テストの `values_borrowed_from_section_outlive_the_section_value` が、この約束を確かめています。

</details>

> **問い2**: `ParseError` のフィールド（`line` と `kind`）は非公開で、`line()` と `kind()` というメソッドで読みます。
> `pub line: usize` にしない理由は何でしょうか。

<details>
<summary>考え方の例</summary>

| 形 | 後から列番号（`column`）を足すと |
| --- | --- |
| `pub struct ParseError { pub line: usize, pub kind: … }` | 利用者が `ParseError { line, kind }` と書いて作ったり、分解したりしているかもしれない。フィールドを足すと、そのコードが壊れる（破壊的変更） |
| フィールド非公開 + メソッド | 利用者は作れず、メソッドで読むだけ。`column()` を足しても誰も壊れない |

エラーを作るのはライブラリの中だけなので、`new` は `pub(crate)` にしてあります。
利用者に「読む」ことだけを許し、「作る」「分解する」ことは許さない、という設計です。

同じ理由で、`ParseErrorKind` と `GetError` には `#[non_exhaustive]` が付いています（Lesson 03-4）。
crate のドキュメントに、`_` の腕が無い `match` が**コンパイルできない**ことを確かめる例（`compile_fail`）を入れてあります。

</details>

> **問い3**: 同じセクションに同じキーが2回出てきたとき、このライブラリはエラーにします。
> 「後に書いたほうを使う」という選択肢もあります。どちらが良いでしょうか。

<details>
<summary>考え方の例</summary>

| 選択 | 良い点 | 困る点 |
| --- | --- | --- |
| エラーにする（この実装） | 書き間違い（同じキーをうっかり2回書いた）に気づける | 「上書きのつもりで書いた」使い方ができない |
| 後に書いたほうを使う | 寛容。一部の INI の実装と同じ動き | 間違いに気づけず、「設定したのに効かない」原因になる |

もう1つの観点は、**後から変えやすいのはどちらか**です。

- 「エラー」から「後勝ち」に変える: 今までエラーだった入力が通るようになるだけ。困る利用者はほぼいない
- 「後勝ち」から「エラー」に変える: 今まで動いていた設定ファイルが、突然読めなくなる

迷ったら**厳しいほう**から始めると、後で緩めることができます。逆は難しいです（Lesson 16-2）。

</details>

> **問い4**: `get_parsed` は `Option<T>` ではなく `Result<T, GetError>` を返し、`GetError` は「見つからない」と「変換できない」を分けています。なぜでしょうか。

<details>
<summary>考え方の例</summary>

`port = abc` と書かれていたとき、`Option<u16>` だと `None` になり、「書き忘れ」と区別できません。
利用者がエラーメッセージを出すとき、「`port` がありません」と「`port` の値 `abc` を数にできません」では、
直し方がまったく違います。

`GetError` はどちらの場合も、セクション名・キー（と値）を持っています。
エラーメッセージだけで、**設定ファイルのどこを直せばよいか**分かるようにするためです（Lesson 06-2）。

「無くてもよい設定」を扱いたい利用者は、次のように書けます。

```rust,ignore
let timeout: u64 = match doc.get_parsed("server", "timeout") {
    Ok(v) => v,
    Err(GetError::Missing { .. }) => 30,   // 書いていなければ既定値
    Err(e) => return Err(e.into()),        // 書いてあるのに変換できないのは、エラー
};
```

</details>

> **問い5**: この crate のテストには、`assert_send_sync::<Document>()` のように「型が `Send + Sync` であること」を確かめるものがあります。
> 何のためでしょうか。

<details>
<summary>考え方の例</summary>

今の `Document` は、`String` と `Vec` だけでできているので、自動的に `Send + Sync` です（Lesson 10-1）。
しかし将来、誰かが内部に `Rc` や `RefCell` を足すと、**何の警告もなく** `Send + Sync` でなくなります。
利用者の「設定を `Arc` で包んでスレッド間で共有する」コードは、そこで壊れます。

`Send + Sync` であることは、型の定義のどこにも書かれていない、**暗黙の約束**です。
テストで確かめておくと、その約束を破る変更をした時点で、ライブラリの作者が気づけます。

`errors_work_with_question_mark_into_box_dyn_error` も同じ考え方で、
「アプリケーションでよく使う `Box<dyn Error + Send + Sync>` に `?` で変換できる」ことを固定しています（Lesson 06-4）。

</details>

## 進め方

`error.rs` と `lib.rs` は実装済みです。まず読んで、利用者に何が見えているかを確かめてください。

1. **`parse.rs` の `parse`**: 解析の規則の表どおりに、1行ずつ処理する
   - `line.strip_prefix('[')` と `strip_suffix(']')` で見出しを、`split_once('=')` でキーと値を分ける
   - 失敗は `ParseError::new(行番号, ParseErrorKind::…)`。行番号は 1 から数える
2. **`Document::section` / `sections`**: `self.sections` から探す・先頭（ルート）を `skip(1)` する
3. **`Section::get` / `entries`**: `SectionData` の中身を `&'a str` で返す
4. **`Document::get` / `get_parsed`**: `section` と `Section::get` を組み合わせる。`get_parsed` は `ok_or_else` と `map_err` で `GetError` を作る
5. **`Display`**: 書き出しの規則（ソースのコメント）どおりに。最後に「書き出して読み直すと元に戻る」テストが通ることを確かめる

## 判定

```bash
cd projects && cargo test -p p07_iniconf
```

crate のドキュメントに書いた例（doctest）も、判定に含まれます。
ドキュメントの例が古くなって動かなくなると、テストが失敗して気づけます（Lesson 16-3）。

```bash
cargo doc -p p07_iniconf --open   # 利用者に見えるドキュメントを確かめる
```

## Challenge

- `Document::get_or(section, key, default)` を足すとしたら、どんなシグネチャが良いでしょうか。`default` は `&str` と `T` のどちらで受けるべきでしょうか
- 値の前後の `"` を取り除く（`name = "hello world"`）機能を足すとき、これは既存の利用者にとって破壊的変更でしょうか。バージョン番号はどう上げるべきでしょうか（Lesson 16-2）
- `ParseErrorKind` に新しい種類（例: キーに空白が含まれる）を足してみましょう。`#[non_exhaustive]` のおかげで、どのコードが壊れずに済むでしょうか

## 振り返り

- [ ] 「動く API」と「ライブラリとして良い API」の違いを、具体的な問題点で説明できる
- [ ] 内部の表現を隠し、借用する型（`Section<'a>`）を通して見せる理由を説明できる
- [ ] フィールドの非公開・`#[non_exhaustive]`・`pub(crate)` を、将来の変更に備える道具として使える
- [ ] エラーの種類を、利用者が「どう直せばよいか」で分けられる
- [ ] 型の暗黙の約束（`Send + Sync` など）をテストで固定できる

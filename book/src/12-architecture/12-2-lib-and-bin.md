# Lesson 12-2: lib / bin の分離

## Concept

`main.rs` に書いたコードは、**統合テストから呼べません**。
テストできるのは `lib.rs`（ライブラリターゲット）の公開APIだけです。
だから `main.rs` は薄く保ち、ロジックは `lib.rs` に置きます。

## Why?

CLIツールを書くとき、`main` に全部書くのが一番早く感じます。
しかしテストを書こうとした瞬間、「プロセスを起動して標準出力を比較する」しかなくなります。

## Bad Example

```rust,ignore
// src/main.rs に全部書いた場合
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];                       // 引数の解析
    let content = std::fs::read_to_string(path).unwrap(); // I/O
    let count = content.split_whitespace().count();        // ロジック
    println!("{count} words");                             // 出力
}
```

## Problem

引数の解析・ファイルI/O・単語数の計算・出力が、1つの関数に混ざっています。
**単語数の計算だけをテストする方法がありません**。`unwrap()` も、失敗時にユーザーへ何も説明しません。

## Think

> **問い**: この処理を分けるとしたら、どこに線を引きますか。
> 「テストしたい部分」と「テストしなくてよい部分」で考えてください。

<details>
<summary>Solution</summary>

```rust,ignore
// src/lib.rs — ロジック。テストできる
pub fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

pub fn run(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(format!("{} words", count_words(&content)))
}
```

```rust,ignore
// src/main.rs — 薄い。引数を読み、run を呼び、エラーを表示して終了コードを決めるだけ
use my_tool::run;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some(path) = args.get(1) else {
        eprintln!("usage: my_tool <path>");
        std::process::exit(2);
    };

    match run(path) {
        Ok(message) => println!("{message}"),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
```

| 置き場所 | 内容 | テスト |
| --- | --- | --- |
| `lib.rs` | ロジック、エラー型、I/Oを含む処理 | 統合テスト・単体テストで直接呼べる |
| `main.rs` | 引数の取得、`run` の呼び出し、終了コードの決定、エラー表示 | ほぼ不要（薄いので） |

`count_words` のような**純粋な関数**は、ファイルが無くてもテストできます。
`run` のようなI/Oを含む関数は、一時ファイルを作ればテストできます。
`main` に残るのは「テストしても得るものが少ない部分」だけです。

エラーの扱いが Chapter 06-4 の「ライブラリとアプリケーションの境界」と一致していることにも注目してください。
**`lib.rs` が具体的なエラーを返し、`main.rs` が表示して終了コードを決めます。**

</details>

## Deep Dive: 1つのパッケージに lib と bin を両方置く

```text
my_tool/
├── Cargo.toml
└── src/
    ├── lib.rs      # ライブラリターゲット（パッケージ名がcrate名）
    └── main.rs     # バイナリターゲット。lib を use して使う
```

Cargo はこの配置を自動で認識します。`main.rs` からは、自分のライブラリを
**外部crateと同じように** `use my_tool::run;` で使います。
これは「自分のライブラリの公開APIを、自分で最初に使ってみる」ことでもあり、
APIの使いにくさに気づく機会になります（Chapter 16）。

バイナリを複数置きたい場合は `src/bin/foo.rs` を追加します。

## Exercise

**`ex047_lib_and_bin`** — `cargo test -p ex047_lib_and_bin` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `count_words` | 空白区切りの単語数を数える純粋な関数 |
| `run(path)` | ファイルを読み、`"N words"` を返す。失敗は `Result` で返す |
| `src/main.rs` | 引数を読み、`run` を呼び、エラーなら終了コード1で終わる（テスト対象外） |

テストは `lib.rs` の公開APIだけを呼びます。`main.rs` にロジックが漏れていると、テストできません。

## Challenge

`main.rs` にロジックを書いたまま、その部分をテストする方法を考えてみてください。
（`std::process::Command` でバイナリを起動する方法があります。どれだけ面倒か試すと、分離の価値が分かります。）

## Review

- [ ] `main.rs` のコードが統合テストから呼べない理由を説明できる
- [ ] ロジックを `lib.rs` に置き、`main.rs` を薄く保つ構造を作れる
- [ ] エラー設計が Chapter 06-4 の lib / app の境界と対応していることを説明できる

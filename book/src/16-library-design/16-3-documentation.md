# Lesson 16-3: ドキュメントと例

## Concept

Rust のドキュメントは、コードの中の `///` コメントから `cargo doc` で自動生成されます。
そしてコメントの中に書いたコード例は、**`cargo test` で実際に実行されます**（Lesson 13-2 のドキュメントテスト）。

つまり Rust では、**ドキュメントのコード例がそのまま「動くことが保証された仕様書」になります**。

## Why?

利用者がライブラリを使うとき、最初に読むのはドキュメントです。
特に**コード例**は、説明文よりも先に読まれ、そのままコピーされます。
コード例が古くて動かないと、利用者はそこでつまずきます。

## 標準的な構成

Rust のドキュメントには、よく使われる見出しがあります。

```rust
/// 2つの数を割る。
///
/// 割り算の結果を返します。0 で割ろうとした場合はエラーになります。
///
/// # Examples
///
/// ```
/// # fn divide(a: i32, b: i32) -> Result<i32, String> {
/// #     if b == 0 { return Err("division by zero".into()); }
/// #     Ok(a / b)
/// # }
/// assert_eq!(divide(10, 2), Ok(5));
/// ```
///
/// # Errors
///
/// `b` が 0 のとき、`Err` を返します。
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("division by zero".into());
    }
    Ok(a / b)
}

fn main() {
    assert_eq!(divide(10, 2), Ok(5));
}
```

| 見出し | 書くこと | いつ書くか |
| --- | --- | --- |
| （最初の1行） | 何をする関数か、短く1文で | 常に。一覧ページにはこの1行だけが表示される |
| `# Examples` | 動くコード例 | 公開する関数・型には、できるだけ書く |
| `# Errors` | どんなときに、どんな `Err` を返すか | `Result` を返す関数 |
| `# Panics` | どんなときに panic するか | panic しうる関数 |
| `# Safety` | 呼ぶ側が守るべき条件 | `unsafe fn`（Lesson 15-1） |

コード例の中の `# ` で始まる行は、**ドキュメントには表示されないが、テストでは実行される**行です。
例を短く見せたいときに、準備のコードを隠すのに使います。

## Bad Example: 説明はあるが、利用者の疑問に答えていない

```rust
/// 文字列を解析する。
pub fn parse_duration(s: &str) -> Result<u64, String> {
    let _ = s;
    Ok(0)
}

fn main() {}
```

## Think

> **問い**: このドキュメントを読んだ利用者は、どんな疑問を持つでしょうか。
> 疑問を全部書き出してから、それに答えるドキュメントを書いてみてください。

<details>
<summary>Solution</summary>

利用者が持ちそうな疑問は、例えば次のとおりです。

- どんな形式の文字列を受け付けるのか？（`"90s"`？ `"1h30m"`？ `"1:30"`？）
- 戻り値の `u64` は何の単位か？（秒？ ミリ秒？）
- どんなときに `Err` になるのか？ `Err` の中身の文字列で判断してよいのか？
- 空文字列を渡したらどうなるのか？

これに答えると、こうなります。

```rust
/// `"1h30m"` のような時間の文字列を、秒数に変換する。
///
/// 使える単位は `h`（時間）・`m`（分）・`s`（秒）で、組み合わせられます。
/// 単位は大きい順に並べる必要はありません。
///
/// # Examples
///
/// ```
/// # fn parse_duration(s: &str) -> Result<u64, String> {
/// #     let mut total = 0u64; let mut num = String::new();
/// #     if s.is_empty() { return Err("empty".into()); }
/// #     for c in s.chars() {
/// #         if c.is_ascii_digit() { num.push(c); continue; }
/// #         let n: u64 = num.parse().map_err(|_| "missing number".to_string())?;
/// #         num.clear();
/// #         total += match c { 'h' => n * 3600, 'm' => n * 60, 's' => n, _ => return Err("bad unit".into()) };
/// #     }
/// #     if !num.is_empty() { return Err("missing unit".into()); }
/// #     Ok(total)
/// # }
/// assert_eq!(parse_duration("90s"), Ok(90));
/// assert_eq!(parse_duration("1h30m"), Ok(5400));
/// ```
///
/// # Errors
///
/// 次の場合に `Err` を返します。
///
/// - 空文字列
/// - 数字の後に単位が無い（`"90"`）
/// - 知らない単位がある（`"5d"`）
pub fn parse_duration(s: &str) -> Result<u64, String> {
    let _ = s;
    Ok(0)
}

fn main() {}
```

最初の1行で**入力と出力**（時間の文字列 → 秒数）が分かり、例で**形式**が分かり、`# Errors` で**失敗の条件**が分かります。

エラーを `String` で返していることに気づいたでしょうか。ドキュメントに失敗の種類を3つ書いたなら、
それは Lesson 06-1 の「区別したい失敗は enum にする」の出番かもしれません。
**ドキュメントを書くと、APIの改善点が見えてくる**、という良い例です。

</details>

## ドキュメントの抜けを防ぐ

crate の先頭に次の1行を書くと、**ドキュメントの無い公開項目があるとコンパイルエラー**になります。

```rust,ignore
#![deny(missing_docs)]
```

ライブラリの利用者から見える項目すべてに、説明を書くことを強制できます。
（`deny` ではなく `warn` にすると、エラーではなく警告になります。）

## Deep Dive: 生成されたドキュメントを見る

```text
cargo doc --open
```

を実行すると、ブラウザで自分のライブラリのドキュメントが開きます。
標準ライブラリや crates.io のライブラリと同じ見た目のページが、自分の `///` コメントから作られます。

実際に開いて、**「このライブラリを初めて使う人」の気持ちで読んでみてください**。
最初の1行だけが並ぶ一覧ページで、どの関数を使えばよいか分かるでしょうか。

## Exercise

**[`ex061_documentation`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex061_documentation)** — `cargo test -p ex061_documentation` で判定します。

この演習のテストの中心は、**ドキュメントの中のコード例**（ドキュメントテスト）です。
`parse_duration` の `# Examples` が仕様書になっていて、実装するとそれが通るようになります。

| 項目 | 内容 |
| --- | --- |
| `parse_duration(s)` | `"1h30m"` のような文字列を秒数に変換する。失敗の種類は `DurationError` で返す |
| `#![deny(missing_docs)]` | crate に設定済み。ドキュメントの無い公開項目を足すとコンパイルできない |

## Challenge

`cargo doc --open` で `ex061` のドキュメントを開き、`parse_duration` のページを読んでください。
`DurationError` の各バリアントに、「どんなときにそのエラーになるか」が書かれているか確認し、
足りなければ自分で書き足してください。

## Review

- [ ] `///` コメントからドキュメントが生成され、コード例がテストとして実行されることを説明できる
- [ ] `# Examples` / `# Errors` / `# Panics` / `# Safety` を使い分けられる
- [ ] コード例の `# ` 行で、準備のコードを隠せる
- [ ] `#![deny(missing_docs)]` で、ドキュメントの抜けを防げる

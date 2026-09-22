# Lesson 06-4: libraryとapplicationのerror設計

## Concept

エラー設計の指針は、**そのコードが「ライブラリ」か「アプリケーション」か**で変わります。

- **ライブラリ**: 呼び出し側が誰で、何をしたいのか分からない。エラーの情報を**できるだけ保持**し、
  利用者が自分で判断できるようにする。
- **アプリケーション**（CLIツールの`main`など）: 呼び出し側はもう存在しない。
  エラーは**最終的にユーザーに見せる・ログに出す**のが仕事になる。

## Why?

この違いを意識しないと、ライブラリなのに情報を握りつぶしたり、
逆にアプリケーションの末端で過剰に構造化されたエラー型を作ってしまったりします。

## Bad Example: ライブラリが情報を握りつぶす

```rust
// これは「ライブラリ」のコードだと想定する
pub fn load_config(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|_| "設定の読み込みに失敗しました".to_string())
}
```

## Problem

このライブラリを使う側は、「ファイルが存在しない」のか「権限が無い」のか「他の理由か」を
区別できません。ライブラリの作者は、**利用者が何をしたいか知らない**にもかかわらず、
先回りして情報を捨ててしまっています。

## Think

> **問い**: ライブラリとして公開するなら、`Err` 型をどう設計しますか？
> 「利用者が何をするか分からない」という前提で考えてください。

<details>
<summary>Solution</summary>

```rust
use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    NotFound(std::io::Error),
    PermissionDenied(std::io::Error),
    Other(std::io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NotFound(_) => write!(f, "設定ファイルが見つかりません"),
            ConfigError::PermissionDenied(_) => write!(f, "設定ファイルへのアクセス権がありません"),
            ConfigError::Other(e) => write!(f, "設定の読み込みに失敗しました: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

pub fn load_config(path: &str) -> Result<String, ConfigError> {
    std::fs::read_to_string(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => ConfigError::NotFound(e),
        std::io::ErrorKind::PermissionDenied => ConfigError::PermissionDenied(e),
        _ => ConfigError::Other(e),
    })
}

fn main() {}
```

利用者（アプリケーション側）は、`ConfigError::NotFound` のときだけ
「デフォルト設定を作成する」といった対応ができます。**ライブラリの役目は、
利用者が判断に使える情報を保存しておくことまでです。利用者がその情報をどう使うかは、
ライブラリの関知するところではありません。**

</details>

## アプリケーション側: 最終的には「表示して終わり」でよい

```rust,no_run
# use std::fmt;
# #[derive(Debug)]
# pub enum ConfigError {
#     NotFound(std::io::Error),
#     Other(std::io::Error),
# }
# impl fmt::Display for ConfigError {
#     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
#         match self {
#             ConfigError::NotFound(_) => write!(f, "設定ファイルが見つかりません"),
#             ConfigError::Other(e) => write!(f, "設定の読み込みに失敗しました: {e}"),
#         }
#     }
# }
# impl std::error::Error for ConfigError {}
# pub fn load_config(path: &str) -> Result<String, ConfigError> {
#     std::fs::read_to_string(path).map_err(|e| match e.kind() {
#         std::io::ErrorKind::NotFound => ConfigError::NotFound(e),
#         _ => ConfigError::Other(e),
#     })
# }
// load_config は上の Solution の「ライブラリ」側の関数（ここでは表示を省略）
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ConfigError は std::error::Error を実装しているので、
    // ? で Box<dyn Error> へ自動変換される。main はそれを表示して終了するだけでよい。
    let config = load_config("config.toml")?;
    println!("{config}");
    Ok(())
}
```

`main` の `Err` 型を `Box<dyn Error>` にしておけば、**ライブラリ側がどんな具体的な
`Error` 型を返してきても、まとめて受け止められます**。
これは06-2で見た「呼び出し側が種類によって処理を変えないなら `Box<dyn Error>` でよい」
という基準に対応しています——`main` の末端は、まさにその状況です。

## 判断のまとめ

| 立場 | 責任 | 選ぶ傾向 |
| --- | --- | --- |
| ライブラリ | 利用者が判断に使える情報を保存する | 具体的な `enum Error`。むやみに `Box<dyn Error>` にして情報を捨てない |
| アプリケーションの末端（`main` など） | 最終的にユーザーへ表示・ログ出力する | `Box<dyn Error>` で十分なことが多い。途中の層では、必要なら具体的な型を保ってよい |
| アプリケーションの中間層 | 状況によって異なる対応をする箇所 | ライブラリと同じ基準（06-1）で、区別が必要なら `enum` を保つ |

**「アプリケーションだから常に `Box<dyn Error>` でよい」わけではありません。**
アプリケーションの中でも、「この結果によって処理を分岐する」箇所は、
06-1の基準（呼び出し側が区別したいか）に従って `enum` を使うべきです。
「末端（最終的に人間に見せるだけの場所）」だけが `Box<dyn Error>` で十分になります。

## Exercise

**`ex025_library_vs_application`** — `cargo test -p ex025_library_vs_application` で判定します。

「ライブラリ」の関数として `ConfigError` の `enum` を設計し、
「アプリケーション」側の `main` 相当の関数で `Box<dyn Error>` に集約します。

## Challenge

自分の書いたコードで、「ライブラリ的な部分」と「アプリケーションの末端」を区別してみてください。
それぞれのエラー型は、この章の基準に沿っているでしょうか。

## Review

- [ ] ライブラリとアプリケーションで、エラー設計の責任がどう違うか説明できる
- [ ] 「アプリケーションだから常に `Box<dyn Error>`」ではないことを説明できる
- [ ] ライブラリの `Error` 型が `std::error::Error` を実装していると、利用側で `Box<dyn Error>` に
      自動変換できることを説明できる

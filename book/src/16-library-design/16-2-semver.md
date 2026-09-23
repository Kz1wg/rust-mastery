# Lesson 16-2: semver と拡張性

## Concept

Rust のライブラリは、**semver（セマンティックバージョニング）**というルールでバージョンを付けます。
`1.4.2` のような3つの数字に、それぞれ意味があります。

| 位置 | 名前 | 上げるとき |
| --- | --- | --- |
| `1`.4.2 | メジャー | **利用者のコードを壊す変更**をしたとき |
| 1.`4`.2 | マイナー | 壊さずに**機能を追加**したとき |
| 1.4.`2` | パッチ | 壊さずに**バグを直した**とき |

利用者が `Cargo.toml` に `mylib = "1.4"` と書くと、Cargo は `1.x.y`（`1.4.0` 以上、`2.0.0` 未満）の最新版を選びます。
つまり**マイナー・パッチの更新は、利用者が何もしなくても自動で入ってきます**。
そこに壊れる変更が紛れ込むと、利用者はある日突然ビルドできなくなります。

> 補足: `0.x` のバージョンでは、真ん中の数字（`0.4` → `0.5`）が「壊す変更」の意味になります。
> まだAPIが固まっていない段階、という扱いです。

## Why?

「この変更は壊すのか、壊さないのか」の判断は、意外と難しいものです。
**一見、機能を足しただけに見える変更が、利用者を壊す**ことがあります。

## Bad Example 1: pub フィールドを足した

ライブラリの `Config` に、あとから `retries` フィールドを足したとします。

```rust,compile_fail,E0063
mod mylib {
    pub struct Config {
        pub path: String,
        pub retries: u8, // 新しく追加した
    }
}

fn main() {
    // 利用者が以前から書いていたコード
    let _c = mylib::Config { path: String::new() };
}
```

「フィールドを足しただけ」ですが、利用者の構造体リテラルが**フィールド不足**でコンパイルエラーになります。
フィールドを全部 `pub` にした構造体は、**フィールドを1つ足すだけで破壊的変更**になるのです。

## Bad Example 2: trait にメソッドを足した

```rust,compile_fail,E0046
mod mylib {
    pub trait Store {
        fn get(&self) -> u32;
        fn put(&mut self, value: u32); // 新しく追加した
    }
}

// 利用者が以前から書いていた実装
struct MyStore;

impl mylib::Store for MyStore {
    fn get(&self) -> u32 {
        0
    }
}

fn main() {}
```

利用者が `Store` を実装していた場合、新しいメソッド `put` の実装が無いので、コンパイルエラーになります。

## Think

> **問い**: この2つの変更を、**利用者を壊さずに**行うにはどうすればよかったでしょうか。

<details>
<summary>Solution</summary>

**1. 構造体: 最初から、フィールドを直接作らせない**

Lesson 03-4 の `#[non_exhaustive]` と、Lesson 16-1 の `Default` + ビルダーの組み合わせです。

```rust
mod mylib {
    #[non_exhaustive]
    #[derive(Debug, Default)]
    pub struct Config {
        pub path: String,
        pub retries: u8, // 後から足しても、利用者は壊れない
    }

    impl Config {
        pub fn new(path: &str) -> Self {
            Config { path: path.to_string(), ..Default::default() }
        }
    }
}

fn main() {
    let c = mylib::Config::new("/tmp/app");
    assert_eq!(c.retries, 0);
}
```

`#[non_exhaustive]` を付けると、**他の crate からは構造体リテラルで作れなくなります**（Lesson 03-4）。
利用者は必ず `Config::new` などを通るので、フィールドを足しても壊れません。
（フィールドを private にして、読み取り用のメソッドを用意する方法でも同じ効果があります。）

**2. trait: デフォルト実装を付けて追加する**

```rust
mod mylib {
    pub trait Store {
        fn get(&self) -> u32;

        // 新しく追加したメソッドには、デフォルトの実装を付ける
        fn describe(&self) -> String {
            format!("value = {}", self.get())
        }
    }
}

struct MyStore;

impl mylib::Store for MyStore {
    fn get(&self) -> u32 {
        7
    }
}

fn main() {
    use mylib::Store;
    assert_eq!(MyStore.describe(), "value = 7"); // 利用者の impl は変えなくてよい
}
```

デフォルト実装があれば、利用者の既存の `impl` はそのままコンパイルできます。
（デフォルト実装を書けないメソッドを足したいなら、それはメジャーバージョンを上げるべき変更です。
あるいは、利用者に実装させる必要がないなら、Lesson 09-3 の sealed trait にしておく方法もあります。）

</details>

## 壊す変更・壊さない変更の早見表

| 変更 | 利用者を壊すか | 理由・対策 |
| --- | --- | --- |
| 公開している関数・型を消す、名前を変える | **壊す** | 呼んでいる所が全部エラーになる |
| 関数の引数を増やす | **壊す** | 既存の呼び出しが引数不足になる。新しい関数を足すか、設定を `Options` に入れる（16-1） |
| 引数の型を広げる（`&String` → `&str`） | 壊さない | 既存の呼び出しはそのまま通る |
| 戻り値の型を変える | **壊す** | 受け取っている側の型が合わなくなる |
| `pub` フィールドを足す（`#[non_exhaustive]` なし） | **壊す** | 構造体リテラルがフィールド不足になる |
| `enum` にバリアントを足す（`#[non_exhaustive]` なし） | **壊す** | 利用者の網羅的な `match` がエラーになる（03-4） |
| trait にメソッドを足す（デフォルト実装なし） | **壊す** | 利用者の `impl` が不足する |
| trait にメソッドを足す（デフォルト実装あり） | ほぼ壊さない | まれに、利用者の型の同名メソッドと名前がぶつかる |
| 新しい関数・型を足す | 壊さない | ただし `use mylib::*;` している利用者とは、名前がぶつかることがある |
| 依存crateの型を公開APIに出していて、その依存をメジャー更新する | **壊す** | 利用者から見ると、公開APIの型が変わる（12-4） |

表の最後の項目は見落とされがちです。Lesson 12-4 の「依存crateの型を公開APIに出さない」は、
このためでもあります。

## Deep Dive: 壊す変更を機械で見つける

「この変更が壊すかどうか」を人間が毎回判断するのは大変です。
`cargo-semver-checks` というツールを使うと、**前のバージョンと比べて、壊す変更が入っていないか**を自動で調べられます。

```text
cargo install cargo-semver-checks
cargo semver-checks
```

ライブラリを公開するなら、リリース前やCIで実行しておくと安心です。

## Exercise

**`ex060_semver_friendly`** — `cargo test -p ex060_semver_friendly` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `Config`（`#[non_exhaustive]`） | `Config::new(path)` と、`with_retries` などのメソッドで設定する |
| `Level`（`#[non_exhaustive]` な enum） | `label()` で表示名を返す |
| `Store` trait | 必須メソッド `get` と、デフォルト実装付きの `describe` |

`compile_fail` doctest で、利用者（別crate）が `Config` を構造体リテラルで作れないこと、
`Level` の `match` にワイルドカードが必要なことを確かめています。

## Challenge

`Store` trait に、デフォルト実装の無いメソッドを1つ足してみてください。どのテストが壊れますか。
それは、実際の利用者に何が起きるかと同じです。

## Review

- [ ] semver の3つの数字の意味と、Cargo が自動で更新を取り込む範囲を説明できる
- [ ] `pub` フィールドや trait メソッドの追加が、なぜ利用者を壊すのか説明できる
- [ ] `#[non_exhaustive]` やデフォルト実装を使って、壊さずに拡張できる形を作れる
- [ ] 壊す変更・壊さない変更を、早見表を使って判断できる

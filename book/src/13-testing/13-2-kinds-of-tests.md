# Lesson 13-2: unit / integration / doctest

## Concept

Rust には、置き場所の違う3種類のテストがあります。どれも `cargo test` でまとめて実行されますが、
**見えるもの（呼べる関数の範囲）が違う**ので、役割も違います。

| 種類 | 置き場所 | 見えるもの | 役割 |
| --- | --- | --- | --- |
| 単体テスト（unit test） | `src/` の中、`#[cfg(test)] mod tests` | **private な関数も含めて全部** | 内部の部品を1つずつ確かめる |
| 統合テスト（integration test） | `tests/` ディレクトリ | **`pub` なものだけ** | 利用者と同じ立場で、公開APIを確かめる |
| ドキュメントテスト（doctest） | `///` コメントの中のコード例 | **`pub` なものだけ** | 「ドキュメントの例が本当に動く」ことを保証する |

## Why?

「テストを書こう」と思ったとき、どこに書けばよいか迷うことがあります。
置き場所によって**何を確かめられるか**が決まるので、目的に合わせて選ぶ必要があります。

## 3種類を1つのcrateで見てみる

```rust,ignore
// src/lib.rs

/// 2つの数を割る。0 で割ると Err を返す。
///
/// ```
/// assert_eq!(my_crate::divide(10, 2), Ok(5));       // ← これがドキュメントテスト
/// assert!(my_crate::divide(1, 0).is_err());
/// ```
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("division by zero".to_string());
    }
    Ok(a / b)
}

fn double(x: i32) -> i32 {        // private（外からは見えない）
    x * 2
}

pub fn double_plus_one(x: i32) -> i32 {
    double(x) + 1
}

#[cfg(test)]                      // テストのときだけコンパイルされる
mod tests {
    use super::*;                 // 親モジュールの private な関数も見える

    #[test]
    fn double_works() {           // ← これが単体テスト
        assert_eq!(double(3), 6);
    }
}
```

```rust,ignore
// tests/api.rs

#[test]
fn double_plus_one_works() {      // ← これが統合テスト
    assert_eq!(my_crate::double_plus_one(3), 7);
}
```

`cargo test` を実行すると、3種類がこの順に実行されます。

```text
running 1 test          ← 単体テスト（src/lib.rs の mod tests）
test tests::double_works ... ok

running 1 test          ← 統合テスト（tests/api.rs）
test double_plus_one_works ... ok

running 1 test          ← ドキュメントテスト
test src/lib.rs - divide (line 5) ... ok
```

## Think

> **問い**: 統合テスト（`tests/api.rs`）から、private な `double` を呼ぶとどうなりますか？
> そして、それは困ったことでしょうか？

<details>
<summary>Solution</summary>

コンパイルエラーになります（`error[E0603]: function 'double' is private`）。

統合テストは、`tests/` の中で**別のcrateとして**コンパイルされます。
つまり、あなたのライブラリを使う**利用者と全く同じ立場**です。利用者から見えないものは、統合テストからも見えません。

これは困ったことではなく、**統合テストの価値そのもの**です。
統合テストが通るなら、「利用者が使える範囲だけで、期待どおりに動く」ことが確認できています。
逆に「統合テストで private なものを呼びたくなる」なら、それは

- その関数は実は公開すべきもの（利用者も使いたいはず）か、
- 単体テストで確かめるべき内部の部品か

のどちらかです。前者なら `pub` を検討し、後者なら単体テストに書きます。

</details>

## どれを書けばよいか

迷ったときの目安です。

| 確かめたいこと | 書く場所 |
| --- | --- |
| private な関数の細かい動き（境界値、計算の正しさ） | 単体テスト |
| 公開APIを組み合わせた、利用者目線の使い方 | 統合テスト |
| 「この関数はこう使う」という**使い方の見本** | ドキュメントテスト |

ドキュメントテストは、**テストというより「動くことが保証された説明書」**です。
コード例を書いておくと、`cargo doc` で生成されるドキュメントに例として表示され、
しかも `cargo test` で実行されるので、**関数を変更して例が古くなったら気づけます**。
説明書のコード例が動かない、という（よくある）問題を防いでくれます。

## Deep Dive: この教材はすでに3種類を使い分けている

これまでの演習を振り返ると、3種類がそれぞれの役割で使われています。

| 演習での使い方 | 種類 | 理由 |
| --- | --- | --- |
| `tests/tests.rs` で公開APIを判定する | 統合テスト | 学習者の実装の中身ではなく、**外から見た動き**だけで判定したいから |
| `/// ```compile_fail` で「使えないこと」を確かめる | ドキュメントテスト | doctest は**別crateとして**コンパイルされるので、利用者の立場で「見えない・使えない」ことを確かめられる（09-3 の sealed trait、12-4 の facade） |
| 本文（mdBook）のコード例 | ドキュメントテストと同じ仕組み | `mdbook test` は rustdoc を使ってコード例を実行している |

「統合テストが公開APIしか見ない」ことは、演習の判定方法としても都合がよい性質でした。
学習者がどんな内部構造で実装しても、公開APIの振る舞いが正しければ正解にできるからです。

## Exercise

**[`ex051_kinds_of_tests`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex051_kinds_of_tests)** — `cargo test -p ex051_kinds_of_tests` で判定します。

この演習では、**3種類のテストがすでに用意されています**。関数を実装すると、3種類すべてが通るようになります。

| 種類 | 場所 | 何を確かめているか |
| --- | --- | --- |
| 単体テスト | `src/lib.rs` の `mod tests` | private な `normalize` |
| 統合テスト | `tests/tests.rs` | 公開API の `word_frequency` |
| ドキュメントテスト | `word_frequency` の `///` コメント | 使い方の例 |

実装後に `cargo test -p ex051_kinds_of_tests` の出力を見て、3つの `running ...` のまとまりが
それぞれどのテストか確認してください。

## Challenge

`ex051` の統合テストに、private な `normalize` を呼ぶテストを追加してみてください。
どんなエラーになりますか。そのテストは、どこに書くのが正しいでしょうか。

## Review

- [ ] 単体テスト・統合テスト・ドキュメントテストの置き場所を説明できる
- [ ] 統合テストが「利用者と同じ立場」で、`pub` なものしか見えない理由を説明できる
- [ ] ドキュメントテストが「動くことが保証された説明書」になる理由を説明できる
- [ ] 確かめたいことに応じて、テストの種類を選べる

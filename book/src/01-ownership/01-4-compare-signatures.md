# Lesson 01-4: シグネチャを比較する

## Concept

同じ処理でも、シグネチャの選び方で**所有権・確保・柔軟性・読みやすさ**が変わります。
この Lesson では「どれが正解か」ではなく、**どの観点でどれが有利か**を言葉にします。

## Why?

実務のRustでは、同じ問題に複数の妥当な書き方があります。
選んだ理由を説明できなければ、次にコードを読む人（未来の自分やAI）は、その設計を守るべきか変えてよいか判断できません。

## 題材

「文字列のリストから、空白だけの要素を捨て、前後の空白を取り除き、大文字にして返す」関数を考えます。

## 3つの実装

```rust
// (A) 所有権を受け取る
fn process_owned(data: Vec<String>) -> Vec<String> {
    data.into_iter()
        .map(|s| s.trim().to_uppercase())
        .filter(|s| !s.is_empty())
        .collect()
}

// (B) スライスを借りる
fn process_slice(data: &[String]) -> Vec<String> {
    data.iter()
        .map(|s| s.trim().to_uppercase())
        .filter(|s| !s.is_empty())
        .collect()
}

// (C) IntoIterator を受け、Iterator を返す
fn process_iter<I>(data: I) -> impl Iterator<Item = String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    data.into_iter()
        .map(|s| s.as_ref().trim().to_uppercase())
        .filter(|s| !s.is_empty())
}

fn main() {
    let list = vec![" a ".to_string(), "  ".to_string(), "b".to_string()];

    let r1 = process_slice(&list);          // list はまだ使える
    let r2 = process_owned(list.clone());   // list を残すなら clone が必要
    let r3: Vec<String> = process_iter(&list).collect();
    let r4: Vec<String> = process_iter(["x", " y "]).collect(); // &str の配列も渡せる

    assert_eq!(r1, vec!["A", "B"]);
    assert_eq!(r1, r2);
    assert_eq!(r1, r3);
    assert_eq!(r4, vec!["X", "Y"]);
}
```

## Think

> **問い**: 次の6つの観点で、(A)(B)(C) を比較してください。
> 特に **「(A) が有利になる状況はあるか？」** を考えてみてください。

| 観点 | 問い |
| --- | --- |
| ownership | 呼び出し側は、呼び出し後に元のデータを使えるか |
| allocation | 入力・出力でヒープ確保は何回起きるか |
| API flexibility | どんな型の入力を受けられるか |
| readability | シグネチャと実装は読みやすいか |
| performance | 不要なコピーや中間コレクションはあるか |
| abstraction | 抽象化のコストに見合っているか |

<details>
<summary>Hint</summary>

この処理では、**新しい `String` を必ず作っています**（`to_uppercase` が新しい文字列を返すため）。
入力の `String` を再利用できているでしょうか。

</details>

<details>
<summary>Solution</summary>

| 観点 | (A) `Vec<String>` | (B) `&[String]` | (C) `IntoIterator` → `Iterator` |
| --- | --- | --- | --- |
| ownership | 入力を消費。残したければ `clone` | 借用のみ。元は使える | 入力次第（`&Vec` なら借用、`Vec` ならmove） |
| allocation | 各要素の新規 `String`。外側の `Vec` は、標準ライブラリの最適化で入力のバッファが再利用されることがある（保証はない） | 出力の `Vec` ＋ 各要素の新規 `String` | 各要素の新規 `String` のみ。**`Vec` は呼び出し側が必要なときだけ作る** |
| API flexibility | `Vec<String>` のみ | スライスに見えるもの（`&Vec`, 配列） | `&str` の配列、`&[String]`、`Vec` など幅広い |
| readability | 最も単純 | 単純 | `where` と `impl Trait` の分だけ複雑。エラーも読みにくい |
| performance | 要素の `String` は再利用できない（`to_uppercase` が新しい `String` を作る）。得られうるのは外側のバッファ1つ分だけ | 妥当 | 中間 `Vec` を作らず、遅延評価できる |
| abstraction | なし | なし | 過剰になりうる（下記） |

**(A) が有利になる状況**は、関数の中で**入力のバッファを再利用できる**ときです。
たとえば `s.make_ascii_uppercase()` のように、その場で書き換えられる処理なら、
`Vec<String>` を受け取って変更し、そのまま返すことで確保を減らせます。
今回の `to_uppercase()` は新しい `String` を返すので、要素ごとの確保は (A) でも減りません。

> **補足（訂正）**: 実は `vec.into_iter().map(...).filter(...).collect::<Vec<_>>()` は、
> 要素の型のサイズが同じなら、標準ライブラリの最適化（in-place collect）によって
> **外側の `Vec` のバッファが再利用されることがあります**（Rust 1.75 で確認）。
> ただしこれは実装の詳細で、ドキュメント上は保証されていません。

つまり (A) で得られうるのは「外側のバッファ1つ分」だけで、しかも保証されない最適化頼みです。
その代わりに、元のリストを残したい呼び出し側には `clone`（全要素のコピー）を強います。
**この処理では、所有権を要求する理由として弱すぎる**、というのが結論です。

**選び方の目安**:

- 内部の小さな関数で、呼び出しが1〜2箇所 → (B)。単純で十分
- ライブラリの公開API、大きなデータ、呼び出し側が `Vec` 以外を持つ → (C)
- 入力を再利用・変更でき、所有権を渡すことに意味がある → (A)

**(C) の代償**: 戻り値が `impl Iterator` だと、呼び出し側は「戻り値を2回使う」「`len()` を呼ぶ」ができません。
`Vec` が要るなら `collect` が必要です。一方、公開APIで `impl Iterator` を返すと、
具体的な型を隠しているので、**内部で使うイテレータの実装を後から変えても利用者を壊さない**という自由が得られます
（その代わり、`ExactSizeIterator` のような追加の能力を利用者が当てにすることはできません）。

</details>

## Deep Dive: `I::Item: AsRef<str>` は何をしているか

(C) の `where` 句が要求しているのは、「要素が `&str` として読める」ことだけです。
`String`、`&str`、`&String` はすべて `AsRef<str>` を実装しているので、同じ関数で受けられます。

```rust
fn shout<T: AsRef<str>>(s: T) -> String {
    s.as_ref().to_uppercase()
}

fn main() {
    assert_eq!(shout("a"), "A");
    assert_eq!(shout(String::from("b")), "B");
    assert_eq!(shout(&String::from("c")), "C");
}
```

`as_ref` の存在は、**「この関数は所有権を必要としない。文字列として読めれば何でもよい」**という宣言です。
これはChapter 04・05でtrait boundsの設計として掘り下げます。

## Exercise

**`ex004_compare_signatures`** — `cargo test -p ex004_compare_signatures` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `fn dedup_sorted_owned(v: Vec<String>) -> Vec<String>` | ソートして重複を除く。**入力のバッファを再利用**する実装にする |
| `fn dedup_sorted_ref(v: &[String]) -> Vec<String>` | 同じ結果を、入力を変更せずに返す |
| `fn dedup_sorted_iter<I>(v: I) -> Vec<String>` | `I: IntoIterator`、`I::Item: AsRef<str>` で受ける |

テストは3つの関数が同じ結果を返すことを確認します。
この課題では `sort` と `dedup` がその場で並べ替えられるので、(A) 版は本文の例と違って
**要素の `String` もバッファも実際に再利用できます**。
任意で `NOTES.md` に、**(A) 版が有利になるのはどんなときか**を3行以内で書いてください。

## Challenge

(C) の戻り値を `Vec<String>` に変えたバージョンを作り、`impl Iterator` を返す版と比べてください。
どんな呼び出しコードなら `Vec` 版の方が使いやすいでしょうか。

## Review

- [ ] `Vec<T>` / `&[T]` / `impl IntoIterator` を、6つの観点で比較できる
- [ ] 所有権を受け取る理由が「実装の中にあるか」で判断できる
- [ ] `AsRef<str>` が何を宣言しているか説明できる

# Lesson 00-2: `cargo test` で学ぶ

## Concept

この教材の演習は、**テストが通れば完成**です。
採点する人はいません。`cargo test` の結果が、そのまま判定になります。

そのため、演習で一番大切な力は「テストの失敗を読む力」です。
失敗のメッセージには、何が期待されていて、実際には何が起きたかが書かれています。

## Why?

テストは、**問題文の一部**です。

演習の `src/lib.rs` には、関数のシグネチャとドキュメントコメントしか書かれていません。
「同じ長さのときはどちらを返すのか」「空の入力ならどうなるのか」といった細かい決まりは、
`tests/tests.rs` のテストに書かれています。
テストを読まずに実装を始めると、問題文の半分を読まずに解き始めるのと同じことになります。

## 演習はどこにあるか

この Web ページ（本文）には、演習のコードは入っていません。
演習は、GitHub のリポジトリ **[Kz1wg/rust-mastery](https://github.com/Kz1wg/rust-mastery)** にあります。
自分のパソコンにコピー（clone）して、そこで解きます。

### 準備

Rust（`cargo`）が使えることを確かめてから、リポジトリを clone します。

```bash
cargo --version    # 1.75 以上なら大丈夫。無ければ https://rustup.rs からインストールする

git clone https://github.com/Kz1wg/rust-mastery.git
cd rust-mastery
cargo run -p check-exercises    # 全演習の一覧が出れば準備完了（最初は全部 ⬜）
```

以降のコマンドは、すべてこの `rust-mastery` ディレクトリ（リポジトリのいちばん上）で実行します。
エディタで開くときも、このディレクトリごと開いてください。rust-analyzer（エディタの Rust 補完）が、全ての演習を認識します。

### リポジトリの中身

```text
rust-mastery/
├── book/          この本文（Web ページの元の Markdown）
├── exercises/     演習。ex001_move_semantics/ から ex061_documentation/ まで、1つの演習が1つのディレクトリ
├── solutions/     演習の模範解答（テストが通るまで開かない）
├── projects/      Chapter 17 の実践プロジェクトと、Chapter 18 の参考実装
├── docs/          演習の一覧（exercise-index.md）など
└── tools/         check-exercises（進み具合の表示）など
```

### Lesson と演習の対応

各 Lesson の終わり近くにある **Exercise** の節に、演習の名前が書いてあります。

> **`ex001_move_semantics`** — `cargo test -p ex001_move_semantics` で判定します。

この名前が、そのまま `exercises/` の下のディレクトリ名です。
全演習と Lesson の対応表は、リポジトリの [docs/exercise-index.md](https://github.com/Kz1wg/rust-mastery/blob/main/docs/exercise-index.md) にあります。

## 演習の形

1つの演習は、1つの小さな crate です。

```text
exercises/ex001_move_semantics/
├── exercise.toml      演習の情報（どの Lesson か、難しさ）
├── Cargo.toml
├── src/lib.rs         ここを書く。関数の本体は todo!() になっている
└── tests/tests.rs     判定に使うテスト。書き換えない
```

`src/lib.rs` の中身は、たとえばこうなっています。

```rust,ignore
/// 長い方の `String` を返す。同じ長さなら `a` を返す。
/// 引数の所有権を受け取る（呼び出し側は以降 `a` / `b` を使えない）。
pub fn longest_owned(a: String, b: String) -> String {
    todo!("a と b の長さを比較し、長い方を返してください（同じ長さなら a）")
}
```

`todo!()` は「まだ書いていない」ことを表すマクロです。
どんな型の値の代わりにも置けるのでコンパイルは通りますが、実行されると panic します。

## 進め方

```bash
# 1. どの Lesson の演習かを確かめ、本文を読む
cat exercises/ex001_move_semantics/exercise.toml

# 2. テストを読む（問題文の続き）
cat exercises/ex001_move_semantics/tests/tests.rs

# 3. src/lib.rs を書き、判定する
cargo test -p ex001_move_semantics

# 4. 全体の進み具合を見る
cargo run -p check-exercises
```

`-p` の後ろは演習の名前（crate 名）です。
入れ子になった演習（`ex012`・`ex048`・`ex055`）だけは、その演習のディレクトリの中で `cargo test` を実行します（例: `cd exercises/ex012_non_exhaustive && cargo test`）。

`check-exercises` は、全演習の状態を章ごとに表示します。

```text
01 Ownership & Borrowing
  ✅ ex001_move_semantics         (01-1)
  ❌ ex002_borrow_errors          (01-2)  1 failed
  ⬜ ex003_ownership_design       (01-3)  5 todo
  ⬜ ex004_compare_signatures     (01-4)  6 todo

Progress: 1 / 4
```

| 表示 | 意味 |
| --- | --- |
| ✅ | 全てのテストが通った |
| ⬜ N todo | 失敗している N 個のテストは、すべて `todo!()`（まだ書いていない所）に届いたもの。骨組みのままの演習はこの表示になる |
| ❌ N failed | `todo!()` 以外の理由で失敗しているテストがある（答えが違う、panic した、など）。N は失敗しているテストの数 |
| ⚠️ build error | コンパイルできない |

演習によっては、骨組みのままでも通るテストがあります（型の性質だけを確かめるテストなど）。
そのため、通ったテストの数ではなく**失敗の理由**で、⬜ か ❌ かを決めています。

## 失敗の読み方

`cargo test` の失敗は、おおむね次の3種類です。

| 種類 | 見分け方 | 意味 | 次にすること |
| --- | --- | --- | --- |
| まだ書いていない | `not yet implemented: …` | `todo!()` が実行された | `todo!()` のメッセージを読み、実装する |
| 答えが違う | `` assertion `left == right` failed `` | 実装は動いたが、期待と違う値を返した | 下の「left と right を読む」へ |
| コンパイルできない | `error[E0382]: …` などで、テストが1つも走らない | 型や所有権の規則に合っていない | **最初の**エラーから読む（Appendix A） |

### left と right を読む

実装を1か所だけ間違えたとき、次のような失敗が出ます。

```text
---- longest_owned_prefers_a_on_tie stdout ----

thread 'longest_owned_prefers_a_on_tie' panicked at exercises/ex001_move_semantics/tests/tests.rs:17:5:
assertion `left == right` failed
  left: "bb"
 right: "aa"
```

読む順番はこうです。

1. **テストの名前**: `longest_owned_prefers_a_on_tie`（同じ長さなら a を選ぶ）。何を確かめるテストかが分かります
2. **場所**: `tests/tests.rs:17`。そのテストのコードを開きます
3. **left と right**: `assert_eq!(left, right)` の、左が実際の値、右が期待された値です（この教材のテストは、この順番で書いています）

```rust,ignore
#[test]
fn longest_owned_prefers_a_on_tie() {
    assert_eq!(longest_owned("aa".to_string(), "bb".to_string()), "aa");
    //         ^^^^^^^^^^^^^^^^ left（実際の値）                      ^^^^ right（期待した値）
}
```

## Think

> **問い**: 上の失敗では、実装はどんな間違いをしているでしょうか。
> 実装を見ずに、失敗のメッセージだけから推測してください。

<details>
<summary>Hint</summary>

`"aa"` と `"bb"` は同じ長さです。実装は、同じ長さのときにどちらを返しましたか？

</details>

<details>
<summary>Solution</summary>

同じ長さのときに `b` を返しています。
たとえば `if a.len() > b.len() { a } else { b }` と書くと、長さが等しいときは `else` に進むので `b` になります。
`>=` にすれば、同じ長さのときに `a` を返します。

このように、**テストの名前と left / right だけで、原因の見当がつく**ことが多いです。
見当がつかないときに初めて、`println!` やデバッガで中を調べます。

</details>

## 1つのテストだけを走らせる

テストの名前の一部を渡すと、名前にその文字列を含むテストだけが走ります。

```bash
cargo test -p ex001_move_semantics tie        # 名前に "tie" を含むテストだけ
cargo test -p ex001_move_semantics -- --nocapture   # テストの中の println! を表示する
```

## 約束ごと

- **`tests/tests.rs` は書き換えない**。テストを変えれば、どんな実装でも「通る」ことになってしまいます
- **関数のシグネチャは変えない**。テストはそのシグネチャで呼んでいます。シグネチャに疑問を持ったら、それは良い問いです。本文の Think で扱っていることが多いので、先に本文を読んでください
- **解答（`solutions/`）は最後に見る**。テストが通った後に見比べると、「同じ結果を出す別の書き方」を学べます

## Deep Dive: テストが通れば、良いコードか

テストが確かめているのは「決められた入力に、決められた出力を返すか」だけです。
読みやすさ、無駄なコピーをしていないか、エラーの扱いが適切かは、テストでは分かりません。

この教材では、テストが通った後に次の2つを実行することをすすめます。

```bash
cargo clippy -p ex001_move_semantics   # 「動くけれど、もっと良い書き方がある」を指摘してくれる
cargo fmt -p ex001_move_semantics      # 書式を標準の形にそろえる
```

`clippy` の指摘は、それ自体が学びになります（P1 のページに例があります）。
そして最後に、本文の Review の項目を自分の言葉で説明できるか確かめてください。
**テストが通ることはスタート地点で、ゴールは「なぜこの設計なのか」を説明できること**です。

## Review

- [ ] リポジトリを clone し、`check-exercises` で演習の一覧を表示できた
- [ ] Lesson の Exercise 節から、対応する演習のディレクトリを見つけられる
- [ ] 演習の3つのファイル（`exercise.toml`・`src/lib.rs`・`tests/tests.rs`）の役割を説明できる
- [ ] 失敗の3種類（未実装・値の違い・コンパイルエラー）を見分けられる
- [ ] `assert_eq!` の失敗で、left と right のどちらが期待値かを知っている
- [ ] テストの名前で絞り込んで実行できる

## 次へ

演習の準備ができたら、**[00-3 診断問題](00-3-diagnostic.md)** で、この教材の前提になる知識を確かめましょう。

# Lesson 00-1: この教材の使い方

## Concept

この教材は「読んで理解する」ものではなく、「考えてから確かめる」ものです。
各Lessonは、答えを最初に見せません。

## Lessonの流れ

```text
Concept → Why? → Bad Example → Problem → Think → Hint
       → Solution → Deep Dive → Exercise → Challenge → Review
```

**Think** の問いには、Hintを開く前に、自分の言葉で答えてみてください。
Hintは1つずつ、Solutionは最後に開きます。

## 例：コンパイルエラーを教材として読む

この教材では、コンパイルエラーを「直して終わり」にしません。
まず次のコードを読んでください。**コンパイルは通るでしょうか？**

```rust,compile_fail,E0505
fn main() {
    let x = String::from("hello");
    let y = &x;

    drop(x);

    println!("{y}");
}
```

### Think

1. 何が borrow されていますか？
2. なぜ `drop(x)` できないのですか？
3. コンパイラは何を保証しようとしていますか？

<details>
<summary>Hint</summary>

`y` が最後に使われる場所と、`drop(x)` の場所の順序に注目してください。

</details>

<details>
<summary>Solution</summary>

1. `let y = &x;` により、`x` は `y` から借用されています。
2. `drop(x)` は `x` の所有権をmoveして破棄します。その後も `println!("{y}")` で `y` が使われるため、`y` は解放済みのメモリを指すことになります。
3. 「参照が生きている間は、参照先が生きている」ことです。これが dangling reference を防ぐ仕組みです。

修正は、`println!` を `drop(x)` の前に移す（借用の期間を先に終わらせる）か、`y` が不要になるように設計を見直すことです。
どちらが望ましいかは、**なぜ `x` を早く破棄したいのか**による——この問いの掘り下げは、Chapter 01 で行います。

</details>

> このページのコードは、`mdbook test` によって「**実際にコンパイルに失敗すること**」が検証されています。
> 「エラーになるはず」という教材の説明と実際の挙動がずれると、テストが失敗します。

## Review

- [ ] Lessonの流れを説明できる
- [ ] Thinkを先に考える、という進め方を理解した

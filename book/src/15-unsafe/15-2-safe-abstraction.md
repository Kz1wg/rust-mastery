# Lesson 15-2: safe abstraction を作る

## Concept

標準ライブラリの中には、たくさんの `unsafe` が使われています。
それでも私たちは、`Vec` や `split_at_mut` を**安全なAPIとして**使えます。

それができるのは、unsafe な部分を**関数の内側に閉じ込め**、
「どんな引数で呼ばれても、メモリが壊れない」ことを関数自身が保証しているからです。
これを **safe abstraction（安全な抽象化）** と言います。

## Why?

unsafe を使うこと自体は避けられない場面があります。大事なのは、
**unsafe を使う場所をできるだけ狭くし、その外側には安全なAPIだけを見せる**ことです。
こうすると、確認が必要な範囲が「その関数の中だけ」に限られます。

## unsafe が必要になる例: `split_at_mut`

スライスを2つに分けて、両方を同時に書き換えたいとします（01-2 の演習で使った関数です）。
安全なコードだけで素直に書くと、こうなります。

```rust,compile_fail,E0499
fn my_split_at_mut(s: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    (&mut s[..mid], &mut s[mid..])
}

fn main() {}
```

## Problem

私たちには、`[..mid]` と `[mid..]` が**重ならない**ことが分かっています。
重ならないなら、両方を同時に可変で借りても問題はありません。

しかしコンパイラには「同じ `s` を2回可変で借りている」としか見えず、エラーになります。
**正しいのに、コンパイラには確認できない**——まさに unsafe の出番です。

## Think

> **問い**: unsafe を使ってこの関数を書くとき、「どんな引数で呼ばれても安全」と言うためには、
> 関数の中で何を確認する必要がありますか？

<details>
<summary>Hint</summary>

`mid` がスライスの長さより大きかったら、どうなるでしょうか。

</details>

<details>
<summary>Solution</summary>

```rust
fn my_split_at_mut(s: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = s.len();

    // (1) まず、安全性の前提を「関数の中で」確かめる
    assert!(mid <= len, "mid ({mid}) がスライスの長さ ({len}) を超えています");

    let ptr = s.as_mut_ptr();

    // SAFETY:
    // - mid <= len を上で確認したので、2つの範囲はどちらもスライスの中に収まる
    // - [0, mid) と [mid, len) は重ならないので、同時に可変で持っても競合しない
    // - 戻り値の lifetime は s の借用と同じなので、元のスライスより長く生きない
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = [1, 2, 3, 4];
    let (a, b) = my_split_at_mut(&mut v, 1);
    a[0] = 10;
    b[0] = 20;
    assert_eq!(v, [10, 20, 3, 4]);
}
```

一番大事なのは、**`assert!` で前提を確かめている**ことです。

もし `assert!` が無く、`mid = 100` で呼ばれたら、`from_raw_parts_mut` はスライスの外のメモリを指す
スライスを作ってしまいます。それはもう**メモリ破壊**です。

`assert!` があれば、不正な `mid` は panic で止まります。panic は「プログラムが止まる」だけで、メモリは壊れません。
だからこの関数は `unsafe fn` にする必要がなく、**普通の `fn` として公開できる**のです。

</details>

## safe abstraction の条件

普通の `fn`（`unsafe` の付かない関数）として公開してよいのは、次を満たすときだけです。

> **どんな引数で呼ばれても、どんな順番で呼ばれても、メモリが壊れない。**

「正しく使えば安全」では足りません。**間違った使い方をされても安全**でなければなりません。
間違った使い方で壊れるなら、その関数は `unsafe fn` にして、`# Safety` で条件を書く必要があります（15-1）。

| 関数の種類 | 安全性を守る責任 | 前提の確認 |
| --- | --- | --- |
| 普通の `fn`（safe abstraction） | **関数の作者** | 関数の中で `assert!` などで確かめる |
| `unsafe fn` | **呼ぶ側** | `# Safety` に書き、呼ぶ側が確かめる |

## Deep Dive: 確かめる道具

unsafe を含むコードのテストでは、「テストが通った」だけでは安心できません。
未定義動作（undefined behavior）は、**たまたま正しく動いてしまう**ことがあるからです。

Rust には **Miri** というツールがあり、テストを「メモリの使い方を1つずつ検査しながら」実行してくれます。

```text
rustup +nightly component add miri
cargo +nightly miri test
```

範囲外アクセスや、解放済みメモリの参照などを検出できます。
unsafe を書いたら、Miri でテストを走らせるのが実務の定番です（nightly ツールチェーンが必要です）。

## Exercise

**`ex057_safe_abstraction`** — `cargo test -p ex057_safe_abstraction` で判定します。

| 関数 | 仕様 |
| --- | --- |
| `my_split_at_mut(s, mid)` | ジェネリックな `&mut [T]` を2つに分ける。範囲外なら `"out of bounds"` を含むメッセージで panic |
| `first_and_last_mut(s)` | 先頭と末尾への可変参照を同時に返す。要素が2つ未満なら `None` |

テストでは、範囲外の `mid` が **メモリを壊さず panic で止まる**ことも確かめています。

## Challenge

`first_and_last_mut` は、実は unsafe を使わずに書けます（`split_first_mut` と `split_last_mut` を組み合わせる、など）。
unsafe 版と safe 版の両方を書いて比べてください。**unsafe を使わずに書けるなら、そちらを選ぶ**のが原則です。

## Review

- [ ] safe abstraction とは何か、`Vec` や `split_at_mut` を例に説明できる
- [ ] 普通の `fn` として公開するための条件（どんな引数でもメモリが壊れない）を説明できる
- [ ] 前提を `assert!` で確かめることで、`unsafe fn` にしなくて済む理由を説明できる
- [ ] unsafe のテストに Miri が役立つことを知っている

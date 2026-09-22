# Lesson 07-3: 遅延評価とallocation

## Concept

`nums.iter().map(...).filter(...)` は、この時点では**何も実行されません**。
`collect()` や `for`、`sum()` のように**消費する操作**が呼ばれて初めて、
1つずつ値が流れ始めます。

## Why?

「途中に `Vec` を作らないと動かない」と誤解していると、不要な `collect()` を書いてしまい、
使うはずのなかったメモリを確保することになります。

## Bad Example: 不要な中間 `Vec`

```rust
fn doubled(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|x| x * 2).collect()
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let doubled_nums = doubled(&nums);
    let total: i32 = doubled_nums.iter().sum();
    println!("{total}");
}
```

## Problem

`doubled` が返す `Vec<i32>` は、`main` の中で合計を求めるためだけに使われ、
**その後は使われません**。`doubled` の中で `collect()` した時点で、
5要素分のヒープ確保が発生していますが、最終的に必要なのは1つの `i32` だけです。

## Think

> **問い**: `doubled` を、`Vec` を作らずに済むように書き直してください。
> 呼び出し側は、それをどう使いますか？

<details>
<summary>Hint</summary>

`doubled` が `Vec<i32>` ではなく `impl Iterator<Item = i32>` を返せば、
`collect()` するかどうかを呼び出し側が決められます。

</details>

<details>
<summary>Solution</summary>

```rust
fn doubled(nums: &[i32]) -> impl Iterator<Item = i32> + '_ {
    nums.iter().map(|x| x * 2)
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let total: i32 = doubled(&nums).sum(); // Vecを経由しない
    println!("{total}");
}
```

戻り値の型にある `+ '_` は、「このイテレータは引数 `nums` を借用している」ことを表します。
edition 2021 では、`impl Trait` の戻り値は**境界に書かれていないlifetimeを借用できません**。
`Item = i32` にはlifetimeが現れないので、`+ '_` を外すとエラーになります:

```rust,compile_fail,E0700
fn doubled(nums: &[i32]) -> impl Iterator<Item = i32> {
    nums.iter().map(|x| x * 2)
}

fn main() {}
```

（edition 2024 では規則が変わり、スコープ内のlifetimeは既定で借用可能になるため `+ '_` は不要です。
この教材は現在 edition 2021 で書かれています。）

`doubled` は「2倍にする」という**変換の手順**を返すだけで、実際の計算は
`main` で `.sum()` が呼ばれたときに、1要素ずつ流れながら行われます。
中間の `Vec` は一度も作られません。

</details>

## `collect()` が本当に必要な場面

遅延評価のままでよい場面と、`collect()` が必要な場面を区別してください。

| 状況 | 判断 |
| --- | --- |
| 1回だけ順番に消費する（合計、表示、forループ） | `collect()` 不要。iteratorのまま渡す |
| 複数回イテレートする必要がある | `collect()` が必要（iteratorは一度使うと空になる） |
| インデックスアクセス（`v[3]`）が必要 | `collect()` してVecなどにする必要がある |
| 関数の外部（呼び出し元やAPIの境界）が具体的な `Vec` / `HashMap` を要求する | `collect()` が必要 |
| 要素数を先に知る必要がある（`len()`） | 多くの場合 `collect()` が必要（`ExactSizeIterator` を除く） |

## Deep Dive: 副作用のあるadapterも実行されない

```rust
fn main() {
    let nums = vec![1, 2, 3];
    let iter = nums.iter().map(|x| {
        println!("processing {x}");
        x * 2
    });
    println!("map()を呼んだだけでは、まだ何も表示されない");
    let _total: i32 = iter.sum(); // ここで初めて println! が実行される
}
```

`map` に渡したクロージャの中に `println!` があっても、`map()` を呼んだ時点では
何も表示されません。`sum()` が呼ばれ、実際に値が1つずつ取り出されるときに、
クロージャが実行されます。**iteratorのメソッドチェーンは、「何をするかの計画」を
組み立てているだけ**で、消費されるまで実行されません。

## Exercise

**`ex028_laziness_and_allocation`** — `cargo test -p ex028_laziness_and_allocation` で判定します。

中間 `Vec` を作らずに済む形へ、関数のシグネチャと実装を書き直します。

## Challenge

自分のコードで、`Vec` を返す関数の戻り値が、呼び出し側で一度しか（forループなどで1回だけ）
使われていない箇所を探してください。`impl Iterator` に変えられないか検討してください。

## Review

- [ ] iteratorのメソッドチェーンが、消費されるまで実行されないことを説明できる
- [ ] `collect()` が本当に必要な場面（複数回イテレート、インデックスアクセスなど）を判断できる
- [ ] 不要な中間 `Vec` を避け、`impl Iterator` を返す設計ができる

# Lesson 07-1: adapterの組み合わせ

## Concept

`filter`・`map`・`fold` のようなadapterの連鎖と、`for` ループは、
多くの場合どちらでも同じ結果を書けます。**どちらが読みやすいか**で選んでください。

## Why?

「関数型っぽく書く」ことや「forループを避けること」自体は目的ではありません。
読みにくいadapterの連鎖は、読みにくいforループと同じくらい良くないコードです。

## Bad Example: forループで書ける単純な変換

```rust
fn even_squares(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &n in nums {
        if n % 2 == 0 {
            result.push(n * n);
        }
    }
    result
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", even_squares(&nums));
}
```

## Think

> **問い**: このforループを、`filter` と `map` を使って書き直してください。
> 書き直した後、どちらが読みやすいと感じますか？

<details>
<summary>Solution</summary>

```rust
fn even_squares(nums: &[i32]) -> Vec<i32> {
    nums.iter().filter(|&&n| n % 2 == 0).map(|&n| n * n).collect()
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", even_squares(&nums));
}
```

「偶数を選んで、二乗する」という**2つの独立した変換**が、`filter` と `map` という
**名前のついた操作**として並んでいます。forループ版は、同じことを`if`と`push`という
低レベルな操作の組み合わせで表現しており、「何をしているか」を読み手が再構築する必要があります。

**この程度の単純な変換（フィルタ、写像、集約）は、adapterの方が意図を素直に表せます。**

</details>

## Bad Example: 無理にadapterへ押し込む

逆に、**状態を持ちながら早期終了する**ようなロジックは、adapterに無理やり詰め込むと
かえって読みにくくなることがあります。

```rust
fn first_index_over(nums: &[i32], threshold: i32) -> Option<usize> {
    nums.iter()
        .scan(0, |sum, &n| {
            *sum += n;
            Some(*sum)
        })
        .position(|sum| sum > threshold)
}

fn main() {
    let nums = vec![1, 2, 3, 10, 1];
    println!("{:?}", first_index_over(&nums, 5));
}
```

## Think

> **問い**: `scan` は「途中経過を持ち歩きながら変換する」adapterです。
> このコードを初めて読む人にとって、何をしているかすぐに分かりますか？
> `for` ループで書き直すと、どう変わりますか？

<details>
<summary>Solution</summary>

```rust
fn first_index_over(nums: &[i32], threshold: i32) -> Option<usize> {
    let mut sum = 0;
    for (i, &n) in nums.iter().enumerate() {
        sum += n;
        if sum > threshold {
            return Some(i);
        }
    }
    None
}

fn main() {
    let nums = vec![1, 2, 3, 10, 1];
    println!("{:?}", first_index_over(&nums, 5));
}
```

`for` ループ版は、`sum` という状態が「変数」として素直に見え、
「累積し、超えたら`i`を返す」という制御フローがそのまま読めます。
`scan` + `position` 版は、`scan` のクロージャが何を`Some`で包んで返しているのか
（次のadapterに渡す値）を読み手が推測する必要があり、**慣れていないと直感的ではありません**。

**判断の目安**:

| 状況 | 向いている書き方 |
| --- | --- |
| 単純な変換・フィルタ・集約（写像・選別・合計など） | adapterの連鎖 |
| 複数の状態を追跡しながら早期終了する | `for` ループ |
| ループの中で複雑な条件分岐や副作用（I/Oなど）がある | `for` ループ |
| 「forループでは何をしているか一目で分からない」定型的な変換 | adapterの連鎖 |

</details>

## Deep Dive: adapterは「意図」を、forループは「手順」を表す

`nums.iter().filter(...).map(...)` は、**「何をするか」**（フィルタして、写す）を宣言しています。
`for` ループは、**「どうやるか」**（1つずつ取り出し、条件を見て、変換し、詰める）を記述しています。

意図がそのまま名前のついた操作の並びで表現できるなら、adapterが勝ります。
手順が複雑になり、名前のついた操作の組み合わせでは逆に読みにくくなるなら、
forループで明示的に書く方が親切です。

## Exercise

**`ex026_adapters_vs_for`** — `cargo test -p ex026_adapters_vs_for` で判定します。

単純な変換をadapterの連鎖で、複数の状態を追跡する処理をforループで実装します。

## Challenge

自分のコードから、深くネストした（3段階以上の）adapterの連鎖を探してください。
forループに書き直したら読みやすくなるか、逆に今のままの方が良いか、判断してみてください。

## Review

- [ ] 単純な変換にはadapterの連鎖が向いていることを説明できる
- [ ] 複数の状態を追跡する処理にはforループが向いていることを説明できる
- [ ] adapterの連鎖とforループを、可読性の観点で選べる

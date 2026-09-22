# Lesson 09-4: zero-cost abstraction

## Concept

Rust はよく「zero-cost abstraction（ゼロコストの抽象化）」を掲げます。
これは「抽象化にはコストがかからない」という意味では**ありません**。正確には、

> **使わない機能にはコストを払わない。使う機能は、手で書くより効率よくは書けない。**

という意味です。このLessonでは、何が zero で、何が zero ではないかを、測れる形で確かめます。

## Why?

「Rust だから速いはず」と思い込むと、`Box<dyn Trait>` や `Rc` のように**コストを払う選択**を、
意識しないまま積み重ねてしまいます。逆に、newtype や iterator を「遅そう」と避ける必要もありません。

## zero のもの（実行時のコストが無い）

```rust
use std::marker::PhantomData;
use std::mem::size_of;

#[repr(transparent)]
struct Meters(f64);

fn main() {
    // newtype は、中身と同じ大きさ（repr(transparent) でレイアウトが保証される）
    assert_eq!(size_of::<Meters>(), size_of::<f64>());

    // PhantomData は大きさ 0
    assert_eq!(size_of::<PhantomData<String>>(), 0);

    // Option<&T> は、null を「None」として使うので、参照と同じ大きさ（言語として保証されている）
    assert_eq!(size_of::<Option<&u8>>(), size_of::<&u8>());
    assert_eq!(size_of::<Option<Box<u8>>>(), size_of::<Box<u8>>());

    println!("すべて追加のメモリなし");
}
```

| 抽象化 | なぜ zero か |
| --- | --- |
| newtype（02-2） | 実行時には中身そのもの。型の区別はコンパイル時だけに存在する |
| `PhantomData`（09-1） | 大きさ 0。型の情報だけを運ぶ |
| generic・静的ディスパッチ（04-2） | 型ごとにコードが生成され、直接呼び出しになる |
| iterator の連鎖（07-1） | 最適化されると、手書きのループと同等のコードになることが多い |
| `Option<&T>` / `Option<Box<T>>` | 使われない値（null）を `None` の表現に使う（niche 最適化） |

iterator については、「最適化されると」という条件が付きます。debug ビルドでは遅いことがあり、
release ビルドでも常に同じ機械語になる保証はありません。気になるなら**計測する**のが正解です（Challenge 参照）。

## zero ではないもの（払っているコストを知っておく）

```rust
use std::mem::size_of;

trait Shape {
    fn area(&self) -> f64;
}

fn main() {
    // &dyn Trait は「データへのポインタ + vtable へのポインタ」の2つ分
    assert_eq!(size_of::<&dyn Shape>(), 2 * size_of::<usize>());

    // Option<u64> には null にあたる「使われない値」が無いので、判別用の領域が増える
    assert!(size_of::<Option<u64>>() > size_of::<u64>());
}
```

| 選択 | 払っているコスト |
| --- | --- |
| `dyn Trait`（04-2） | ポインタが2倍の大きさ、vtable 経由の間接呼び出し、インライン化されにくい |
| `Box<T>` | ヒープ確保と解放 |
| `Rc<T>` / `Arc<T>`（Chapter 10） | 参照カウントの増減（`Arc` はアトミック操作） |
| `RefCell<T>`（Chapter 10） | 実行時の借用チェック |
| generic（単相化） | 実行時は zero だが、型の数だけコードが増える（バイナリサイズ・コンパイル時間） |

これらは「悪い」選択ではありません。**必要な柔軟性の対価**です。
問題なのは、コストを払っていることに気づかず、必要ないのに払っている場合です。

## Think

> **問い**: 次の2つの関数は、release ビルドでほぼ同じ速さになると期待できますか？
> 期待できるとしたら、何を根拠に、どう確かめますか？

```rust
fn sum_even_squares_iter(v: &[u64]) -> u64 {
    v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum()
}

fn sum_even_squares_loop(v: &[u64]) -> u64 {
    let mut total = 0;
    for &x in v {
        if x % 2 == 0 {
            total += x * x;
        }
    }
    total
}

fn main() {
    let v: Vec<u64> = (1..=10).collect();
    assert_eq!(sum_even_squares_iter(&v), sum_even_squares_loop(&v));
}
```

<details>
<summary>Solution</summary>

**期待はできるが、保証はされない**、が正確な答えです。
iterator の各 adapter は小さな関数で、コンパイラがインライン化すると、手書きのループと同等の形になることが多いです。
しかしそれは最適化の結果であって、言語仕様の保証ではありません。

確かめ方は2つあります。

1. **計測する**: `cargo bench` やベンチマークツールで、実際の入力に対して時間を測る
2. **生成されたコードを見る**: Compiler Explorer（godbolt.org）や `cargo asm` で、release ビルドの機械語を比べる

「zero-cost のはず」を前提にするのではなく、**速さが重要な場所では測る**。これが Rust における zero-cost の正しい使い方です。

</details>

## Deep Dive: 抽象化を選ぶ順番

コストの観点からは、次の順で検討すると無駄が少なくなります。

1. 具体的な型（抽象化なし）
2. generic・`impl Trait`（実行時コスト zero、コードサイズは増える）
3. `dyn Trait`（異なる型を混ぜたいとき。間接呼び出しのコスト）
4. `Rc` / `RefCell` など（所有権や借用の規則を実行時に回すとき）

Chapter 04 の「必要になってから抽象化する」と同じ考え方を、コストの面から見たものです。

## Exercise

**`ex037_zero_cost`** — `cargo test -p ex037_zero_cost` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `sum_even_squares_iter` / `sum_even_squares_loop` | 同じ結果を、iterator の連鎖と for ループの両方で実装 |
| `UserId` | `#[repr(transparent)]` の newtype（用意済み）。`new` / `get` を実装 |

テストでは、結果が一致することに加えて、`UserId` が `u64` と同じ大きさであること、
`Option<&UserId>` が参照と同じ大きさであることを確かめています。

## Challenge

`sum_even_squares_iter` と `sum_even_squares_loop` を Compiler Explorer（`-C opt-level=3`）に貼り付け、
生成される機械語を比べてください。同じになりましたか？ `-C opt-level=0` ではどうでしょうか。

## Review

- [ ] zero-cost abstraction の正確な意味（使わないものにはコストを払わない）を説明できる
- [ ] newtype・`PhantomData`・`Option<&T>` が追加のメモリを使わないことを説明できる
- [ ] `dyn Trait`・`Box`・`Rc`・`RefCell` が払っているコストを説明できる
- [ ] 「zero-cost のはず」ではなく、必要なら計測・機械語で確かめる

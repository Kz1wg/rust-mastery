# Lesson 05-1: genericにするメリットはあるか

## Concept

genericは、**同じロジックが複数の型で重複しているとき**に、その重複を消すための道具です。
「型パラメータを持たせておけば柔軟になりそう」というだけでは、導入する理由になりません。

## Why?

重複が実際に無いのにgenericにすると、読む人は「他にどんな型が来うるのか」を考えなければならず、
具象型を使うより理解のコストが上がります。

## Bad Example: 重複を放置している

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("{} {}", largest_i32(&numbers), largest_char(&chars));
}
```

## Problem

2つの関数は、型が違うだけで**ロジックは完全に同じ**です。
新しい型（`f64` や `String`）向けにも同じ関数が必要になったら、また複製することになります。
ロジックにバグがあれば、全てのコピーを直す必要があります。

## Think

> **問い**: この重複を、genericで1つの関数にまとめてください。
> `T` にはどんなtrait boundが必要ですか？

<details>
<summary>Hint</summary>

`>` で比較するには `PartialOrd` が必要です。`list[0]` を`largest`の初期値として**コピー**するなら、`Copy` も必要です。

</details>

<details>
<summary>Solution</summary>

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("{} {}", largest(&numbers), largest(&chars));
}
```

関数は1つになり、`i32` でも `char` でも、`PartialOrd + Copy` を満たす型なら何でも使えます。
これは**genericが正当に価値を生んでいる例**です——実際に重複していたロジックが1つになりました。

</details>

## Bad Example: 重複が無いのにgenericにする

```rust
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }

    fn get(&self) -> &T {
        &self.value
    }
}

fn main() {
    // このコードベースには Wrapper<String> しか登場しない
    let w: Wrapper<String> = Wrapper::new("hello".to_string());
    println!("{}", w.get());
}
```

## Think

> **問い**:
> 1. このコードベースで `Wrapper<T>` は何種類の `T` で使われていますか？
> 2. `<T>` を取り除いて `value: String` に固定したら、何を失いますか？
> 3. 「将来 `Wrapper<i32>` も使うかもしれない」は、今genericにする理由になりますか？（04-1を思い出してください）

<details>
<summary>Solution</summary>

```rust
struct Wrapper {
    value: String,
}

impl Wrapper {
    fn new(value: String) -> Self {
        Wrapper { value }
    }

    fn get(&self) -> &str {
        &self.value
    }
}

fn main() {
    let w = Wrapper::new("hello".to_string());
    println!("{}", w.get());
}
```

`Wrapper<String>` を `Wrapper<T>` と書く手間や、`<String>` という型注釈が
あちこちに現れる煩わしさが無くなりました。**失ったものはありません**——
`T` が実際に複数の型になったことは、コードベースの中に一度もなかったからです。

04-1で見た「traitを今作ってよい理由」と同じ基準が、genericにも当てはまります。
**「今、実際に複数の型で使われているか」**が判断軸であり、
「将来使うかもしれない」だけでは理由になりません。

</details>

## Deep Dive: genericのコストは「無料」ではない

genericにすると、コンパイラは使われた型ごとに**コードを複製**します（単相化 / monomorphization）。
これにより実行時のコストはゼロですが、次のコストは残ります。

| コスト | 内容 |
| --- | --- |
| 読む人の負担 | 「`T`は何でもよいのか、どんな制約があるのか」を確認する必要がある |
| バイナリサイズ | 使われた型の数だけ、コンパイル後のコードが複製される |
| コンパイル時間 | 型ごとに個別にコンパイルされるため、型の種類が増えるとビルドが遅くなりうる |

これらは「genericを使うな」という意味ではありません。**重複が実際にあるなら、
これらのコストを払ってでもgenericにする価値があります。** 重複が無いなら、
コストだけを払うことになります。

## Exercise

**[`ex018_generic_benefit`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex018_generic_benefit)** — `cargo test -p ex018_generic_benefit` で判定します。

`min_max_i32` と `min_max_f64`（同じロジックの重複）を、1つのgeneric関数にまとめます。

## Challenge

自分のコードから、`T` が実質1種類の型でしか使われていない `struct Foo<T>` や `fn bar<T>()` を探してください。
見つかったら、genericを外しても本当に何も失わないか確認してください。

## Review

- [ ] 重複したロジックをgenericで1つにまとめられる
- [ ] 「今、複数の型で実際に使われているか」でgenericの要否を判断できる
- [ ] genericが実行時コストゼロでも、読みやすさ・ビルド時間のコストを持つことを説明できる

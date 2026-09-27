# Lesson 09-3: 型レベルでの制約表現

## Concept

型で表せる制約は、「どの型か」だけではありません。
**「長さはいくつか」**（const generics）や、**「誰が実装してよいか」**（sealed trait）も、型で表せます。

## Why?

実行時にチェックしていた条件を型に移すと、間違いはコンパイル時に見つかります。
Chapter 02 の「不正な状態を型で表現できなくする」考え方の延長です。

## Part 1: const generics

### Bad Example: 長さを実行時にチェックする

```rust
struct Vector(Vec<f64>);

impl Vector {
    fn dot(&self, other: &Vector) -> Result<f64, String> {
        if self.0.len() != other.0.len() {
            return Err("次元が違います".to_string());
        }
        Ok(self.0.iter().zip(&other.0).map(|(a, b)| a * b).sum())
    }
}

fn main() {
    let a = Vector(vec![1.0, 2.0]);
    let b = Vector(vec![1.0, 2.0, 3.0]);
    println!("{:?}", a.dot(&b)); // 実行して初めて間違いが分かる
}
```

### Think

> **問い**: 次元がコード上で決まっている（2次元・3次元が混ざらないはず）なら、この間違いをコンパイル時に見つけられないでしょうか？

<details>
<summary>Solution</summary>

```rust
struct Vector<const N: usize>([f64; N]);

impl<const N: usize> Vector<N> {
    fn dot(&self, other: &Vector<N>) -> f64 {
        self.0.iter().zip(&other.0).map(|(a, b)| a * b).sum()
    }
}

fn main() {
    let a = Vector([1.0, 2.0]);
    let b = Vector([3.0, 4.0]);
    println!("{}", a.dot(&b)); // Result も不要になった
}
```

`const N: usize` は、**値（ここでは長さ）を型パラメータにする**仕組みです。
`Vector<2>` と `Vector<3>` は別の型になるので、次元の違うベクトルの `dot` はコンパイルできません。

```rust,compile_fail,E0308
struct Vector<const N: usize>([f64; N]);

impl<const N: usize> Vector<N> {
    fn dot(&self, other: &Vector<N>) -> f64 {
        self.0.iter().zip(&other.0).map(|(a, b)| a * b).sum()
    }
}

fn main() {
    let a = Vector([1.0, 2.0]);
    let b = Vector([1.0, 2.0, 3.0]);
    a.dot(&b);
}
```

戻り値から `Result` が消えたことにも注目してください。**失敗しうる状況そのものが型で排除された**ので、
エラーを返す必要がなくなりました。

**限界**: const generics が使えるのは、長さが**コンパイル時に決まる**場合だけです。
ファイルから読んだデータのように、長さが実行時に決まるなら、`Vec` と実行時チェックに戻る必要があります
（Lesson 03-3 の typestate と同じ種類の制約です）。

</details>

## Part 2: sealed trait

### Bad Example: 公開した trait を誰でも実装できる

ライブラリが単位を表す `Unit` trait を公開し、`Meters` と `Seconds` にだけ実装しているとします。
利用者は、ライブラリが想定していない型にも `Unit` を実装できてしまいます。
その結果、ライブラリ側は「`Unit` を実装しているのは自分が知っている型だけ」という前提を置けず、
将来 `Unit` にメソッドを追加すると**利用者の実装を壊す**（破壊的変更になる）ことになります。

### Think

> **問い**: trait は公開して**使って**もらいたいが、**実装**はライブラリの中だけに限りたい。どうしますか？

<details>
<summary>Solution</summary>

```rust
mod units {
    mod sealed {
        pub trait Sealed {}
    }

    /// 利用者は Unit を使えるが、実装はできない。
    pub trait Unit: sealed::Sealed {
        const SYMBOL: &'static str;
    }

    pub struct Meters;
    pub struct Seconds;

    impl sealed::Sealed for Meters {}
    impl sealed::Sealed for Seconds {}
    impl Unit for Meters {
        const SYMBOL: &'static str = "m";
    }
    impl Unit for Seconds {
        const SYMBOL: &'static str = "s";
    }
}

fn format_value<U: units::Unit>(v: f64) -> String {
    format!("{v}{}", U::SYMBOL)
}

fn main() {
    println!("{}", format_value::<units::Meters>(3.0));
}
```

`Unit` は `sealed::Sealed` を supertrait に持ちます。`sealed` モジュールは非公開なので、
外からは `Sealed` という名前を**書くことすらできません**。`Sealed` を実装できない以上、`Unit` も実装できません。

```rust,compile_fail,E0277
mod units {
    mod sealed {
        pub trait Sealed {}
    }
    pub trait Unit: sealed::Sealed {
        const SYMBOL: &'static str;
    }
}

struct Mine;

impl units::Unit for Mine { // Mine は Sealed を実装していない（し、実装できない）
    const SYMBOL: &'static str = "x";
}

fn main() {}
```

これが **sealed trait** パターンです。言語機能ではなく、**モジュールの公開範囲（Lesson 02-4）を利用した慣用句**です。
得られるものは、「実装する型の一覧をライブラリが完全に把握できる」ことで、
trait にメソッドを追加しても利用者を壊しません（Lesson 03-4 の `#[non_exhaustive]` と同じ動機です）。

</details>

## Deep Dive: どちらも「使いどころが狭い」道具

| 道具 | 向いている場面 | 向いていない場面 |
| --- | --- | --- |
| const generics | 固定長の配列・行列、長さがコード上で決まる | 長さが実行時に決まる |
| sealed trait | ライブラリが実装の一覧を管理したい | 利用者に拡張してほしい trait（`Iterator` のようなもの） |

どちらも、「型で表せるから表す」のではなく、**実行時チェックや将来の破壊的変更という具体的な問題があるときに**使ってください。

## Exercise

**[`ex036_type_level_constraints`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex036_type_level_constraints)** — `cargo test -p ex036_type_level_constraints` で判定します。

| 課題 | 仕様 |
| --- | --- |
| `Vector<const N: usize>` | `dot`（内積）と `add`（成分ごとの和）を実装。次元の違う `dot` がコンパイルできないことは `compile_fail` doctest で確認 |
| `Unit`（sealed） | `Meters` / `Seconds` への実装は用意済み。`format_value::<U: Unit>` を実装。外部の型に `Unit` を実装できないことは `compile_fail` doctest で確認 |

doctest は**別crate**として実行されるので、sealed trait の効果を、本物の「ライブラリ利用者」の立場から確認できます。

## Challenge

`Vector<N>` に `fn concat<const M: usize>(&self, other: &Vector<M>) -> Vector<{ N + M }>` を足そうとすると、
stable の Rust ではどうなるか試してください。const generics で「計算した長さ」を扱うことには、まだ制約があります。

## Review

- [ ] const generics で、長さの不一致をコンパイル時に見つけられる
- [ ] const generics が使えない場面（長さが実行時に決まる）を説明できる
- [ ] sealed trait が、モジュールの公開範囲を利用した慣用句であることを説明できる
- [ ] sealed trait を使う動機（実装の一覧を管理し、破壊的変更を避ける）を説明できる

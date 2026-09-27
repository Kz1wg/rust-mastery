# Lesson 09-2: variance

## Concept

有効期限のあるチケットで考えてみます。

- **1年間有効なチケット**は、「今日1日だけ使えればよい」場面でも問題なく使えます。
  長く有効なものを、短い用途に使っても困る人はいません。
- では、「**1年間有効なチケットだけを入れる箱**」を、「今日1日有効なチケットの箱」として人に渡したらどうでしょう。
  受け取った人は、今日しか使えないチケットをその箱に入れるかもしれません。
  箱の持ち主は「中身は全部1年有効」と信じているので、明日その箱から取り出したチケットを使おうとして困ります。

Rust の参照でも、同じことが起きます。

| 場面 | Rust での例 | 許されるか |
| --- | --- | --- |
| 長く有効な参照を、短い期間の参照として**読むだけ** | `&'static str` を `&'a str` として渡す | **許される**（これを**共変**と言う） |
| 長く有効な参照だけが入る入れ物を、**書き込める形で**短い期間の入れ物として渡す | `&mut Vec<&'static str>` を `&mut Vec<&'a str>` として渡す | **許されない**（これを**不変**と言う） |

違いは、**読むだけか、書き込めるか**です。
このLessonの目標は「共変」「不変」という用語を覚えることではなく、**なぜ `&mut` だけ扱いが厳しいのか**を説明できるようになることです。

## Why?

variance は、普段は意識しなくても正しく働いています。意識するのは、
「なぜかlifetimeのエラーが出るが、理由が分からない」ときです。その多くは `&mut` の不変性が原因です。

## 共変: 長いものを短いものとして使える

```rust
fn pick<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let local = String::from("local!");
    let s: &'static str = "static";
    // s は &'static str だが、local と同じ短い 'a の参照として渡せる
    println!("{}", pick(&local, s));
}
```

`&'static str` を `&'a str` として扱っても、読むだけなら問題は起きません。
「より長く生きる」ものを「より短く生きる」ものとして扱うのは、常に安全側だからです。

## Bad Example: `&mut` 越しに短い参照を入れる

```rust,compile_fail,E0597
fn push_str<'a>(v: &mut Vec<&'a str>, s: &'a str) {
    v.push(s);
}

fn main() {
    let mut v: Vec<&'static str> = vec!["static"];
    {
        let local = String::from("local");
        push_str(&mut v, &local);
    } // local はここで破棄される
    println!("{v:?}");
}
```

## Think

> **問い**: もし `&mut Vec<&'static str>` を `&mut Vec<&'a str>`（短い `'a`）として渡せたら、何が起きますか？

<details>
<summary>Solution</summary>

`push_str` の中から見ると、`v` は「短い `'a` の参照を入れてよい `Vec`」です。そこで `&local` を `push` します。
しかし呼び出し元から見ると、`v` はずっと `Vec<&'static str>`、つまり「プログラム終了まで有効な参照だけが入っている `Vec`」です。

`local` が破棄された後も、`v` の中には `&local` が残り、**`&'static str` のはずの要素が解放済みのメモリを指す**ことになります。

**読むだけ**（`&T`）なら、長いものを短いものとして扱っても、何かを書き込まれる心配がないので安全です。
**書き込める**（`&mut T`）場合、短いものとして扱うと、短いものを書き込まれてしまいます。
だから `&mut T` は `T` について不変で、`T` の lifetime を1ミリも変えることを許しません。

コンパイラの判断は、「`v` の型が `Vec<&'static str>` なので `'a = 'static` でなければならない。
すると `&local` も `'static` でなければならないが、そうではない」という形で現れます（E0597）。
エラーメッセージは `local` の寿命について語りますが、**本当の原因は `&mut` の不変性**です。

</details>

## 代表的な型の variance

| 型 | `T`（や `'a`）について | 理由 |
| --- | --- | --- |
| `&'a T` | 共変 | 読むだけなので、長いものを短いものとして扱っても安全 |
| `&'a mut T` | `'a` には共変、**`T` には不変** | `T` の中に短いものを書き込まれるおそれがある |
| `Box<T>`, `Vec<T>` | 共変 | 所有しているので、他に同じ値を見ている者がいない |
| `Cell<T>`, `RefCell<T>` | 不変 | `&` 越しでも書き込めるため（interior mutability、Chapter 10） |
| `fn(T) -> U` | `T` には**反変**、`U` には共変 | 引数は「受け取れる範囲」が広いほうが安全なため（実用上まれなので、読み飛ばして構いません） |

09-1 の `PhantomData<T>` と `PhantomData<fn() -> T>` の違いも、ここに関わっています。
`PhantomData` の書き方は、自分の型の variance を決める手段でもあります。

## Deep Dive: エラーの読み方

lifetime のエラーで「この値は十分長く生きていない」と言われたのに、どう見ても参照を長く使っていないとき、
次を確認してください。

1. 途中に `&mut` がないか
2. その `&mut` の中身の型が、長い lifetime（`'static` など）に固定されていないか

多くの場合、`Vec<&'static str>` のような**型注釈を外すか、所有型（`Vec<String>`）にする**ことで解決します。
`&mut` の中身の lifetime を「縮める」ことはできないので、設計側で合わせる必要があります。

## Exercise

**[`ex035_variance`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex035_variance)** — `cargo test -p ex035_variance` で判定します。

| 関数 | 実装すること |
| --- | --- |
| `pick_longer` | 2つの `&str` のうち長い方を返す。テストでは `&'static str` とローカルな `String` の参照を混ぜて渡す（共変） |
| `push_word` | `&mut Vec<&'a str>` に単語を追加する。`Vec<&'static str>` にローカルな参照を入れられないことは `compile_fail` doctest で確認（不変） |
| `collect_short_words` | 単語のうち指定の長さ以下のものを、`Vec<String>` に**所有型として**集める |

`collect_short_words` が参照ではなく `Vec<String>` を返すと何が楽になるのか、
`push_word` の doctest と結びつけて説明してください。

## Challenge

`Cell<&'static str>` を `Cell<&'a str>` として扱えたら、何が起きるでしょうか。`&mut` の場合と同じ理屈で説明してください。

## Review

- [ ] 共変・不変を、「読むだけか、書き込めるか」で説明できる
- [ ] `&mut T` が `T` について不変である理由を、具体的な例で説明できる
- [ ] lifetime エラーの原因が `&mut` の不変性にあるとき、それに気づける

# Lesson 04-1: traitは「共通化」のためではない

## Concept

「2つの型で似たようなメソッドがある」ことは、traitを作る理由として**十分ではありません**。
traitは、**複数の実装を同じように扱いたい呼び出し側が実際にいる**ときに意味を持ちます。

## Why?

「とりあえずtraitにしておく」設計は、実装が1つしかないのに抽象化のコストだけを払います。
読む人は「他にも実装があるのか？」と探し、見つからず、遠回りをさせられます。

## Bad Example

```rust
trait Repository {
    fn save(&self, data: &str);
}

struct SqlRepository;

impl Repository for SqlRepository {
    fn save(&self, data: &str) {
        println!("INSERT: {data}");
    }
}

fn store(repo: &impl Repository, data: &str) {
    repo.save(data);
}

fn main() {
    let repo = SqlRepository;
    store(&repo, "hello");
}
```

## Problem

このコードベースには `SqlRepository` しか存在せず、`store` も `SqlRepository` でしか呼ばれません。
`Repository` traitと`store`のgeneric引数は、**何の柔軟性も生んでいません**。
`SqlRepository::save` を直接呼ぶのと、実行結果は何も変わりません。

## Think

> **問い**:
> 1. この `Repository` trait は何を保証していますか？その保証を必要としている呼び出し側は、今どこにありますか？
> 2. このtraitと `store` 関数を削除して、同じ動作をより単純に書き直してください。
> 3. 将来 `PostgresRepository` を追加する**かもしれない**ことは、今traitを作る理由になりますか？

<details>
<summary>Hint</summary>

「将来のために」traitを先回りして作ることと、「今、複数の実装を同じように扱う必要がある」ことは別です。
後者が無いなら、traitを削除して具象型のメソッドにしてください。

</details>

<details>
<summary>Solution</summary>

```rust
struct SqlRepository;

impl SqlRepository {
    fn save(&self, data: &str) {
        println!("INSERT: {data}");
    }
}

fn main() {
    let repo = SqlRepository;
    repo.save("hello");
}
```

trait も `store` 関数も消え、コードは短く、読みやすくなりました。**失ったものは何もありません**——
`Repository` を実装した別の型を受け取れるようになる、という柔軟性は、
今のところ誰も使っていなかったからです。

**「将来のため」は、traitを今作る理由になりません。** 実際に2つ目の実装が必要になったとき、
`impl` を trait に昇格させるのは簡単な作業です（既存の呼び出し側を壊さずに済むことも多い）。
逆に、使われない抽象化を後から削るのは、依存箇所を洗い出す手間がかかります。
**必要になってから作る方が、多くの場合安上がりです。**

ただし、次のような場合は「今」traitを導入する理由になります。

- **テストのためにモックへ差し替えたい** — `SqlRepository` の代わりに `MockRepository` を注入してテストしたい、という**今ある**要求
- **すでに2つ以上の実装がある** — `SqlRepository` と `InMemoryRepository` が両方存在し、呼び出し側がどちらでもよいように書きたい
- **ライブラリとして外部に公開する** — 利用者が独自の実装を差し込めるようにする、という設計上の約束（Chapter 16）

いずれも「今、複数の実装を同じように扱う具体的な理由がある」という点で共通しています。

</details>

## Deep Dive: traitは「保証」である

traitを設計するときは、次の問いを立ててください。

> **このtraitを実装した型は、呼び出し側に何を保証するのか？**

`Iterator` は「`next()` を呼べば、いつか `None` になるまで値が取れる」ことを保証します。
`Display` は「人間が読める形で文字列化できる」ことを保証します。

保証が曖昧な、あるいは「たまたま同じ名前のメソッドがある」というだけのtraitは、
呼び出し側にとって**何を信頼してよいか分からない**抽象化になります。

## Exercise

**`ex013_trait_purpose`** — `cargo test -p ex013_trait_purpose` で判定します。

このLessonとは逆に、**traitを導入する理由が実際にある**場面を実装します。
`PaymentMethod` trait を、2つの実装（`CreditCard` / `BankTransfer`）と、
両方を同じように扱う `process_payment` 関数とともに実装してください。

## Challenge

自分が過去に書いた（あるいはAIに書かせた）コードから、実装が1つしかないtraitを探してください。
見つかったら、削除しても呼び出し側が壊れないか確認してみてください。

## Review

- [ ] 「実装が2つ以上あるか、今すぐ必要な柔軟性があるか」でtraitの要否を判断できる
- [ ] 「将来のため」だけでは今traitを作る理由にならないことを説明できる
- [ ] traitが「呼び出し側への保証」であることを説明できる

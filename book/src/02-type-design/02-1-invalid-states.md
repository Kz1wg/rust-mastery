# Lesson 02-1: `bool` と文字列が表現してしまう不正な状態

## Concept

`admin: bool` は簡単で分かりやすく見えます。
この型が**表現できてしまう状態**の数と、**本当に意味のある状態**の数を比べてみましょう。

## Why?

型が表現できる状態の数が、有効な状態の数より多いと、
「無効な状態を検査するコード」が必要になります。検査は書き忘れられ、テストされ忘れます。

## Bad Example

```rust
struct Account {
    name: String,
    is_admin: bool,
    is_guest: bool,
}

fn main() {
    // コンパイルできてしまう。これは何を意味するのか？
    let odd = Account {
        name: "mystery".to_string(),
        is_admin: true,
        is_guest: true,
    };
    let _ = odd;
}
```

呼び出し側にも問題があります。

```rust
fn create(name: &str, is_admin: bool, is_guest: bool) { let _ = (name, is_admin, is_guest); }

fn main() {
    create("alice", true, false); // true, false は何を意味している？
}
```

## Problem

- `is_admin` と `is_guest` の組み合わせは4通りあるが、意味があるのは3通り（一般 / 管理者 / ゲスト）。**「管理者かつゲスト」が表現できてしまう。**
- 呼び出し側で `true, false` が何かが読み取れない。引数の順序を間違えてもコンパイルが通る。
- 「一般ユーザー」に相当する状態が、両方 `false` という**「何もないこと」**で表現されている。

## Think

> **問い**:
> 1. 上の `Account` は、型として何通りの状態を表現でき、そのうち有効なのは何通りですか？
> 2. 将来 `is_suspended: bool` を追加したら、状態の数はどうなりますか？
> 3. この問題を型で解決するなら、どう書きますか？

<details>
<summary>Hint</summary>

「どれか1つ」を表したいときに使える、Rustの型は何でしょうか。

</details>

<details>
<summary>Solution</summary>

1. 2 × 2 = **4通り**。有効なのは3通り。
2. 2 × 2 × 2 = **8通り**。有効な組み合わせはさらに絞られる。**フラグが増えるほど、無効な状態が指数的に増える。**
3. **`enum`**：

```rust
enum Role {
    User,
    Admin,
    Guest,
}

struct Account {
    name: String,
    role: Role,
}

fn create(name: &str, role: Role) -> Account {
    Account { name: name.to_string(), role }
}

fn main() {
    let a = create("alice", Role::Admin); // 読めばわかる
    let _ = a;
}
```

`Role` は3通りしか表現できず、無効な状態が**そもそも書けません**。

さらに、`match` は網羅性が検査されます。バリアントを増やしたとき、対応を書き忘れるとコンパイルエラーになります。

```rust,compile_fail,E0004
enum Role {
    User,
    Admin,
    Guest,
}

fn can_delete(role: Role) -> bool {
    match role {
        Role::Admin => true,
        Role::User => false,
    }
}
```

**注意**: `_ => false` のようなワイルドカードを書くと、この検査が無効になります。
新しいバリアントが**黙って `false` 扱い**になり、コンパイラの助けを自分で捨ててしまいます。
「将来のバリアントにも安全側に倒したい」という意図があるときだけ、意識して使ってください。

</details>

## Deep Dive: すべての `bool` が悪いわけではない

次の場合は、`bool` のままで問題ありません。

| 状況 | 理由 |
| --- | --- |
| 他のフラグと**独立**している（`is_verified` と `Role` など） | 組み合わせが全て有効。無効な状態が生じない |
| 単純なON/OFFで、今後増える見込みが薄い | enum にする利点より、冗長さのコストが大きい |
| 内部の実装詳細（private） | 呼び出し側に公開されず、影響範囲が狭い |

`bool` 引数が公開APIに出るときの読みにくさ（`create("alice", true, false)`）は、
**引数が1つで、関数名が意味を示している**（例: `set_visible(true)`）なら、それほど問題になりません。
複数の `bool` が並ぶ場合は、enum や builder への置き換えを検討してください。

判断の軸は「**その `bool` の組み合わせに、意味のない状態があるか**」です。

## Exercise

**`ex005_invalid_states`** — `cargo test -p ex005_invalid_states` で判定します。
（`cargo test` で自動判定するため、型定義や公開APIのシグネチャはあらかじめ用意してあります。本体の `todo!()` を実装してください。）

| 課題 | 仕様 |
| --- | --- |
| 信号機 | `enum Light { Red, Yellow, Green }` は用意済み。`fn next(self) -> Self`（Green → Yellow → Red → Green）を実装 |
| 注文 | `enum OrderStatus`（Pending / Paid / Shipped / Delivered）は用意済み。`all_statuses()` と `label()` を実装 |

元の `struct Order { paid: bool, shipped: bool, delivered: bool }` が表現できてしまった
無効な状態を、任意で `NOTES.md` に書き出してください（型を自分で書き換える体験は、本文の Think / Solution で行います）。

## Challenge

`struct Settings { dark_mode: bool, notifications: bool }` は enum にすべきでしょうか？
「すべき」「すべきでない」のそれぞれの立場から、理由を書いてください。

## Review

- [ ] `bool` フィールドが n 個あるとき、表現できる状態が 2ⁿ 通りになることを説明できる
- [ ] `enum` で不正な状態を書けなくする設計ができる
- [ ] `bool` のままでよい場合の判断基準を説明できる

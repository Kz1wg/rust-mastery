# Lesson 02-2: newtype pattern

## Concept

`String` は「なんでも文字列」です。ID もメールアドレスも名前も、同じ `String` で表せてしまいます。
この**「同じ型」であること**が、バグの原因になることがあります。

## Why?

型が同じなら、コンパイラは取り違えを検出できません。
検出できない間違いは、テストか本番で初めて見つかります。

## Bad Example

```rust
struct User {
    id: String,
    email: String,
}

fn send_mail(id: String, email: String) {
    println!("to user {id} <{email}>");
}

fn main() {
    let user = User {
        id: "u-123".to_string(),
        email: "alice@example.com".to_string(),
    };
    // 引数の順序を取り違えている。コンパイルは通る。
    send_mail(user.email, user.id);
}
```

## Problem

`id` と `email` は意味がまったく違いますが、型は同じ `String` です。
引数の順序を間違えても、フィールドを入れ替えても、**コンパイラは何も言いません**。

## Think

> **問い**:
> 1. 型を分けて、この取り違えをコンパイルエラーにするには、どう書きますか？
> 2. その設計には、どんな**コスト**がありますか？
> 3. 「newtype にしない方がよい」場面はありますか？

<details>
<summary>Hint</summary>

1つのフィールドしか持たない `struct` を作ると、型は別物になります。

</details>

<details>
<summary>Solution</summary>

```rust
struct UserId(String);
struct Email(String);

struct User {
    id: UserId,
    email: Email,
}

fn send_mail(id: UserId, email: Email) {
    println!("to user {} <{}>", id.0, email.0);
}

fn main() {
    let user = User {
        id: UserId("u-123".to_string()),
        email: Email("alice@example.com".to_string()),
    };
    send_mail(user.id, user.email); // OK
}
```

引数を入れ替えると、コンパイルエラーになります。

```rust,compile_fail,E0308
struct UserId(String);
struct Email(String);

fn send_mail(id: UserId, email: Email) {
    let _ = (id, email);
}

fn main() {
    let id = UserId("u-123".to_string());
    let email = Email("alice@example.com".to_string());
    send_mail(email, id); // 順序を取り違えた
}
```

**newtype のコスト**:

| コスト | 内容 |
| --- | --- |
| ボイラープレート | 内側の値へのアクセス、`Display`、`Clone`、`Eq` などを `derive` や手書きで用意する必要がある |
| 変換の手間 | `String` を必要とするAPIには、`.0` や `as_str()` などで取り出す |
| 学習コスト | 型が増え、読む人が理解すべき概念が増える |

**newtype を使わなくてよい場面**:

| 場面 | 理由 |
| --- | --- |
| 関数内だけで完結するローカルな値 | 取り違える機会が少ない |
| 引数が1つだけで、関数名が意味を示している | 取り違えようがない |
| 同じ型が複数あるが、**順序を間違えても意味が壊れない** | 型を分ける価値が小さい |
| プロトタイプ・使い捨てコード | 設計コストが見合わない |

**判断の軸**は「取り違えたとき、どれだけ被害が大きく、どれだけ気づきにくいか」です。
ID・金額・単位（メートルとフィート）のように、**取り違えが静かに壊れる**値は newtype の価値が高いです。

</details>

## Deep Dive: `Deref` で newtype を「透明」にしてはいけない

ボイラープレートを減らそうと、`Deref<Target = String>` を実装したくなるかもしれません。
しかし、これは **`Email` を `String` のように使えてしまう**ので、型を分けた意味が薄れます。
`Deref` は「スマートポインタ」のための仕組みで、newtype で意味の異なる型を作る目的には向きません。

必要なメソッドだけを、**明示的に**公開します。

```rust
struct Email(String);

impl Email {
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn main() {
    let e = Email("a@example.com".to_string());
    assert_eq!(e.as_str(), "a@example.com");
}
```

## Exercise

**`ex006_newtype`**（Phase 4 で提供）

| 課題 | 仕様 |
| --- | --- |
| `Meters(f64)` と `Feet(f64)` | `fn add_meters(a: Meters, b: Meters) -> Meters`。`Feet` を渡すとコンパイルエラーになる（`compile_fail` doctest で判定） |
| `From<Feet> for Meters` | 単位変換を `From` で実装（1 ft = 0.3048 m） |
| 「newtypeにしなかった」ケース | 課題の中に、あえて newtype にしなくてよい箇所を1つ含める。どれか、なぜかを `NOTES.md` に書く |

## Challenge

実際に書いたことのあるコードで、`String` や `i64` が複数の意味で使われている箇所を探してください。
newtype にするとしたら、どこが最もバグの防止効果が高いでしょうか。

## Review

- [ ] newtype が「取り違えのバグ」をコンパイルエラーにする仕組みを説明できる
- [ ] newtype のコストと、使わなくてよい場面を説明できる
- [ ] `Deref` で newtype を透明にしない理由を説明できる

# Lesson 04-5: 不要なtraitを見抜く（レビュー演習）

## Concept

このLessonには「新しい知識」はありません。04-1〜04-4で学んだ判断基準を使って、
他人（あるいはAI）が書いたコードをレビューする練習です。

## Why?

自分でゼロから設計するときより、**既にあるコードのtraitが必要かどうかを判断する**方が、
実務では頻度が高い作業です。AIにコードを書かせる時代は、なおさらです。

## Bad Example（AIが書いた想定のコード）

次のコードは、「ログ出力を抽象化したい」というプロンプトに対してAIが書きそうな設計です。

```rust
trait Logger {
    fn log(&self, message: &str) -> String;
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) -> String {
        format!("[LOG] {message}")
    }
}

struct App<L: Logger> {
    logger: L,
}

impl<L: Logger> App<L> {
    fn new(logger: L) -> Self {
        App { logger }
    }

    fn run(&self) -> String {
        self.logger.log("starting up")
    }
}

fn main() {
    let app = App::new(ConsoleLogger);
    println!("{}", app.run());
}
```

## Think

このコードを読んで、次を自分で（AIに頼らず）考えてください。

> **問い**:
> 1. `Logger` の実装はいくつありますか？
> 2. `App<L: Logger>` は、`ConsoleLogger` 以外のロガーで呼ばれる予定がありますか
>    （コードのどこかにその証拠はありますか）？
> 3. このtraitと `App<L>` のgenericパラメータを取り除くと、コードはどう変わりますか？
> 4. 04-1で見た「traitを今作ってよい理由」（テストでの差し替え、既に複数実装がある、
>    外部公開）のうち、当てはまるものはありますか？

<details>
<summary>Hint</summary>

「App構造体をgenericにする」という選択は、いかにも「拡張性の高い設計」に見えます。
しかし、実際に複数の `L` で `App` が使われている箇所がコード中にあるか、探してください。

</details>

<details>
<summary>Solution</summary>

`Logger` の実装は `ConsoleLogger` の1つだけで、`App<L>` も `ConsoleLogger` でしか
インスタンス化されていません。04-1の基準に照らすと、このtraitと`App<L>`のgenericパラメータは
**「将来のため」の先回りにすぎず、今削除しても失うものがありません**。

```rust
struct App {
    // ロガーが1種類しかないなら、フィールドとして持つ必要すらない場合もある。
    // ここでは元の構造をなるべく保ったまま、traitだけを取り除く。
}

impl App {
    fn new() -> Self {
        App {}
    }

    fn run(&self) -> String {
        format!("[LOG] {}", "starting up")
    }
}

fn main() {
    let app = App::new();
    println!("{}", app.run());
}
```

コードは短くなり、`App<L: Logger>` のような型パラメータを読み解く負担も無くなりました。
04-1のチェックリストのどれにも当てはまらない限り、この単純化が正しい選択です。

**ここで重要なのは、「AIが書いたコードだから信用する／しない」ではなく、**
**「このtraitは何を保証し、それを必要としている呼び出し側が実在するか」を毎回問うこと**です。
それは、人間が書いたコードをレビューするときと同じ基準です。

</details>

## Exercise

**[`ex017_review_logger`](https://github.com/Kz1wg/rust-mastery/tree/main/exercises/ex017_review_logger)** — `cargo test -p ex017_review_logger` で判定します。

上の `Logger` / `App<L>` の例を、trait を使わない設計に書き直してください
（`ConsoleLogger` のロジックを直接メソッドとして実装します）。

## Challenge

AIに小さな機能（例: 「通知を送る仕組みを作って」）を実装させてみてください。
出てきたコードに、実装が1つしかないtraitがあれば、04-1〜04-5の基準で見直してください。

## Review

- [ ] genericパラメータ付きの構造体（`App<L: Logger>`）が、実装が1つしかないときは不要な複雑さになることを説明できる
- [ ] 04-1〜04-4の基準を、自分で書いたのではないコードのレビューにも適用できる

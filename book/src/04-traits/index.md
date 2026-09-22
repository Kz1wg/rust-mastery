# 04 Traits

trait は「共通化のための道具」ではありません。
**「この型はこういう振る舞いができる、と保証する」宣言**です。

この章では、traitを設計する際に立てるべき問いを扱います。

## この章の到達目標

- traitを「とりあえず共通化のため」に使わない
- generic（静的ディスパッチ）とtrait object（動的ディスパッチ）の違いと、それぞれのコスト・制約を説明できる
- associated typeとgenericパラメータのどちらでtraitを設計すべきか判断できる
- 標準trait（`From`, `TryFrom`, `AsRef`, `Display`, `Default`）を実装してよい場合とそうでない場合を区別できる
- 不要なtraitを見抜き、具象型に戻せる

## Lesson一覧

| Lesson | 問い |
| --- | --- |
| [04-1 traitは「共通化」のためではない](04-1-trait-purpose.md) | 何を抽象化し、何を保証させるか |
| [04-2 generic vs trait object](04-2-generic-vs-dyn.md) | 静的/動的ディスパッチのコストと制約 |
| [04-3 associated type](04-3-associated-type.md) | genericパラメータとの違いは何か |
| [04-4 標準trait実装の設計](04-4-standard-traits.md) | どのtraitを実装するのが誠実か |
| [04-5 不要なtraitを見抜く](04-5-unnecessary-traits.md) | 具象型で十分なケース（レビュー演習） |

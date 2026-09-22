# 05 Generics

genericは「重複を消すための道具」です。ただし、型パラメータを1つ増やすたびに、
読む人が「Tは何でもよいのか、何が保証されているのか」を追う負担が増えます。

## この章の到達目標

- genericにする理由（重複の除去）が実際にあるかを判断できる
- trait boundsを、関数の中身が本当に必要とする分だけに絞れる
- `impl Trait` を引数位置・戻り値位置で使い分け、それぞれの制約を説明できる
- GAT（Generic Associated Types）が必要になる最小限の場面を説明できる

## Lesson一覧

| Lesson | 問い |
| --- | --- |
| [05-1 genericにするメリットはあるか](05-1-generic-benefit.md) | 重複削減か、複雑化か |
| [05-2 trait boundsの設計](05-2-trait-bounds.md) | boundは最小か、読みやすいか |
| [05-3 `impl Trait`](05-3-impl-trait.md) | 引数位置と戻り値位置の違い |
| [05-4 GATの基本用途](05-4-gat-basics.md) | なぜ通常のassociated typeでは足りないのか |

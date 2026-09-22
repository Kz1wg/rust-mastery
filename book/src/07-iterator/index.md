# 07 Iterator

`Iterator` は、Rustで最もよく使われる抽象化の1つです。
この章では、「いつadapterを使い、いつforループの方がよいか」「自作iteratorが必要になるのはいつか」
「遅延評価は何を約束しているか」を扱います。

## この章の到達目標

- iterator adapterの連鎖と`for`ループを、読みやすさの観点で使い分けられる
- 自作の `Iterator` 実装が必要になる場面を説明できる
- iteratorが遅延評価であることを理解し、`collect()` が本当に必要な場面を判断できる
- `&self` を借用するiteratorを返す関数の、戻り値の型とlifetimeの関係を説明できる

## Lesson一覧

| Lesson | 問い |
| --- | --- |
| [07-1 adapterの組み合わせ](07-1-adapters-vs-for.md) | forループと何が違い、いつforの方が良いか |
| [07-2 `Iterator` trait](07-2-custom-iterator.md) | 自作iteratorが必要になるのはいつか |
| [07-3 遅延評価とallocation](07-3-laziness-and-allocation.md) | `collect` はどこで必要か |
| [07-4 借用するiteratorを返す](07-4-borrowing-iterators.md) | 戻り値の型とlifetimeの関係 |

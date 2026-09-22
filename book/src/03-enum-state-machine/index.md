# 03 Enum & State Machine

Rustの `enum` は、「複数の状態のうち**ちょうど1つ**」を表し、**状態ごとに異なるデータ**を持てます。
この性質は、状態機械の設計にそのまま使えます。

## この章の到達目標

- 「複数の `Option` フィールド」が表現してしまう不正な状態を指摘できる
- 状態ごとにデータを持つ `enum` を設計できる
- 状態遷移の API を、複数の設計から比較して選べる
- typestate pattern の利点と**限界**を説明できる
- 公開する `enum` に `#[non_exhaustive]` を付けるべきか判断できる

## Lesson一覧

| Lesson | 問い |
| --- | --- |
| [03-1 enumで状態を表す](03-1-enum-states.md) | 複数の `Option` は何を壊すか |
| [03-2 状態遷移をenumで設計する](03-2-transitions.md) | 不正な遷移をどう扱うか |
| [03-3 typestate pattern](03-3-typestate.md) | 遷移の誤りをコンパイル時に検出できるか |
| [03-4 `#[non_exhaustive]` と将来の拡張](03-4-non-exhaustive.md) | バリアント追加は破壊的変更か |

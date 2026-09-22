# 09 Advanced Type System

この章では、型システムの少し奥にある道具を扱います。どれも「知っていると格好いい機能」ではなく、
**ある種の問題を型で表現しようとしたときに、必要に迫られて出てくるもの**です。

## この章の到達目標

- `PhantomData` が必要になる場面と、`#[derive]` と組み合わせたときの罠を説明できる
- variance（共変・不変）が、なぜ安全性のために必要なのかを説明できる
- const generics と sealed trait で、「値の範囲」や「実装できる型」を型で制約できる
- zero-cost abstraction の「zero」が何を指し、何を指さないかを説明できる

## Lesson一覧

| Lesson | 問い |
| --- | --- |
| [09-1 `PhantomData`](09-1-phantom-data.md) | 使っていない型パラメータをなぜ持つのか |
| [09-2 variance](09-2-variance.md) | `&'a T` が共変で、`&mut T` が不変である意味 |
| [09-3 型レベルでの制約表現](09-3-type-level-constraints.md) | const generics、sealed trait |
| [09-4 zero-cost abstraction](09-4-zero-cost.md) | 何がzeroで、何がzeroではないか |

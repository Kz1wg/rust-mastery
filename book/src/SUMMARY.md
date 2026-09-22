# Summary

[はじめに](introduction.md)

# 00 Introduction

- [この教材の使い方](00-introduction/00-1-how-to-use.md)
- [`cargo test` で学ぶ]()
- [診断問題]()
- [「Rustらしさ」とは]()

# 01〜03 基礎の設計

- [01 Ownership & Borrowing](01-ownership/index.md)
  - [01-1 moveは何を守っているのか](01-ownership/01-1-move.md)
  - [01-2 借用のエラーを読む](01-ownership/01-2-borrow-errors.md)
  - [01-3 所有権で設計する](01-ownership/01-3-ownership-design.md)
  - [01-4 シグネチャを比較する](01-ownership/01-4-compare-signatures.md)
- [02 Type Design](02-type-design/index.md)
  - [02-1 boolと文字列が表現してしまう不正な状態](02-type-design/02-1-invalid-states.md)
  - [02-2 newtype pattern](02-type-design/02-2-newtype.md)
  - [02-3 構築時検証](02-type-design/02-3-parse-dont-validate.md)
  - [02-4 フィールドの公開範囲とAPI](02-type-design/02-4-visibility.md)
- [03 Enum & State Machine](03-enum-state-machine/index.md)
  - [03-1 enumで状態を表す](03-enum-state-machine/03-1-enum-states.md)
  - [03-2 状態遷移をenumで設計する](03-enum-state-machine/03-2-transitions.md)
  - [03-3 typestate pattern](03-enum-state-machine/03-3-typestate.md)
  - [03-4 `#[non_exhaustive]` と将来の拡張](03-enum-state-machine/03-4-non-exhaustive.md)

# 04〜07 抽象化の設計

- [04 Traits]()
- [05 Generics]()
- [06 Error Handling]()
- [07 Iterator]()

# 08〜11 発展

- [08 Lifetimes]()
- [09 Advanced Type System]()
- [10 Concurrency]()
- [11 Async Rust]()

# 12〜16 構造と品質

- [12 Module & Architecture]()
- [13 Testing]()
- [14 Macros]()
- [15 Unsafe Rust]()
- [16 Library Design]()

# 総合

- [17 Practical Projects]()
- [18 Final Project]()

# Appendix

- [A. Compiler Error 読解集]()
- [B. AIレビュー用プロンプト集]()
- [C. 用語集]()

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

- [04 Traits](04-traits/index.md)
  - [04-1 traitは「共通化」のためではない](04-traits/04-1-trait-purpose.md)
  - [04-2 generic vs trait object](04-traits/04-2-generic-vs-dyn.md)
  - [04-3 associated type](04-traits/04-3-associated-type.md)
  - [04-4 標準trait実装の設計](04-traits/04-4-standard-traits.md)
  - [04-5 不要なtraitを見抜く](04-traits/04-5-unnecessary-traits.md)
- [05 Generics](05-generics/index.md)
  - [05-1 genericにするメリットはあるか](05-generics/05-1-generic-benefit.md)
  - [05-2 trait boundsの設計](05-generics/05-2-trait-bounds.md)
  - [05-3 `impl Trait`](05-generics/05-3-impl-trait.md)
  - [05-4 GATの基本用途](05-generics/05-4-gat-basics.md)
- [06 Error Handling](06-error-handling/index.md)
  - [06-1 `Result` の型設計](06-error-handling/06-1-result-design.md)
  - [06-2 独自Error型](06-error-handling/06-2-custom-error-types.md)
  - [06-3 error propagation](06-error-handling/06-3-error-propagation.md)
  - [06-4 libraryとapplicationのerror設計](06-error-handling/06-4-library-vs-application.md)
- [07 Iterator](07-iterator/index.md)
  - [07-1 adapterの組み合わせ](07-iterator/07-1-adapters-vs-for.md)
  - [07-2 `Iterator` trait](07-iterator/07-2-custom-iterator.md)
  - [07-3 遅延評価とallocation](07-iterator/07-3-laziness-and-allocation.md)
  - [07-4 借用するiteratorを返す](07-iterator/07-4-borrowing-iterators.md)

# 08〜11 発展

- [08 Lifetimes](08-lifetimes/index.md)
  - [08-1 lifetime annotationは何を主張しているのか](08-lifetimes/08-1-what-annotations-say.md)
  - [08-2 構造体が参照を持つ設計](08-lifetimes/08-2-structs-with-references.md)
  - [08-3 elisionと `'static`](08-lifetimes/08-3-elision-and-static.md)
  - [08-4 HRTB入門](08-lifetimes/08-4-hrtb.md)
- [09 Advanced Type System](09-advanced-type-system/index.md)
  - [09-1 `PhantomData`](09-advanced-type-system/09-1-phantom-data.md)
  - [09-2 variance](09-advanced-type-system/09-2-variance.md)
  - [09-3 型レベルでの制約表現](09-advanced-type-system/09-3-type-level-constraints.md)
  - [09-4 zero-cost abstraction](09-advanced-type-system/09-4-zero-cost.md)
- [10 Concurrency](10-concurrency/index.md)
  - [10-1 `Send` / `Sync`](10-concurrency/10-1-send-sync.md)
  - [10-2 共有と可変性](10-concurrency/10-2-arc-mutex.md)
  - [10-3 interior mutability](10-concurrency/10-3-interior-mutability.md)
  - [10-4 メッセージパッシング](10-concurrency/10-4-message-passing.md)
- [11 Async Rust](11-async/index.md)
  - [11-1 `Future` とは何か](11-async/11-1-what-is-a-future.md)
  - [11-2 `Pin` / `Unpin`](11-async/11-2-pin.md)
  - [11-3 asyncと所有権](11-async/11-3-async-and-ownership.md)
  - [11-4 asyncを使うべきか](11-async/11-4-async-vs-threads.md)

# 12〜16 構造と品質

- [12 Module & Architecture](12-architecture/index.md)
  - [12-1 module分割の基準](12-architecture/12-1-module-boundaries.md)
  - [12-2 lib / bin の分離](12-architecture/12-2-lib-and-bin.md)
  - [12-3 workspaceの設計](12-architecture/12-3-workspaces.md)
  - [12-4 依存の方向と公開範囲](12-architecture/12-4-dependencies-and-visibility.md)
- [13 Testing](13-testing/index.md)
  - [13-1 テスト可能な設計](13-testing/13-1-testable-design.md)
  - [13-2 unit / integration / doctest](13-testing/13-2-kinds-of-tests.md)
  - [13-3 失敗ケースのテスト](13-testing/13-3-testing-failures.md)
- [14 Macros](14-macros/index.md)
  - [14-1 マクロが必要な瞬間](14-macros/14-1-when-macros.md)
  - [14-2 declarative macro](14-macros/14-2-macro-rules.md)
  - [14-3 procedural macro 入門](14-macros/14-3-proc-macros.md)
- [15 Unsafe Rust]()
- [16 Library Design]()

# 総合

- [17 Practical Projects]()
- [18 Final Project]()

# Appendix

- [A. Compiler Error 読解集]()
- [B. AIレビュー用プロンプト集]()
- [C. 用語集]()

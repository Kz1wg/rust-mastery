# はじめに

この教材の目的は、Rustの知識を増やすことではありません。

> 「Rustの文法を知っている人」から
> 「型・所有権・trait・エラー設計・抽象化・モジュール構成を使って、**自分でRustらしい設計ができる人**」へ

コードを見たときに、次のように考えられるようになることを目指します。

- この状態は型で表現した方がいいのでは？
- ここはtraitにする必要があるのか？
- この所有権設計は本当に自然か？
- このErrorはどこで処理すべきか？
- genericにするメリットはあるか？
- このAPIは利用者に何を保証しているか？

## 対象者

struct / enum / match / Result / Option を書け、所有権の基本を理解し、小さなRustアプリを作ったことがある方。
文法の入門解説はしません。

## 進め方

サイドバーの各章を順に進めます。

1. まず [00-2](00-introduction/00-2-cargo-test.md) で演習の進め方を、[00-3](00-introduction/00-3-diagnostic.md) の診断問題で前提知識を確かめてください
2. Chapter 01〜16 で、1つずつ考え方を学びます。各 Lesson には `cargo test` で判定できる演習があります
3. Chapter 17 の実践プロジェクトで、複数の章の考え方を組み合わせます
4. Chapter 18 の Final Project で、何もないところから設計します

付録は、必要なときに引く辞書として使ってください。
コンパイルエラーに出会ったら [Appendix A](appendix/a-compiler-errors.md)、AI にレビューさせるときは [Appendix B](appendix/b-ai-review-prompts.md)、用語が分からなくなったら [Appendix C](appendix/c-glossary.md) です。

演習とプロジェクトのコードは、GitHub のリポジトリ [Kz1wg/rust-mastery](https://github.com/Kz1wg/rust-mastery) にあります（準備のしかたは [Lesson 00-2](00-introduction/00-2-cargo-test.md)）。
全体の計画は、リポジトリの [ROADMAP.md](https://github.com/Kz1wg/rust-mastery/blob/main/ROADMAP.md) にあります。

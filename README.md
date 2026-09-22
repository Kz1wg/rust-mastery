# Rust Mastery

**Rust中級者が、Rustで「設計」できる上級者になるための実践型学習教材。**

Rustの文法を教える教材ではありません。
「この状態は型で表現した方がいいか？」「このtraitは本当に必要か？」「このAPIは利用者に何を保証するか？」——
そう考えられるようになることが目的です。

> 開発状況: **Phase 3 完了**（設計 + mdBook + 第1〜3章）。演習crate（`cargo test` で判定）は Phase 4 で追加します。現状、各Lessonの Exercise 節は**仕様のみ**で、対応するcrateはまだありません。

## 対象者

- struct / enum / match / Result / Option を書ける
- 所有権の基本を理解している
- Cargoを使え、小規模なRustアプリを作った経験がある

## 学習ロードマップ

全体像は [ROADMAP.md](ROADMAP.md)、思想は [DESIGN.md](DESIGN.md) を参照してください。

```text
基礎の設計       01 Ownership → 02 Type Design → 03 Enum & State Machine
抽象化の設計     04 Traits → 05 Generics → 06 Error Handling → 07 Iterator
発展             08 Lifetimes → 09 Advanced Type System → 10 Concurrency → 11 Async
構造と品質       12 Architecture → 13 Testing → 14 Macros → 15 Unsafe → 16 Library Design
総合             17 Practical Projects → 18 Final Project
```

## 必要な環境

| ツール | 推奨・要件 | 動作確認済み |
| --- | --- | --- |
| Rust | 1.85 以上を推奨（edition 2024 が使える最小バージョン）。現時点の教材コードは edition 2021 | 1.98.1 / 1.75.0 |
| mdBook | 0.4系・0.5系のどちらでも動作 | 0.5.4 / 0.4.40 |

（動作確認の内容: `mdbook test` と `python3 tools/check_error_codes.py` が第1〜3章で通ること）

## mdBookの起動

```bash
cd book
mdbook serve --open     # http://localhost:3000
mdbook build            # 静的HTMLを book/book/ に出力
mdbook test             # 本文中のRustコードを検証
```

`compile_fail,EXXXX` のエラーコード一致は `mdbook test` では検証されないため、別途:

```bash
python3 tools/check_error_codes.py   # どのディレクトリからでも実行可
```

## 演習の実行（Phase 4で提供）

```bash
cargo test -p ex001_newtype                      # 1つの演習を判定
cargo run --bin check-exercises                  # 全演習の進捗を確認
```

演習の骨組みは `exercises/`、模範解答は `solutions/` にあります。**まず自分で解き、答えを見るのは最後にしてください。**

## 推奨学習方法

1. **Think を先に。** 各Lessonの「考える」問いに、コードを見る前に自分の言葉で答える
2. **Hint は段階的に。** 詰まったら1つずつ開く。Solutionは最後
3. **エラーを読む。** コンパイルエラーは「直す」前に、何が保証されようとしているかを説明する
4. **複数の解を比較する。** 「どれが正解か」ではなく「なぜそれを選ぶか」
5. **AIを使ってよい。** ただし AIの出力は必ずレビューし、設計上の問題を自分で指摘する（Appendix B）

## プロジェクト構成

```text
book/        mdBookプロジェクト（教材本文）
exercises/   演習（骨組み + テスト）        ← Phase 4
solutions/   模範解答                       ← Phase 4
projects/    実践プロジェクト・Final Project ← Phase 7〜8
tools/       check-exercises など           ← Phase 4
```

詳細は [ARCHITECTURE.md](ARCHITECTURE.md) を参照。

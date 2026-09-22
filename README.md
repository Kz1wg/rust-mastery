# Rust Mastery

**Rust中級者が、Rustで「設計」できる上級者になるための実践型学習教材。**

Rustの文法を教える教材ではありません。
「この状態は型で表現した方がいいか？」「このtraitは本当に必要か？」「このAPIは利用者に何を保証するか？」——
そう考えられるようになることが目的です。

> 開発状況: 設計・mdBook・第1〜9章・全37演習（ex001〜ex037）・`check-exercises`（進捗確認CLI）を実装済みです。Chapter 10（Concurrency）以降はまだです。

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

## 演習の実行

```bash
cargo test -p ex001_move_semantics    # 1つの演習を判定（ex012 以外の全演習）
```

`ex012_non_exhaustive` だけは、独立した2crate（`lib`/`app`）構成なので:

```bash
cd exercises/ex012_non_exhaustive && cargo test
```

ルートで `-p` なしの `cargo test` を実行すると、`check-exercises` だけが対象になります
（`default-members` の設定）。未着手の演習は `todo!()` で失敗するのが正常な状態なので、
全演習をまとめて確認したいときは次の `check-exercises` を使ってください。

## 進捗の確認

```bash
cargo run -p check-exercises                  # 全演習の進捗を表示（章ごと）
cargo run -p check-exercises -- --lesson 02   # 章で絞り込む
```

`✅` 完了 / `❌ N failed` 一部失敗（着手済み） / `⬜ not started` 未着手（`todo!()`のまま）
/ `⚠️ build error` コンパイルエラー、の4通りで表示します。

## 解答の検証（メンテナ向け）

全演習の解答が正しく動くことを、骨組みへ一時的に上書きして確認できます
（終了時に骨組みへ自動で復元されます）。2つの方法があります。

```bash
cargo run -p check-exercises -- --solutions   # test のみ。進捗表示と同じ枠組みで手軽に
./tools/verify_solutions.sh                    # test のみ
./tools/verify_solutions.sh --fmt --clippy     # fmt / clippy も含める（CIで使用）
```

## CI

`.github/workflows/ci.yml` が、push・PR ごとに次を確認します。

- mdBookのビルドと `mdbook test`
- `compile_fail` のエラーコード検証（`tools/check_error_codes.py`）
- 演習の骨組みがビルドできること
- 解答が test / fmt / clippy を通ること（`tools/verify_solutions.sh --fmt --clippy`）

> このワークフローは、実際にGitHub Actions上で実行して確認したものではありません。
> 各ステップに相当するコマンドをローカルで実行し、通ることは確認しています。
> 最初のpush後、Actionsタブで結果を確認してください。

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
exercises/   演習（骨組み + テスト）。exNNN_<name>/ ごとに1crate
solutions/   模範解答（exercises/ と同じディレクトリ名で src/ のみ）
projects/    実践プロジェクト・Final Project（Phase 7〜8、未着手）
tools/       check-exercises（進捗CLI）、verify_solutions.sh、check_error_codes.py
```

詳細は [ARCHITECTURE.md](ARCHITECTURE.md) を参照。

## ライセンス

MIT License。詳細は [LICENSE](LICENSE) を参照してください。

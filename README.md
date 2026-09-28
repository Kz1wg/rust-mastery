# Rust Mastery

**Rust を書いたことがある人が、Rust で「設計」できるようになるための学習教材です。**

📖 **Web で読む: https://kz1wg.github.io/rust-mastery/**

この教材は、Rust の文法を教えるものではありません。
コードを見たときに、次のように考えられるようになることを目指します。

- この状態は、型で表現した方がいいのでは？
- ここは、trait にする必要があるのか？
- この所有権の設計は、本当に自然か？
- この Error は、どこで処理すべきか？
- generic にするメリットはあるか？
- この API は、利用者に何を保証しているか？

## この教材の特徴

| 特徴 | 内容 |
| --- | --- |
| **答えを先に見せない** | 各 Lesson は「悪い例 → 何が問題か → 考える → ヒント → 解答」の順。自分で考えてから開きます |
| **コンパイルエラーが教材** | エラーを「直して終わり」にせず、コンパイラが何を守ろうとしているかを読みます |
| **正解を1つに絞らない** | 複数の実装を並べ、所有権・性能・読みやすさ・API の使いやすさなどで比べます |
| **`cargo test` で判定** | 61 の演習は、テストが通れば完成。進み具合は `check-exercises` で一覧できます |
| **AI を使ってよい** | AI にコードを書かせ、それをレビューして直す練習をします（プロンプト集つき） |
| **依存ゼロ** | 演習は標準ライブラリだけで動きます。async も最小のランタイムを自作して学びます |

## 対象者

- `struct` / `enum` / `match` / `Result` / `Option` を書ける
- 所有権の基本を理解している
- Cargo を使って、小さな Rust アプリを作ったことがある

自信がなければ、[診断問題（Lesson 00-3）](https://kz1wg.github.io/rust-mastery/00-introduction/00-3-diagnostic.html)で前提知識を確かめてから始めてください。

## 内容

| 分野 | 章 |
| --- | --- |
| はじめに | 00 Introduction（使い方、`cargo test` の読み方、診断問題、「Rustらしさ」とは） |
| 基礎の設計 | 01 Ownership & Borrowing / 02 Type Design / 03 Enum & State Machine |
| 抽象化の設計 | 04 Traits / 05 Generics / 06 Error Handling / 07 Iterator |
| 発展 | 08 Lifetimes / 09 Advanced Type System / 10 Concurrency / 11 Async Rust |
| 構造と品質 | 12 Module & Architecture / 13 Testing / 14 Macros / 15 Unsafe Rust / 16 Library Design |
| 実践 | 17 Practical Projects（CLI、CSV パーサ、ログ解析、HTTP クライアント、キャッシュ、tokio による非同期取得、ライブラリ） |
| 総合 | 18 Final Project（要件から設計して、小さなアプリを作る） |
| 付録 | A. Compiler Error 読解集 / B. AI レビュー用プロンプト集 / C. 用語集 |

各章の Lesson と学ぶ順番は [ROADMAP.md](ROADMAP.md) にあります。

## 始め方

### 1. 本文を読む

本文は [Web ページ](https://kz1wg.github.io/rust-mastery/) で読めます。
手元で読みたい場合は、「[本文を手元で開く](#本文を手元で開く)」を見てください。

### 2. リポジトリを手元に用意する

演習は手元で解きます。

```bash
git clone https://github.com/Kz1wg/rust-mastery.git
cd rust-mastery
cargo run -p check-exercises    # 全演習の状態を表示する（最初は全部 ⬜）
```

### 3. 演習を解く

各 Lesson の最後に、対応する演習があります。

```bash
cargo test -p ex001_move_semantics    # 1つの演習を判定する
```

1. `exercises/ex001_move_semantics/tests/tests.rs` を読む（テストも問題文の一部です）
2. `exercises/ex001_move_semantics/src/lib.rs` の `todo!()` を実装する
3. `cargo test` が通れば完成

詳しい進め方と、テストの失敗の読み方は [Lesson 00-2](https://kz1wg.github.io/rust-mastery/00-introduction/00-2-cargo-test.html) にあります。

> **解答について**: 模範解答は `solutions/` にあります。リポジトリを見れば誰でも開けますが、
> **自分で解いてテストが通るまでは、開かないでください**。
> 解いた後に見比べると、「同じ結果を出す別の書き方」から多くを学べます。

## 学び方のすすめ

1. **Think を先に。** 問いには、ヒントを開く前に自分の言葉で答える
2. **ヒントは1つずつ。** 解答は最後に開く
3. **エラーを読む。** コンパイルエラーは、直す前に「何が保証されようとしているか」を説明する（[Appendix A](https://kz1wg.github.io/rust-mastery/appendix/a-compiler-errors.html)）
4. **比べる。** 「どれが正解か」ではなく「なぜそれを選ぶか」を考える
5. **AI を使ってよい。** ただし、AI の出力は必ず自分でレビューする（[Appendix B](https://kz1wg.github.io/rust-mastery/appendix/b-ai-review-prompts.html)）

## 必要な環境

| ツール | 必要なバージョン | 確認したバージョン |
| --- | --- | --- |
| Rust | 1.75 以上（最新の stable を推奨）。教材のコードは edition 2021 | 1.98.1 / 1.95.0 / 1.75.0 |
| mdBook（本文を手元で開く場合だけ） | 0.4 系・0.5 系のどちらでも | 0.5.4 / 0.4.40 |

`rustfmt` と `clippy` も使います。`rust-toolchain.toml` があるので、rustup を使っていれば自動で用意されます。

## 演習と実践プロジェクトの詳細

### 演習（Chapter 01〜16）

```bash
cargo test -p ex001_move_semantics            # 1つの演習を判定する
cargo run -p check-exercises                  # 全演習の進み具合（章ごと）
cargo run -p check-exercises -- --lesson 02   # 章で絞り込む
```

| 表示 | 意味 |
| --- | --- |
| ✅ | 全てのテストが通った |
| ⬜ N todo | 失敗しているテストは、すべて `todo!()`（まだ書いていない所）によるもの。骨組みのままならこの表示 |
| ❌ N failed | `todo!()` 以外の理由で失敗しているテストがある（答えが違う、など） |
| ⚠️ build error | コンパイルできない |

次の3つの演習は、中に複数の crate を持つ構成（入れ子の workspace）なので、そのディレクトリの中で実行します。

```bash
cd exercises/ex012_non_exhaustive && cargo test
cd exercises/ex048_workspace && cargo test
cd exercises/ex055_derive_macro && cargo test
```

リポジトリのルートで `-p` を付けずに `cargo test` を実行すると、`check-exercises` 自身のテストだけが走ります。
未着手の演習は失敗するのが正常な状態だからです。全体を見たいときは `check-exercises` を使ってください。

### 実践プロジェクト（Chapter 17）

`projects/` は、ルートとは別の workspace です。
演習は依存 crate を使いませんが、実践プロジェクトでは実務と同じく依存 crate を使う場面があるため、範囲をこのディレクトリに限っています。

```bash
cd projects
cargo test -p p01_wordstat
cargo run -p p01_wordstat -- --top 3 notes.txt
```

P6 は tokio を使うので、初めて `cargo test` するときに crates.io からダウンロードします（ネットワーク接続が必要です）。

### Final Project（Chapter 18）

小さなアプリケーションを、要件から設計して作ります。主役はコードより**設計の過程**です。
設計メモは、[docs/final-project-template.md](docs/final-project-template.md) をコピーして書きます。

例題「図書室の貸出管理」の参考実装が `projects/final_lending/` にあります。
**Lesson 18-7 まで進むまで、開かないでください。**

```bash
cd projects
cargo test -p final_lending
cargo run -p final_lending -- --data /tmp/lib.tsv list
```

## リポジトリの構成

```text
book/        教材の本文（mdBook）
exercises/   演習（骨組み + テスト）。exNNN_<name>/ ごとに1つの crate
solutions/   演習の模範解答（exercises/ と同じディレクトリ名で src/ だけ）
projects/    実践プロジェクト（p01〜p07）と Final Project の参考実装（final_lending）。別の workspace
docs/        Final Project のテンプレート、演習の一覧、執筆ガイド
tools/       check-exercises（進捗表示）、解答の検証スクリプト、エラーコードの検証
```

教材の設計の考え方は [DESIGN.md](DESIGN.md)、技術的な構成は [ARCHITECTURE.md](ARCHITECTURE.md) にあります。

---

## メンテナ向け

### 本文を手元で開く

```bash
cd book
mdbook serve --open     # http://localhost:3000 で開き、編集すると自動で再読み込み
mdbook build            # 静的な HTML を book/book/ に出力
mdbook test             # 本文中の Rust コードを検証
```

`compile_fail,EXXXX` と書いたエラーコードが実際と一致するかは、`mdbook test` では確かめられないため、別に実行します。

```bash
python3 tools/check_error_codes.py
```

### 解答の検証

解答を骨組みへ一時的に重ねて、テストが通るかを確かめます。終わると、骨組みは自動で元に戻ります。

```bash
cargo run -p check-exercises -- --solutions   # 演習: test のみ
./tools/verify_solutions.sh --fmt --clippy     # 演習: test + fmt + clippy
./tools/verify_projects.sh --fmt --clippy      # 実践プロジェクトと Final Project
```

### CI

`.github/workflows/ci.yml` が、push と pull request のたびに次を確かめます。

- 本文のビルドと `mdbook test`
- `compile_fail` のエラーコード（`tools/check_error_codes.py`）
- 演習の骨組みがビルドでき、rustfmt 済みであること
- 演習の解答が test / fmt / clippy を通ること
- 実践プロジェクトの解答と Final Project の参考実装が test / fmt / clippy を通ること

### Web ページの公開（GitHub Pages）

`.github/workflows/pages.yml` が、`main` ブランチの `book/` が変わるたびに本文をビルドし、GitHub Pages に公開します。
最初の1回だけ、リポジトリの **Settings → Pages → Build and deployment → Source** を **GitHub Actions** にしてください。

### 本文を書く・直すとき

[docs/writing-guide.md](docs/writing-guide.md) の基準に合わせてください。
対象は「Rust を書いたことがある初中級者」です。用語は、名前より先に「何をしているか」を説明します。

## ライセンス

MIT License。詳しくは [LICENSE](LICENSE) を見てください。

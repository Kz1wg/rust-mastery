# ARCHITECTURE.md — 技術設計

mdBook、演習、テストシステムの技術設計。**Phase 1 時点の設計**であり、実装状況は各節の「状態」に記す。

## 1. 要件

| # | 要件 | 満たし方 |
| --- | --- | --- |
| R1 | 教材を `mdbook serve` で読める | `book/` にmdBookプロジェクト |
| R2 | 本文中のコードが壊れていないことを機械的に保証する | `mdbook test`（`compile_fail` も検証対象） |
| R3 | 演習は `cargo test` で判定できる | `exercises/` の演習ごとにcrate |
| R4 | 学習者が答えを誤って見ない | 解答は `solutions/` に分離 |
| R5 | 出題側の間違い（解けない演習）を防ぐ | 全演習について「解答を当てるとテストが通る」ことをCIで検証 |
| R6 | 進捗を確認できる | `check-exercises`（将来） |
| R7 | 依存crateは最小限 | 演習crateは標準ライブラリのみを原則とする |

## 2. リポジトリ構成

```text
rust-mastery/
├── README.md
├── DESIGN.md / ROADMAP.md / ARCHITECTURE.md
├── rust-toolchain.toml        # stable + rustfmt/clippy（Phase 2で追加済み）
├── Cargo.toml                 # workspace（exercises/* と tools/*）※Phase 4で追加（最初の演習crateと同時に）
├── book/                      # mdBookプロジェクト
│   ├── book.toml
│   └── src/
│       ├── SUMMARY.md
│       ├── 00-introduction/
│       ├── 01-ownership/
│       │   ├── index.md       # 章の概要・到達目標
│       │   ├── 01-move.md     # Lesson
│       │   └── ...
│       └── appendix/
├── exercises/                 # 学習者が解く演習（骨組み + テスト）
│   └── ex001_newtype/
│       ├── Cargo.toml
│       ├── exercise.toml      # メタデータ（後述）
│       ├── src/lib.rs         # todo!() を含む骨組み
│       └── tests/tests.rs     # 判定テスト
├── solutions/                 # 模範解答（srcのみ。crateではない）
│   └── ex001_newtype/src/lib.rs
├── projects/                  # 実践プロジェクト・Final Project（Phase 7〜8）
├── tools/
│   ├── check_error_codes.py   # compile_fail のエラーコード検証（Phase 3で追加済み）
│   └── check-exercises/       # 進捗確認・解答検証CLI（Phase 4）
└── .github/workflows/ci.yml   # CI（Phase 4）
```

### 設計判断

**解答をcrateにしない理由**: 演習crateと解答crateが同じworkspaceに入るとcrate名が衝突し、学習者の `cargo test` の対象も曖昧になる。解答は `src/` のみを置き、検証時に演習crateへ重ねて実行する（§5）。

**workspaceの `default-members`**: 演習の骨組みは初期状態でテストが失敗する。ルートで `cargo test` を実行すると全演習が失敗して混乱するため、`default-members` は `tools/check-exercises` のみとし、演習は `-p` で個別に指定する。

## 3. mdBook設計

### 3.1 設定（`book/book.toml`）

```toml
[book]
title = "Rust Mastery"
language = "ja"
src = "src"

[rust]
edition = "2021"          # mdbook test が使うedition

[output.html]
default-theme = "light"
git-repository-url = ""   # 公開時に設定
no-section-label = true   # 章番号（00〜18, A〜C）はタイトル文字列側で管理するため、
                           # mdBookのサイドバー自動連番（パートを跨いで連番になり、
                           # タイトルの番号とずれる）を無効化する
```

### 3.2 Lessonのテンプレート

`<details>` で「考える時間」を作る。mdBookはインラインHTMLをそのまま出力するため追加の拡張は不要。

````markdown
# Lesson 02-1: `bool` が表現してしまう不正な状態

## Concept
（1〜2文。論点を「問い」の形で）

## Why?
（現実の問題と結びつける）

## Bad Example
```rust,compile_fail,E0000   ← 意図的に悪い例は compile_fail か ignore を明示
```

## Problem
（何が起きるか。誤用例・テストで示す）

## Think
> **問い**: ...

<details>
<summary>Hint 1</summary>

...
</details>

<details>
<summary>Solution</summary>

**採用した設計**と**理由**、**別解**、**trade-off** を必ず書く。
</details>

## Deep Dive
## Exercise
`exercises/ex001_newtype` を開き、`cargo test -p ex001_newtype` が通るようにしてください。
## Challenge
## Review
- [ ] 〜を説明できる
````

### 3.3 コードの検証方針

| 種類 | 記法 | 検証 |
| --- | --- | --- |
| 動くコード | ` ```rust ` | `mdbook test` でコンパイル・実行 |
| 意図的にコンパイルエラーになる例 | ` ```rust,compile_fail,E0505 ` | `mdbook test` が「**コンパイルに失敗すること**」を検証 |
| 実行しないが型は通したい例 | ` ```rust,no_run ` | コンパイルのみ |
| 断片（検証不能） | ` ```rust,ignore ` | 検証しない。**理由を本文に書く**（乱用しない） |

エラーコード（`E0505` など）は読者への情報として付ける。**ただし、コードが一致するかの検証はstable版では行われない**（Rust 1.75 で、誤ったコードを指定しても `mdbook test` が通ることを確認済み。検証はnightlyのrustdocのみと理解している）。
そのため「エラーコードまで一致する」ことは機械的には保証されない。代わりに `tools/check_error_codes.py` が、本文中の `compile_fail,EXXXX` ブロックを `rustc` で実際にコンパイルし、指定したエラーコードが出ることを検証する（`python3 tools/check_error_codes.py`。CIにも組み込む）。

### 3.4 バージョン

| ツール | 要件 | 動作確認 | 備考 |
| --- | --- | --- | --- |
| Rust | **1.85 以上を推奨**（edition 2024が使える最小バージョン）。現時点では edition 2021 を使用 | 1.98.1（利用者の環境）、1.75.0（開発サンドボックス） | 2024 edition への移行は Phase 4 で再検討 |
| mdBook | 0.4系・0.5系 | 0.5.4（利用者の環境）、0.4.40（開発サンドボックス） | `book.toml` は0.4系の記法のまま、0.5.4 でも `mdbook test` が通ることを確認 |

`rust-toolchain.toml` で stable チャンネルと `rustfmt` / `clippy` を指定し、必要な最小バージョンを `README.md` に明記する。

## 4. 演習システム設計

### 4.1 演習crateの規約

| 項目 | 規約 |
| --- | --- |
| 名前 | `exNNN_<snake_case>`（例: `ex001_newtype`）。NNNは通し番号 |
| 依存 | 原則ゼロ（標準ライブラリのみ） |
| 骨組み | 公開APIのシグネチャは完成させ、本体は `todo!()` |
| テスト | `tests/tests.rs`（公開APIだけをテスト。内部実装に依存させない） |
| 解答 | `solutions/exNNN_<name>/src/` に配置 |

### 4.2 テストの書き方

テストは**振る舞い**と**設計上の制約**の両方を判定する。

1. **振る舞いテスト**: 通常の入出力
2. **境界・失敗ケース**: `Result::Err` の種類まで確認（Error設計の演習）
3. **設計制約のテスト**: 「この型が不正な値を保持できない」ことを、公開APIから構築できないことで担保する

> 「コンパイルできてはいけないコード」は `cargo test` だけでは判定しにくい。
> 必要な演習では、テストの `compile_fail` doctest（`///` の ```` ```compile_fail ````）を使う。これは `cargo test` の一部として実行される。

### 4.3 `exercise.toml`（メタデータ）

```toml
id = "ex001_newtype"
title = "newtypeでIDを型付けする"
lesson = "02-2"              # ROADMAPのLesson ID
difficulty = 1               # 1〜5
concepts = ["newtype", "type safety"]
prerequisites = ["ex000_..."] # 省略可
```

`check-exercises` はこのファイルを読んで一覧・進捗を表示する。
メタデータを演習crateの外（本文）に持たせないのは、本文の書き換えで進捗管理が壊れないようにするため。

### 4.4 `check-exercises`（Phase 4で実装）

```bash
cargo run --bin check-exercises                 # 全演習を実行し進捗を表示
cargo run --bin check-exercises -- --lesson 02  # 章で絞り込み
cargo run --bin check-exercises -- --solutions  # 解答検証（メンテナ向け）
```

出力例（設計イメージ）:

```text
01 Ownership & Borrowing
  ✅ ex001_move_semantics        (01-1)
  ❌ ex002_borrow_errors         (01-2)   2 failed
  ⬜ ex003_signatures            (01-4)   not started

Progress: 1 / 3
```

- 「未着手」の判定は、`todo!()` の panic（`not yet implemented`）で失敗している演習を `not started`、それ以外の失敗を `failed` とする。
- 実装は `std::process::Command` で `cargo test --manifest-path ... --message-format=json` を実行して結果を集約する。依存crateは `toml` のみを許容する（メタデータ解析）。

### 4.5 解答検証（R5）

```text
1. exercises/exNNN を一時ディレクトリへコピー
2. solutions/exNNN/src を src に上書き
3. cargo test を実行
4. 通らなければ「出題が壊れている」として失敗
```

これをCIで全演習に対して実行する。

## 5. AIレビュー型Lessonの構造

```text
exercises/exNNN_ai_review/
├── PROMPT.md          # 学習者がAIに渡す問題文
├── src/lib.rs         # 「AIが書いた想定」のコード（設計上の問題を含む）
├── REVIEW.md          # 学習者が問題点を書くテンプレート
└── tests/tests.rs     # 修正後のコードが満たすべき仕様
```

学習フロー: AIにPROMPT.mdを解かせる（または `src/lib.rs` を用いる）→ 問題点を `REVIEW.md` に書く → 修正 → `cargo test`。
評価は「テストが通ること」＋「REVIEW.md に設計上の問題を3つ以上、理由つきで挙げられていること」（後者は自己チェックリスト）。

## 6. CI（Phase 4）

| ジョブ | コマンド | 目的 |
| --- | --- | --- |
| book-build | `mdbook build book` | 本のビルド |
| book-test | `mdbook test book` | 本文コードの検証 |
| error-codes | `python3 tools/check_error_codes.py` | `compile_fail` のエラーコード一致の検証 |
| fmt | `cargo fmt --all --check` | rustfmt |
| clippy | `cargo clippy -p check-exercises -- -D warnings` | 演習ツールのlint（骨組みは `todo!()` を含むため対象外） |
| solutions | `cargo run --bin check-exercises -- --solutions` | 解答検証 |

## 7. 未決事項

| 項目 | 状況 |
| --- | --- |
| edition 2024への移行 | Phase 4で再検討。利用者の環境（Rust 1.98.1）なら検証可能だが、開発サンドボックス（1.75）では検証できない点に注意 |
| mdBook 0.5系への対応 | 0.5.4 で `mdbook test` が通ることを確認済み。`book.toml` を0.5系の推奨記法へ更新するかは未検討 |
| 演習のヒント段階を `check-exercises hint` で出すか | 保留。まず本文の `<details>` で運用する |
| 公開（GitHub Pages）| Phase 2以降に検討 |

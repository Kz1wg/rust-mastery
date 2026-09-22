# DESIGN.md — 教材の思想・対象者・到達目標

## 1. この教材が目指すもの

> 「Rustの文法を知っている人」から
> 「型システム・所有権・trait・エラー設計・抽象化・モジュール構成を使って、**自分でRustらしい設計ができる人**」へ

Rustの知識を増やすことは目的ではない。学習者がコードを見たときに、次の問いを自然に立てられるようになることが目的である。

- この状態は型で表現した方がいいか？
- ここはtraitにする必要があるのか？
- この所有権設計は本当に自然か？
- このErrorはどこで処理すべきか？
- genericにするメリットはあるか？
- この抽象化は本当に必要か？
- このAPIは利用者に何を保証しているか？

**Rustの文法を教える教材ではなく、Rustで設計する思考法を身につける教材**である。

## 2. 想定する学習者

| 項目 | 想定 |
| --- | --- |
| 文法 | struct / enum / match / Result / Option を書ける |
| 所有権 | 基本は理解している（moveと借用の区別ができる） |
| ツール | Cargoを使える |
| 経験 | 小規模なRustアプリを作ったことがある |

**やらないこと**: 文法の入門解説に紙面を割かない。前提知識の確認は、演習の冒頭にある「診断問題」（数問）で足りるようにする。

## 3. 最終到達目標

学習終了時に、次を**自力で判断し、理由を説明できる**状態を目指す。

| 領域 | 到達状態 |
| --- | --- |
| 型設計 | struct/enumの使い分け、newtype、不正な状態を表現しにくい型、enum/型による状態機械 |
| Ownership / Borrowing | 仕組みの説明、borrow checkerエラーの読解、lifetimeを必要以上に恐れない、annotationが必要な理由の説明 |
| Trait | 適切なtrait設計、generic vs trait objectの区別、associated typeの使いどころ、静的/動的ディスパッチの説明、「とりあえず共通化」のためにtraitを使わない |
| Generics | generic type設計、trait bounds設計、associated type vs generic parameter、`impl Trait`、GATの基本用途 |
| Error Handling | `Result`設計、独自Error型、error propagation、library/applicationでの設計差 |
| Iterator | adapterの組み合わせ、`Iterator` trait、独自iterator |
| Architecture | module分割、workspace、lib/binの分離、依存の方向、公開範囲 |
| Advanced Rust | `Send`/`Sync`、interior mutability、`Cell`/`RefCell`、`Arc`/`Mutex`、async、`Pin`、unsafe、zero-cost abstraction の「なぜ存在するのか」 |

Advanced Rustは「使いこなせる」ではなく「**なぜ存在するのかを説明できる**」を到達点とする。

## 4. 教育設計の原則

### P1. 答えを先に教えない
各Lessonは「問題 → 考える → ヒント → 解答」の順に進む。`admin: bool` を見せて `enum Role` を説明するのではなく、
「`admin: bool` で表現できてしまう状態にはどんな問題があるか」を先に問う。

### P2. 機能ではなく問題から導入する
難しい機能を「知識として暗記」させない。すべての機能は
**「なぜこの機能が必要になったのか？」** という具体的な困りごとから導入する。

### P3. 正解を1つに絞らない（trade-offを教える）
設計問題は複数の実装を並べ、次の観点で比較させる。

`ownership` / `allocation` / `API flexibility` / `readability` / `performance` / `abstraction`

「newtypeが正解」とは限らない。**なぜその設計を選んだか**を説明できることを評価する。

### P4. Compiler Errorは教材である
エラーは「直す」だけで終わらせず、次の5段階で読ませる。

1. 何がborrow（あるいはmove）されているか
2. なぜできないのか
3. コンパイラは何を保証しようとしているか
4. どう直せるか
5. どの直し方が設計として望ましいか

### P5. AI時代を前提にする
AIの利用は禁止しない。むしろ次のサイクルを学習法として組み込む。

```text
AIにコードを書かせる → 読む → 問題点を発見する → 設計を説明する → 自分で直す → cargo test
```

### P6. 動くコード ≠ Rustらしいコード ≠ 短いコード
「Rustらしさ」は短さではない。次を総合して判断させる。

`correctness` / `readability` / `maintainability` / `API design` / `type safety` / `ownership` / `performance` / `abstraction` / `error handling`

## 5. Lessonの構造

```text
Concept → Why? → Bad Example → Problem → Think → Hint
       → Solution → Deep Dive → Exercise → Challenge → Review
```

| 節 | 役割 | 書き方の規則 |
| --- | --- | --- |
| Concept | 今日扱う設計上の論点を1〜2文で | 機能名ではなく「問い」で始める |
| Why? | なぜ学ぶ必要があるか | 現実のバグ・保守性の問題と結びつける |
| Bad Example | 一見動くが問題があるコード | 実際にありがちな書き方にする |
| Problem | Bad Exampleで起きる具体的な問題 | 再現できる形（テストや誤用例）で示す |
| Think | 学習者に考えさせる問い | `<details>` の**外**に置く |
| Hint | 段階的なヒント | `<details>` で折りたたむ |
| Solution | 解答と設計理由 | 「別解」と「trade-off」を必ず併記 |
| Deep Dive | 仕組み・背景（ABI、MIR、標準ライブラリの実装など） | 読み飛ばしても先へ進める |
| Exercise | `cargo test` で判定できる演習 | `exercises/` の対応crateを指す |
| Challenge | 設計を選ぶ・拡張する発展課題 | 正解が複数あってよい |
| Review | 振り返りチェックリスト | 「〜を説明できる」形式 |

## 6. コード例の品質基準

- **「なぜこのコードなのか」を説明できないコードは載せない。**
- すべての例は `rustfmt` / `clippy` クリーンを目標とする。
- Bad Exampleは意図的に悪いコードなので、`compile_fail` や `ignore` の理由をコメントに書く。
- 依存crateは必要最小限。使う場合は Lesson 内で理由を説明する。
- 本文中のコードは可能な限り `mdbook test` で検証する（詳細は ARCHITECTURE.md）。

## 7. 非目標（やらないこと）

- Rust言語リファレンスの代替
- 暗記用の機能一覧・チートシート化
- 1つの「正しい設計」を押し付けること
- 一度に巨大な教材本文を生成すること（Phaseごとに小さく作り、検証してから進める）

## 8. 決定ログ

| 日付 | 決定 | 理由 |
| --- | --- | --- |
| 2026-09-22 | 章構成は原案（00〜18）を維持し、Appendixを追加 | 実装前に章順を変える根拠が弱い。実際に教材を書いて問題が出たら見直す（詳細は ROADMAP.md） |
| 2026-09-22 | 演習crateのeditionは 2021 で開始 | 開発環境（サンドボックス）のRustが古く、2024 edition を検証できないため。移行は Phase 4 で再検討 |
| 2026-09-22 | 解答は `solutions/` に分離し、演習crateには置かない | 学習者が誤って答えを見ないため。検証は `check-exercises --solutions` で自動化する |
| 2026-09-22 | 動作確認済みバージョンを README / ARCHITECTURE に記録（Rust 1.98.1・mdBook 0.5.4 は利用者環境、Rust 1.75.0・mdBook 0.4.40 は開発サンドボックス） | 「推奨バージョン」と「実際に確認したバージョン」を区別して記録するため |
| 2026-09-22 | `book.toml` に `no-section-label = true` を追加 | mdBookのサイドバー自動連番（パートを跨いで連番）が、タイトル文字列の章番号（00〜18, A〜C）とずれるため無効化（利用者からの指摘で発見） |

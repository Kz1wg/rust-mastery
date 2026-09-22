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
| 2026-09-22 | Phase 4 を ex001〜ex004（Chapter 01）から着手し、Chapter 02・03 前に一度確認をもらう進め方にした | 12個まとめて作ってからずれに気づくと手戻りが大きいため |
| 2026-09-22 | ex003 の演習では「シグネチャを自分で決める」という本文の narrative に反し、骨組みでシグネチャを固定した | ARCHITECTURE.md の演習crate規約（公開APIのシグネチャは完成させ、本体のみ todo!()）に合わせるため。本文の思考課題としての「自分で決める」体験は Lesson 内の Think / Solution で担保する |
| 2026-09-22 | Chapter 02 の演習（ex005〜ex008）を実装 | 予定どおり Chapter 01 の確認後に着手 |
| 2026-09-22 | 骨組み（`todo!()`）状態で `cargo clippy --workspace` を走らせると `unused variable` で失敗することを確認し、ARCHITECTURE.md §6 の「clippyは演習骨組みを対象外にする」という既存方針の妥当性を裏付けた | 検証手順の誤り（全crateではなく一部だけ解答を重ねていた）で発覚。以後、fmt/clippy検証は必ず全crateに解答を重ねた状態で行う |
| 2026-09-22 | Chapter 03 の演習（ex009〜ex012）を実装し、全12演習が揃った | 予定どおり |
| 2026-09-22 | ex012 用に、ルートworkspaceから独立したnested workspace（lib/app）を作った | Lesson 03-4 の主題（別crateの利用者への影響）を機械的にテストするには、本当に別crateが必要だったため |
| 2026-09-22 | GitHubへの公開に備え、LICENSE（MIT）、`.github/workflows/ci.yml`、`tools/verify_solutions.sh` を追加した | ユーザーからの明示的な依頼。ライセンスは相談せずMITを選んだ（学習教材として一般的な選択）。ユーザー自身の判断で変更可能 |
| 2026-09-22 | `.github/workflows/ci.yml` はローカルで同等コマンドの成功のみ確認し、GitHub Actions上での実行では確認していない | このサンドボックスにGitHub Actionsを実行する手段がないため。最初のpush後、Actionsタブでの確認をユーザーに委ねる |
| 2026-09-22 | GitHub Actionsで実行し、Node.js 20非推奨の警告（actions/checkout@v4, peaceiris/actions-mdbook@v2）を確認。actions/checkout@v5へ更新し、peaceiris/actions-mdbook（2024年から更新停止）はmdBookバイナリの直接curlダウンロードに置き換えた | ユーザーからの実行結果報告。peaceiris/actions-mdbookはNode24対応の見込みが薄いメンテナンス停止プロジェクトのため、サードパーティNode actionへの依存自体をなくす方針にした |
| 2026-09-22 | `ubuntu-latest` が2026-10-19からUbuntu 26に移行するというnoticeが出ているが、対応は保留（現時点ではUbuntu 24のまま動作しており、移行後の互換性は未検証） | 情報提供のみのnoticeであり、今すぐ変更が必要な警告ではないため |
| 2026-09-22 | `check-exercises` を実装した | ユーザーからの依頼どおり |
| 2026-09-22 | `check-exercises` の依存を `toml` クレートからゼロ依存（自前の最小パーサー）に変更した | `toml` の依存先 `indexmap` が新しいバージョンでedition2024を要求し、開発環境のRust 1.75でビルドできなかったため。exercise.toml/Cargo.tomlは自分たちが書く単純な形式なので、自前パーサーで十分と判断した |
| 2026-09-22 | ルートworkspaceに `default-members = ["tools/check-exercises"]` を設定した | ARCHITECTURE.mdが当初から計画していたとおり。これで、ルートで `-p` なしの `cargo build`/`cargo test` を実行しても、未着手の演習を巻き込まなくなった |
| 2026-09-22 | `check-exercises --solutions` と `tools/verify_solutions.sh` の両方を残した（前者はfmt/clippyを検証しない） | 目的が異なる：前者は進捗確認の延長として手軽に使うもの、後者はCIのゲート（fmt/clippyも含む）。統合すると複雑になるため、役割を分けたまま両方残す判断をした |
| 2026-09-22 | Chapter 04（Traits）の本文5 Lessonと演習5つ（ex013〜ex017）を実装。全17演習になった | 予定どおり |
| 2026-09-22 | ex013で再び `todo!()` 内の `{}` 補間バグを踏んだ（今回は `{var}` 名前付き補間で `cannot find value` エラー）。5回目 | 何度も同じミスをしているため、Lesson作成時のチェックリストとして明記した（docs/exercise-index.md） |
| 2026-09-22 | `solutions/` へのfmt書き戻し忘れで `verify_solutions.sh --fmt` が失敗する問題を発見・修正した（ex013） | 一時的に `exercises/` 側でfmtして満足し、`solutions/` への書き戻しを忘れていた。`check-exercises --solutions` はfmtを検証しないため見逃していた——2つのツールを併用している価値が実際に発揮された事例 |
| 2026-09-22 | Chapter 05（Generics）の本文4 Lessonと演習4つ（ex018〜ex021）を実装。全21演習になった | 予定どおり |
| 2026-09-22 | `-> impl Trait` を返す関数の骨組みで、本体をまるごと `todo!()` にするとopaque type推論が失敗する（`() is not an iterator`）ことを発見した（ex020） | rustcの制約。外側の式構造は骨組みに残し、`todo!()` は内側の式にのみ埋め込む方針にした。今後の演習作成でも注意すべき点としてdocs/exercise-index.mdに記録した |
| 2026-09-22 | Chapter 06（Error Handling）の本文4 Lessonと演習4つ（ex022〜ex025）を実装。全25演習になった | 予定どおり |
| 2026-09-22 | mainがResultを返す例（06-3 Deep Dive）が実際に実行され、存在しないファイルの読み込みで失敗した | `rust`のみのコードブロックはmdbook testで実行される。概念を示すだけの例は`no_run`にする必要があることを再確認した |
| 2026-09-22 | ex024の初期設計は、`?`の代わりに`.map_err()`を使っていたため、テーマである「`?`がFromを経由する」ことを実際には検証していなかった | 演習を書く際、テストではなく「その演習が検証すべき概念」を先に確認すべきだった教訓。再設計してFrom経由を実際にテストする形にした |
| 2026-09-22 | Chapter 07（Iterator）の本文4 Lessonと演習4つ（ex026〜ex029）を実装。全29演習になった | 予定どおり |
| 2026-09-22 | 解答を重ねた段階でex026・ex027のテスト自体にバグが見つかった（期待値の計算間違い、u64オーバーフロー） | 骨組みの todo!() 失敗確認だけでは検出できない種類のミス。解答を重ねてグリーンになることの確認が、テストの正しさそのものの検証としても機能した実例 |
| 2026-09-22 | Chapter 01〜07 を監査し、訂正した（詳細は下記） | Chapter 08 着手前の利用者からの依頼 |
| 2026-09-22 | 本文の事実誤認を訂正: 01-4（in-place collect で外側のバッファは再利用されうる）、02-3（`split_once` は `@` が1つであることを保証しない／`TryFrom` の説明）、04-2（戻り値の `Box<Self>` も object safety 違反。`where Self: Sized` の逃げ道を追記）、04-3（実装が1つなら推論は通る。本当の問題は2つ目の impl で呼び出し側が E0283 で壊れること）、05-2（`where` でしか書けない例が常に成り立つ無意味な bound だった → `String: From<T>` に差し替え）、07-2（u64 のフィボナッチは93個目の取得で溢れる）、07-3（`+ '_` を説明していなかった → E0700 の例を追加）、07-4（「唯一の入力lifetimeを暗黙に借用」は不正確 → 戻り値の型にlifetimeが現れるかが分かれ目） | いずれも実機（Rust 1.75, edition 2021）で確認してから訂正した |
| 2026-09-22 | Chapter 01〜03 の Exercise 節が、実装した演習と食い違っていたのを訂正（「Phase 4 で提供」の削除、型が用意済みであること、`NOTES.md` は任意であること等）。全 Lesson の Exercise 節に実行コマンドを明記 | 本文を先に書き、演習を後から規約に合わせて作ったため、本文側の更新が漏れていた |
| 2026-09-22 | ex011 を再設計: URL を `Option<String>` にして `build` で `unwrap` していたのを、`HasUrl` 状態が URL を持つ形に変更（`PhantomData` も不要になった） | typestate の Lesson の演習が、型で表すべき不変条件を `unwrap` に頼っていたのは教材として矛盾していた |
| 2026-09-22 | ex028 の「遅延評価を検証する」と称したテストを削除 | 外側で `inspect` を足しているだけで、`doubled` が内部で `collect` していても通ってしまい、何も検証していなかった。外部から検証できない性質は、テストのふりをせず自己レビューに回す方針にした |
| 2026-09-22 | ARCHITECTURE.md の「GitHub Actions 上で全て通ることを確認した」という記述を訂正 | 確認できていたのは `checkout@v4` 版の初回実行（警告のみ）だけで、`v5` 更新後の結果は未確認だった |
| 2026-09-22 | CI の exercises-build ジョブに骨組みの `cargo fmt --all --check` を追加 | ex029 の骨組みだけが未整形のまま残っていた。解答側しか fmt を検査していなかったため見逃していた |
| 2026-09-22 | Chapter 08（Lifetimes）の本文4 Lessonと演習4つ（ex030〜ex033）を実装。全33演習になった | 予定どおり |
| 2026-09-22 | 08-2 の中心を「構造体のメソッドの戻り値を `&self` と構造体の `'a` のどちらに結びつけるか」にした | 実機で `Option<&str>` を返すと2回目の呼び出しが E0499 になることを確認し、lifetime の結びつけ方が設計の問題として現れる最も分かりやすい例だと判断した |
| 2026-09-22 | 08-1 と ex030 の `first<'a, 'b>` を、慣用的な `first<'a>(a: &'a str, _b: &str)` に変更（明示版は説明として残した） | clippy の `needless_lifetimes` に指摘された。省略規則1により別の lifetime が割り当てられるので、名前を付ける必要があるのは関係を宣言する lifetime だけ、という点を本文に加えた |
| 2026-09-22 | CI（最新 stable）で ex016 が失敗。`Password(String)` の中身がどこからも読まれず、Rust 1.77 以降の `dead_code` 警告が `clippy -D warnings` でエラーになったと判断し、照合用の `verify` メソッドを追加した | 中身を表示しないことと、中身を使えないことは別。機密値を持つ型は「表示は隠し、必要な操作だけ公開する」べきなので、lint 対応であると同時に設計の修正でもある。本文 04-4 にも同じ補足を追加した |
| 2026-09-22 | 開発サンドボックス（Rust 1.75）より新しい lint は CI でしか検出できないことを ARCHITECTURE.md に明記 | ローカル検証が通っても CI で落ちうる。新しい lint への最終判定は CI に委ねる |

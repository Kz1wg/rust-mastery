# ROADMAP.md — 全Lessonと学習順序

## 1. 章構成の方針

原案（00〜18）をそのまま採用する。**変更したのは「Appendixの追加」と「Lessonごとの依存関係の明示」のみ**。

| 変更 | 理由 |
| --- | --- |
| Appendix A〜C を追加 | Compiler Errorの読み方・AIレビュー用プロンプト・用語集は、章をまたいで参照されるため独立させた |
| 章順は維持 | Error Handling（06）は `std::error::Error` / `From` などtraitの知識を要するため、Trait（04）の後ろが自然。Lifetimes（08）はTrait/Generics/Iteratorで参照付きの型を扱った後の方が「必要な理由」を実感しやすい |
| Testing（13）は残しつつ、`cargo test` の基本は Chapter 00 で扱う | 演習は最初から `cargo test` で判定するため。13章ではテスト**設計**（property的な考え方、テスト容易性、doctest）を扱う |

> 教材を実際に書いて章順に問題が見つかった場合は、DESIGN.md の決定ログに理由を追記して見直す。

## 2. 依存関係（学習順序）

```text
00 Intro
 └─ 01 Ownership ─┬─ 02 Type Design ── 03 Enum & State Machine
                  │                        │
                  │                        ▼
                  │                     04 Traits ── 05 Generics ── 06 Error Handling
                  │                        │              │
                  │                        ▼              ▼
                  │                     07 Iterator ── 08 Lifetimes ── 09 Advanced Type System
                  │                                                        │
                  └────────────── 10 Concurrency ── 11 Async ◄─────────────┘
                                                        │
        12 Module & Architecture ── 13 Testing ── 14 Macros ── 15 Unsafe ── 16 Library Design
                                                        │
                                            17 Practical Projects ── 18 Final Project
```

## 3. Lesson一覧

各Lessonは ARCHITECTURE.md のテンプレートに従い、対応する演習crateを `exercises/` に持つ。
ID は `NN-M`（章-Lesson）。演習crate名は `exNNN_<name>`。

### 00 Introduction
| ID | Lesson | 問い |
| --- | --- | --- |
| 00-1 | この教材の使い方 | 「読む」と「設計する」の違いは何か |
| 00-2 | `cargo test` で学ぶ | 演習はどう判定され、失敗をどう読むか |
| 00-3 | 診断問題 | 前提知識（中級）の確認 |
| 00-4 | 「Rustらしさ」とは | 動く / 短い / Rustらしい は何が違うか |

### 01 Ownership & Borrowing
| ID | Lesson | 問い |
| --- | --- | --- |
| 01-1 | moveは何を守っているのか | なぜコピーではなくmoveなのか |
| 01-2 | 借用のエラーを読む | `cannot move out of x because it is borrowed` の5段階読解 |
| 01-3 | 所有権で設計する | 引数は `T` / `&T` / `&mut T` のどれにすべきか |
| 01-4 | `Vec<String>` / `&[String]` / `impl IntoIterator` | 3つのシグネチャを6観点で比較する |

### 02 Type Design
| ID | Lesson | 問い |
| --- | --- | --- |
| 02-1 | `bool` と文字列が表現してしまう不正な状態 | `admin: bool` の何が問題か |
| 02-2 | newtype pattern | `UserId(String)` と `Email(String)` はいつ有効か／不要か |
| 02-3 | 構築時検証（parse, don't validate） | 不正な値を型の外に出さない設計 |
| 02-4 | フィールドの公開範囲とAPI | 何を保証し、何を保証しないか |

### 03 Enum & State Machine
| ID | Lesson | 問い |
| --- | --- | --- |
| 03-1 | enumで状態を表す | 複数の `Option` フィールドは何を壊すか |
| 03-2 | 状態遷移をenumで設計する | 不正な遷移をどこまで型で防ぐか |
| 03-3 | typestate pattern | enum版とtypestate版のtrade-off |
| 03-4 | `#[non_exhaustive]` と将来の拡張 | バリアント追加は破壊的変更か |

### 04 Traits
| ID | Lesson | 問い |
| --- | --- | --- |
| 04-1 | traitは「共通化」のためではない | 何を抽象化し、何を保証させるか |
| 04-2 | generic vs trait object | 静的/動的ディスパッチのコストと制約 |
| 04-3 | associated type | genericパラメータとの違いは何か |
| 04-4 | 標準trait実装の設計（`From`, `AsRef`, `Display`, `Default`） | どのtraitを実装するのが誠実か |
| 04-5 | 不要なtraitを見抜く | 具象型で十分なケース（レビュー演習） |

### 05 Generics
| ID | Lesson | 問い |
| --- | --- | --- |
| 05-1 | genericにするメリットはあるか | 重複削減か、柔軟性か、それとも複雑化か |
| 05-2 | trait boundsの設計 | boundは最小か、`where`で読みやすいか |
| 05-3 | `impl Trait` | 引数位置と戻り値位置の違い |
| 05-4 | GATの基本用途 | なぜ通常のassociated typeでは足りないのか |

### 06 Error Handling
| ID | Lesson | 問い |
| --- | --- | --- |
| 06-1 | `Result` の型設計 | 失敗をどの粒度で型にするか |
| 06-2 | 独自Error型 | `enum Error` と `Box<dyn Error>` の使い分け |
| 06-3 | error propagation | `?` と `From` は何をしているか |
| 06-4 | libraryとapplicationのerror設計 | 誰がエラーを処理する責任を負うか |

### 07 Iterator
| ID | Lesson | 問い |
| --- | --- | --- |
| 07-1 | adapterの組み合わせ | forループと何が違い、いつforの方が良いか |
| 07-2 | `Iterator` trait | 自作iteratorが必要になるのはいつか |
| 07-3 | 遅延評価とallocation | `collect` はどこで必要か |
| 07-4 | 借用するiteratorを返す | 戻り値の型とlifetimeの関係 |

### 08 Lifetimes
| ID | Lesson | 問い |
| --- | --- | --- |
| 08-1 | lifetime annotationは何を主張しているのか | なぜ推論できないケースがあるのか |
| 08-2 | 構造体が参照を持つ設計 | 参照を持つべきか、所有すべきか |
| 08-3 | elisionと `'static` | 省略規則と `'static` の誤解 |
| 08-4 | 高階の話：HRTB入門 | `for<'a>` が現れる理由 |

### 09 Advanced Type System
| ID | Lesson | 問い |
| --- | --- | --- |
| 09-1 | `PhantomData` | 使っていない型パラメータをなぜ持つのか |
| 09-2 | variance | `&'a T` が共変である意味 |
| 09-3 | 型レベルでの制約表現 | const generics、sealed trait |
| 09-4 | zero-cost abstraction | 何がzeroで、何がzeroではないか |

### 10 Concurrency
| ID | Lesson | 問い |
| --- | --- | --- |
| 10-1 | `Send` / `Sync` とは何を保証するか | なぜ自動traitなのか |
| 10-2 | 共有と可変性：`Rc` / `Arc` / `Mutex` / `RwLock` | 共有可変状態をどう設計するか |
| 10-3 | interior mutability：`Cell` / `RefCell` | 実行時借用チェックは何を引き受けるか |
| 10-4 | メッセージパッシング | 共有しない設計という選択肢 |

### 11 Async Rust
| ID | Lesson | 問い |
| --- | --- | --- |
| 11-1 | `Future` とは何か | async/awaitは何に展開されるのか |
| 11-2 | `Pin` / `Unpin` | なぜ「動かない」ことが必要なのか |
| 11-3 | asyncと所有権 | `Send` なFutureとは何か |
| 11-4 | asyncを使うべきか | スレッドと比べたtrade-off |

### 12 Module & Architecture
| ID | Lesson | 問い |
| --- | --- | --- |
| 12-1 | module分割の基準 | 「ファイルを分ける」と「責務を分ける」の違い |
| 12-2 | lib / bin の分離 | main.rsに何を置き、何を置かないか |
| 12-3 | workspaceの設計 | crate境界はどこに引くか |
| 12-4 | 依存の方向と公開範囲 | `pub` / `pub(crate)` と semver |

### 13 Testing
| ID | Lesson | 問い |
| --- | --- | --- |
| 13-1 | テスト可能な設計 | テストしにくいのは設計のサインか |
| 13-2 | unit / integration / doctest | どこに何を置くか |
| 13-3 | 失敗ケースのテスト | Error型はテストしやすいか |

### 14 Macros
| ID | Lesson | 問い |
| --- | --- | --- |
| 14-1 | マクロが必要な瞬間 | 関数・generic・traitで書けないのは何か |
| 14-2 | declarative macro | `macro_rules!` の限界 |
| 14-3 | procedural macro入門 | derive / attribute / function-like の違い |

### 15 Unsafe Rust
| ID | Lesson | 問い |
| --- | --- | --- |
| 15-1 | unsafeは何を宣言しているのか | 「コンパイラが検証できない不変条件」とは |
| 15-2 | safe abstractionを作る | unsafeを内側に閉じ込める設計 |
| 15-3 | FFI | 境界で何が失われ、何を守るべきか |

### 16 Library Design
| ID | Lesson | 問い |
| --- | --- | --- |
| 16-1 | 公開APIのレビュー | このAPIは利用者に何を保証するか |
| 16-2 | semverと拡張性 | 何が破壊的変更か |
| 16-3 | ドキュメントと例 | doctestは仕様書になるか |

### 17 Practical Projects
| ID | Project | 主な題材 |
| --- | --- | --- |
| P1 | Rust CLI | 引数設計、error設計、lib/bin分離 |
| P2 | CSV parser | Iterator、Error、lifetime |
| P3 | ログ解析ツール | Iterator、抽象化のやりすぎ問題 |
| P4 | HTTP client | trait設計、テスト容易性 |
| P5 | キャッシュシステム | interior mutability、Send/Sync |
| P6 | 非同期データ取得 | async、所有権 |
| P7 | Rust library | Library Design の総合 |

### 18 Final Project
中規模アプリをゼロから設計する。**コードよりも設計プロセス**を評価する。

```text
Requirements → Domain Model → Type Design → Trait Design
            → Error Design → Module Design → Implementation → Testing → Refactoring
```

各段階で設計ドキュメント（1ページ）を書き、AIにレビューさせ、指摘への対応を記録する。

### Appendix
| ID | 内容 |
| --- | --- |
| A | Compiler Error 読解集（エラーコード別：何が保証されているか） |
| B | AIレビュー用プロンプト集（設計レビュー／代替案の提示／過剰抽象化の検出） |
| C | 用語集（Send/Sync/variance などを「なぜ必要か」つきで） |

## 4. 実装Phase（開発順序）

| Phase | 内容 | 完了条件 |
| --- | --- | --- |
| 1 | 設計（DESIGN / ROADMAP / ARCHITECTURE、ディレクトリ、演習システム設計） | 3文書が揃い、レビューできる |
| 2 | mdBook最小構成 | `mdbook serve` / `mdbook build` / `mdbook test` が通る |
| 3 | 第1〜3章（Ownership / Type Design / Enum & State Machine） | 3章の全Lessonがテンプレート通りに揃う |
| 4 | 演習システム | `cargo test` と `check-exercises` が動き、解答検証がCIで通る |
| 5 | Trait / Generics / Error Handling | 04〜06章 |
| 6 | Iterator〜Advanced Rust | 07〜16章（12 Module & Architecture を含む） |
| 7 | 実践プロジェクト | P1〜P7 |
| 8 | Final Project | 18章 |

# 演習インデックス

全61演習（ex001〜ex061）の一覧。仕様の詳細は各 Lesson の Exercise 節を参照。
全て実装済み（骨組み・テスト・`exercise.toml`・`solutions/`）。

判定は `cargo test -p <ID>`（入れ子 workspace の ex012・ex048・ex055 のみ、ディレクトリ内で `cargo test`）。
全体の進捗は `cargo run -p check-exercises`。

| ID | Lesson | 題材 | 判定方法 |
| --- | --- | --- | --- |
| ex001_move_semantics | 01-1 | `longest_owned` / `longest_ref` | 通常テスト |
| ex002_borrow_errors | 01-2 | `first_then_push`、`bump_two`（`split_at_mut`） | 通常テスト |
| ex003_ownership_design | 01-3 | `count_words(&str)`、`Config::new(impl Into<String>)` | 通常テスト（複数の呼び出し形式でコンパイルできること） |
| ex004_compare_signatures | 01-4 | 3つのシグネチャで `dedup_sorted` | 通常テスト |
| ex005_invalid_states | 02-1 | 信号機（`Light::next`）・注文（`OrderStatus`） | 通常テスト |
| ex006_newtype | 02-2 | `Meters` / `Feet` / `From<Feet> for Meters` | 通常テスト＋ `compile_fail` doctest |
| ex007_parse_dont_validate | 02-3 | `Percentage` / `NonEmptyString` | 通常テスト＋ `compile_fail` doctest |
| ex008_visibility | 02-4 | `Temperature`、`Inventory`（予約／解除） | 通常テスト＋ `compile_fail` doctest ×2 |
| ex009_enum_states | 03-1 | `Connection`（Disconnected/Connected/Failed） | 通常テスト |
| ex010_state_machine | 03-2 | ドアの状態機械（Open/Closed/Locked） | 全（状態×操作）の網羅テスト |
| ex011_typestate | 03-3 | `RequestBuilder<NoUrl/HasUrl>`（URLを状態が保持） | 通常テスト＋ `compile_fail` doctest |
| ex012_non_exhaustive | 03-4 | `ex012_lib`/`ex012_app` の2crate（独立workspace） | 通常テスト |
| ex013_trait_purpose | 04-1 | `PaymentMethod`（CreditCard/BankTransfer） | 通常テスト |
| ex014_generic_vs_dyn | 04-2 | `Shape`（generic版 / dyn版の`total_area`） | 通常テスト＋ `compile_fail` doctest |
| ex015_associated_type | 04-3 | `Stack`（associated type、`IntStack`） | 通常テスト |
| ex016_standard_traits | 04-4 | `Percentage`（`TryFrom`）、`Password`（秘匿`Debug`） | 通常テスト |
| ex017_review_logger | 04-5 | `Logger` traitの除去（レビュー演習の簡易版） | 通常テスト |
| ex018_generic_benefit | 05-1 | `min_max`（重複除去のgeneric化） | 通常テスト |
| ex019_trait_bounds | 05-2 | `describe`/`values_equal`（不要boundの削除） | 通常テスト |
| ex020_impl_trait | 05-3 | `shout`（引数位置）/ `evens_up_to`（戻り値位置） | 通常テスト |
| ex021_gat_basics | 05-4 | `Container`（GAT、borrowするイテレータ） | 通常テスト |
| ex022_result_design | 06-1 | `parse_config`（`ConfigError`の粒度設計） | 通常テスト |
| ex023_custom_error_types | 06-2 | `ReadNumberError`（Display/std::error::Error） | 通常テスト |
| ex024_error_propagation | 06-3 | `parse_two_numbers`（`?`と`From`） | 通常テスト |
| ex025_library_vs_application | 06-4 | `load_config`/`run_app`（lib/app境界のエラー設計） | 通常テスト |
| ex026_adapters_vs_for | 07-1 | `even_squares`（adapter）/ `first_index_over`（forループ） | 通常テスト |
| ex027_custom_iterator | 07-2 | `Fibonacci`（`Iterator`の自作） | 通常テスト |
| ex028_laziness_and_allocation | 07-3 | `doubled`（中間Vecを作らない） | 通常テスト（遅延評価そのものは自己レビュー） |
| ex029_borrowing_iterators | 07-4 | `merged_over`（2引数からの借用、`chain`+`filter`） | 通常テスト |
| ex030_lifetime_annotations | 08-1 | `longest` / `first`（戻り値がどの引数を借りるか） | 通常テスト（`b` 破棄後も結果を使えること） |
| ex031_struct_with_reference | 08-2 | `Parser<'a>::next_word`（戻り値を `'a` に結びつける） | 通常テスト（単語を持ったまま次を取り出せること） |
| ex032_static_bound | 08-3 | `describe_later<T: 'static>` | 通常テスト＋ `compile_fail` doctest |
| ex033_hrtb | 08-4 | `apply_to_local`（`for<'a>`）/ `count_matching`（省略形） | 通常テスト |
| ex034_phantom_data | 09-1 | `Id<T>`（型付きID、手書きの `Clone`/`Copy`） | 通常テスト＋ `compile_fail` doctest |
| ex035_variance | 09-2 | `pick_longer`（共変）/ `push_word`（不変）/ `collect_short_words` | 通常テスト＋ `compile_fail` doctest |
| ex036_type_level_constraints | 09-3 | `Vector<const N>`、sealed trait `Unit` | 通常テスト＋ `compile_fail` doctest ×2（doctest は別crateとして実行されるので封印を確認できる） |
| ex037_zero_cost | 09-4 | iterator版とloop版、`#[repr(transparent)]` の `UserId` | 通常テスト（`size_of` でレイアウトも確認） |
| ex038_send_sync | 10-1 | `spawn_sum`、`assert_send`/`assert_sync` | 通常テスト＋ `compile_fail` doctest（`Rc` は渡せない） |
| ex039_arc_mutex | 10-2 | `parallel_increment`（Mutex）、`total_length_times_three`（RwLock） | 通常テスト |
| ex040_interior_mutability | 10-3 | `Counter`（Cell）、`Logger`（RefCell、`try_borrow_mut`） | 通常テスト |
| ex041_message_passing | 10-4 | `sum_with_join`、`sum_with_channel`、閉じたチャネルへの送信 | 通常テスト |
| ex042_future_basics | 11-1 | `Countdown`（`Future` 実装）、`block_on`、poll回数 | 通常テスト |
| ex043_pin_and_boxing | 11-2 | `run_boxed`、`make_boxed_future` | 通常テスト＋ `compile_fail` doctest |
| ex044_async_ownership | 11-3 | `sum_shared`（`Arc`）、`assert_send_future` | 通常テスト＋ `compile_fail` doctest |
| ex045_join2 | 11-4 | `Join2::poll`（スレッドなしの並行実行） | 通常テスト（ログ順で交互実行を確認） |
| ex046_module_boundaries | 12-1 | `order` モジュール（公開項目を最小に） | 通常テスト＋ `compile_fail` doctest ×2 |
| ex047_lib_and_bin | 12-2 | `count_words` / `run` と薄い `main.rs` | 通常テスト（lib の公開APIのみ） |
| ex048_workspace | 12-3 | `ex048_core` / `ex048_storage`（依存は一方向） | 入れ子 workspace。ディレクトリ内で `cargo test` |
| ex049_visibility_and_facade | 12-4 | `pub use` による facade、`pub(crate)` ヘルパー | 通常テスト＋ `compile_fail` doctest ×2 |
| ex050_testable_design | 13-1 | `greeting_for_hour`、`is_business_hours`、`pick_with`（選び方を注入） | 通常テスト |
| ex051_kinds_of_tests | 13-2 | `word_frequency` と private な `normalize` | 単体テスト＋統合テスト＋ドキュメントテスト（3種類すべて） |
| ex052_testing_failures | 13-3 | `parse_age`（Errの種類）、`get_item`（panicのメッセージ） | 通常テスト（`should_panic(expected = ...)` を含む） |
| ex053_when_to_use_macros | 14-1 | `square`（関数で書く）、`max_of!`（可変長） | 通常テスト＋ドキュメントテスト（引数が1回だけ評価されること） |
| ex054_macro_rules | 14-2 | `hashmap!`（繰り返し）、`impl_unit!`（項目の生成） | 通常テスト＋ドキュメントテスト |
| ex055_derive_macro | 14-3 | `#[derive(Describe)]`（依存ゼロ、標準の `proc_macro` のみ） | 入れ子 workspace。ディレクトリ内で `cargo test` |
| ex056_unsafe_basics | 15-1 | `first_via_ptr`、`read_value`（`unsafe fn`）、`swap_values`（unsafe 不要の引っかけ） | 通常テスト＋ `compile_fail` doctest |
| ex057_safe_abstraction | 15-2 | `my_split_at_mut`、`first_and_last_mut` | 通常テスト（範囲外は panic で止まること） |
| ex058_ffi | 15-3 | `c_abs`（`i32::MIN` を弾く）、`c_strlen`、`rust_add`、`from_c_str` | 通常テスト（C の標準ライブラリを依存なしで呼ぶ） |
| ex059_api_review | 16-1 | `analyze`（`Unit` enum、`Options` ビルダー、`Count` 構造体） | 通常テスト |
| ex060_semver_friendly | 16-2 | `Config`・`Level`（`#[non_exhaustive]`）、`Store`（デフォルト実装） | 通常テスト＋ `compile_fail` doctest ×2 |
| ex061_documentation | 16-3 | `parse_duration`（`#![deny(missing_docs)]`） | **ドキュメントのコード例が主なテスト**＋補足の統合テスト |

## 演習の設計方針

- **解答は複数ファイルでもよい。** `solutions/<ID>/` 配下の `.rs` は、対応する `exercises/<ID>/` の同じ相対パスへ
  まとめて重ねられる（ex047 の `main.rs`、ex049 の `config.rs`/`parser.rs`、入れ子 workspace の各crate）。
- **依存crateはゼロ**（標準ライブラリのみ）。Chapter 11 の async 演習も、各crateに最小ランタイム（`src/runtime.rs`、約40行）を同梱して動かす。

- 型定義と公開APIのシグネチャは骨組みに固定し、本体だけを `todo!()` にする（`cargo test` で自動判定するため）。
  「型を自分で書き換える」体験は、各 Lesson の Think / Solution / Challenge で担保する。
- `NOTES.md` への記述は自己チェック用で、`cargo test` では判定しない。
- 振る舞いはテストで判定できても、**設計の性質（遅延評価か、中間Vecを作っていないか等）は外部からテストできないことがある**。
  その場合はテストで判定しているふりをせず、演習のdocコメントで自己レビューを求める（ex028）。

## 演習を作るときのチェックリスト（過去に実際に踏んだ罠）

- **`todo!()` のメッセージに、テストが期待する文字列を書かない。** `todo!()` も panic なので、
  メッセージに `#[should_panic(expected = "...")]` の文字列が含まれていると、**未実装のままテストが通る**。
  ex052 で「index out of range」を `todo!()` の説明文に書いてしまい、骨組みの確認中に気づいた。
- **`todo!()` のメッセージに波かっこを書かない。** `{}` や `{var}` は `format!` の引数として解釈され、
  コンパイルエラーになる（ex007・ex009・ex010・ex011・ex013・ex024 で発生）。
  具体例を示したいときは「例: "..." のような形式」と、波かっこを含めない言い方にする。
- **マクロの骨組みでは、`todo!()` の型が決まらないことがある。** マクロの展開先で `!` 型のまま比較されると
  コンパイルできない（ex053）。`if false { a } else { todo!(...) }` のように、既にある値と型を揃える形にする。
  値を生成するマクロ（ex054 の `hashmap!`）は、テスト側で戻り値の型を明示しておくと、未実装でも型が決まる。
  procedural macro（ex055）は、`todo!()` を**生成するコードの中**に置けば、骨組みでもコンパイルが通る。
- **`todo!()` だけでは戻り値の型が決まらない場面が、`impl Trait` 以外にもある。** スレッドのクロージャ
  （`thread::spawn(move || todo!())` は `()` を返すと推論され、`sum()` が失敗）や、送信するまで型が決まらない
  `mpsc::Sender`（E0283）など。型注釈付きのローカル変数（`let total: usize = todo!(...); total`）を骨組みに残すか、
  型が決まる呼び出し（`tx.send(partial)`）を骨組み側に置く。
- **`-> impl Trait` を返す関数の本体を、まるごと `todo!()` にしない。** opaque type の推論は本体の
  具体的な式から行われるため、`error[E0277]: () is not an iterator` になる（ex020）。外側の式は骨組みに残し、
  `todo!()` はクロージャの中など内側の式に埋め込む。最終式が `.chain(...)` のようにクロージャを持たない場合は、
  題材に軽い条件（`.filter(...)` など）を足して埋め込む場所を作る（ex029）。
- **骨組みを作ったら、テストファイルも含めて `cargo fmt --all` をかけてから検証する。** 解答側だけでなく
  `tests/tests.rs` も未整形のまま残りやすい（ex033・ex034 の長い `vec![...]` の行）。CI の exercises-build が検出する。
- **解答に `cargo fmt` をかけたら、その結果を `solutions/` へ書き戻す。** 忘れると
  `verify_solutions.sh --fmt` が失敗する（ex013）。`check-exercises --solutions` は fmt を見ないので検知できない。
- **テストの期待値は自分で計算し直す。** 骨組みの `todo!()` 失敗確認では、テスト自体の誤りは検出できない。
  解答を重ねてグリーンになることで初めて検証される（ex026 の期待値の計算ミス、ex027 の u64 オーバーフロー）。
- **その演習が検証すべき概念を、テストが本当に通っているか確認する。** `?` の演習なのに `.map_err()` で
  変換していて `From` を経由していなかった（ex024）、遅延評価を検証すると称したテストが実は何も検証していなかった（ex028）。
- **解答だけでなく、骨組みのシグネチャも clippy の目で見る。** 骨組みは `todo!()` のため clippy に掛けられないが、
  シグネチャは解答と共有される。教材として丁寧に書いた `fn first<'a, 'b>(..., _b: &'b str)` が
  `needless_lifetimes` に引っかかった（ex030）。
- **private なフィールドは、どこかで必ず読まれるようにする。** `Password(String)` の中身を手書き `Debug` で
  意図的に隠した結果、どこからも読まれず、Rust 1.77 以降の `dead_code` 警告（`field 0 is never read`）で
  CI の `clippy -D warnings` が失敗した（ex016）。開発サンドボックス（Rust 1.75）では再現しなかった。
  読まれないフィールドは、たいてい「その値を使うAPIが設計されていない」サインでもある。
- **typestate の題材で `unwrap` が必要になったら、状態の持たせ方を疑う。** 状態を型にしたのに
  `Option` を `unwrap` していた（ex011）。値はその状態の型に持たせる。

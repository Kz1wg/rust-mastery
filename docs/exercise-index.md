# 演習インデックス

全33演習（ex001〜ex033）の一覧。仕様の詳細は各 Lesson の Exercise 節を参照。
全て実装済み（骨組み・テスト・`exercise.toml`・`solutions/`）。

判定は `cargo test -p <ID>`（ex012 のみ `cd exercises/ex012_non_exhaustive && cargo test`）。
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

## 演習の設計方針

- 型定義と公開APIのシグネチャは骨組みに固定し、本体だけを `todo!()` にする（`cargo test` で自動判定するため）。
  「型を自分で書き換える」体験は、各 Lesson の Think / Solution / Challenge で担保する。
- `NOTES.md` への記述は自己チェック用で、`cargo test` では判定しない。
- 振る舞いはテストで判定できても、**設計の性質（遅延評価か、中間Vecを作っていないか等）は外部からテストできないことがある**。
  その場合はテストで判定しているふりをせず、演習のdocコメントで自己レビューを求める（ex028）。

## 演習を作るときのチェックリスト（過去に実際に踏んだ罠）

- **`todo!()` のメッセージに波かっこを書かない。** `{}` や `{var}` は `format!` の引数として解釈され、
  コンパイルエラーになる（ex007・ex009・ex010・ex011・ex013・ex024 で発生）。
  具体例を示したいときは「例: "..." のような形式」と、波かっこを含めない言い方にする。
- **`-> impl Trait` を返す関数の本体を、まるごと `todo!()` にしない。** opaque type の推論は本体の
  具体的な式から行われるため、`error[E0277]: () is not an iterator` になる（ex020）。外側の式は骨組みに残し、
  `todo!()` はクロージャの中など内側の式に埋め込む。最終式が `.chain(...)` のようにクロージャを持たない場合は、
  題材に軽い条件（`.filter(...)` など）を足して埋め込む場所を作る（ex029）。
- **解答に `cargo fmt` をかけたら、その結果を `solutions/` へ書き戻す。** 忘れると
  `verify_solutions.sh --fmt` が失敗する（ex013）。`check-exercises --solutions` は fmt を見ないので検知できない。
- **テストの期待値は自分で計算し直す。** 骨組みの `todo!()` 失敗確認では、テスト自体の誤りは検出できない。
  解答を重ねてグリーンになることで初めて検証される（ex026 の期待値の計算ミス、ex027 の u64 オーバーフロー）。
- **その演習が検証すべき概念を、テストが本当に通っているか確認する。** `?` の演習なのに `.map_err()` で
  変換していて `From` を経由していなかった（ex024）、遅延評価を検証すると称したテストが実は何も検証していなかった（ex028）。
- **解答だけでなく、骨組みのシグネチャも clippy の目で見る。** 骨組みは `todo!()` のため clippy に掛けられないが、
  シグネチャは解答と共有される。教材として丁寧に書いた `fn first<'a, 'b>(..., _b: &'b str)` が
  `needless_lifetimes` に引っかかった（ex030）。
- **typestate の題材で `unwrap` が必要になったら、状態の持たせ方を疑う。** 状態を型にしたのに
  `Option` を `unwrap` していた（ex011）。値はその状態の型に持たせる。

# P6: 非同期データ取得（tokio）

## 何を作るか

たくさんの URL（ここでは path）から、データを**同時に**取ってくるライブラリです。
この教材で初めて、依存crate（**tokio**）を使います。

```rust,ignore
let fetcher = Arc::new(HttpFetcher::new("127.0.0.1:8080".parse()?));
let paths = vec!["/users/1".to_string(), "/users/2".to_string(), "/users/3".to_string()];

let options = Options {
    concurrency: NonZeroUsize::new(2).unwrap(), // 同時に走らせるのは2件まで
    timeout: Duration::from_secs(3),            // 1件あたり3秒まで
};
let results = fetch_all(fetcher, paths, &options).await;
// results[0] は /users/1 の結果、results[1] は /users/2 の結果 …（渡した順）
```

| 決まりごと | 内容 |
| --- | --- |
| 同時実行数 | `options.concurrency` 件まで。それ以上は、前の取得が終わるのを待つ |
| 制限時間 | 1件ごとに `options.timeout`。過ぎたら `Err(FetchError::Timeout)` |
| 結果の順番 | 渡した `paths` と**同じ順番**（終わった順ではない） |
| 失敗 | 1件が失敗しても（panic しても）、他の取得は続ける |

通信の相手は、テストの中で立てる**ローカルのサーバー**（`127.0.0.1`）です。外部のネットワークには出ません。

必要な Rust のバージョンは **1.75 以上**です（trait のメソッドの戻り値に `impl Future` を書くため。後述）。

## 使う章

| 章 | このプロジェクトでの使いどころ |
| --- | --- |
| 11-3 async と所有権 | `tokio::spawn` に渡す Future は `Send + 'static`。だから `fetcher` を `Arc` で受け取る |
| 11-4 async を使うべきか | tokio のランタイム、features の選び方。そもそも async にする価値があるか |
| 10-2 共有と可変性 | `Arc` による共有、`Semaphore`（同時に入れる数を数える仕組み） |
| 06 エラー処理 | `FetchError` の設計。「1件の失敗」と「全体の失敗」を分ける |
| 13-1 テスト可能な設計 | `Fetch` trait で通信を差し替える（P4 と同じ考え方）。時計を止めてテストする |
| 02-1 不正な状態 | 同時実行数 0 を `NonZeroUsize` で禁止する（0 という値を、そもそも作れなくする） |

## 全体の形

```text
fetch_all(fetcher, paths, options)
  │
  ├─ path ごとに: Semaphore の空きを1つ取る → tokio::spawn
  │                                            └─ fetch_one: timeout で包んで fetcher.fetch(path)
  │
  └─ JoinHandle を起こした順に await して、Vec に並べる
```

| 部品 | 役割 | async か |
| --- | --- | --- |
| `parse_response` | HTTP の応答から本文を取り出す | **いいえ**（ただのバイト列の処理） |
| `Fetch` trait | 「path を渡すと本文が返る」取得処理 | はい |
| `HttpFetcher` | 本物。tokio の `TcpStream` で HTTP/1.0 の GET を送る | はい |
| `fetch_one` | 1件を制限時間つきで取得 | はい |
| `fetch_all` | たくさんを、同時実行数を守って取得 | はい |

`parse_response` を async にしていないことに注目してください。
**待ち時間のない処理は、ふつうの関数のままにしておく**と、ふつうの `#[test]` で確かめられ、使い回しもしやすくなります。

## tokio の準備

`projects/p06_fetch_all/Cargo.toml` では、使う機能だけを有効にしています（Lesson 11-4）。

```toml
[dependencies]
tokio = { version = "1", features = ["rt", "net", "io-util", "time", "sync"] }

[dev-dependencies]
tokio = { version = "1", features = ["rt", "net", "io-util", "time", "sync", "macros", "test-util"] }
```

| feature | 何のため |
| --- | --- |
| `rt` | ランタイム本体と `tokio::spawn` |
| `net` | `TcpStream`（と、テストのサーバーの `TcpListener`） |
| `io-util` | `write_all` や `read_to_end`（`AsyncWriteExt` / `AsyncReadExt`） |
| `time` | `timeout` と `sleep` |
| `sync` | `Semaphore` |
| `macros`（テストだけ） | `#[tokio::test]` |
| `test-util`（テストだけ） | 時計を止めて進める（後述） |

`#[tokio::main]` に必要な `rt-multi-thread` は入れていません。
これは**ライブラリ**なので、ランタイムを起動するのは利用者の側だからです。

## 設計の問い

> **問い1**: `fetch_all` は `fetcher` を `&F` ではなく `Arc<F>` で受け取ります。なぜでしょうか。

<details>
<summary>考え方の例</summary>

`tokio::spawn` に渡す Future は、`'static`（借りた参照を持たない）でなければなりません（Lesson 11-3）。
起こしたタスクは、`fetch_all` を呼んだ関数より長く生きるかもしれないからです。
`&F` をタスクに持ち込むと、「借り元がもう無いのに、タスクがまだ使っている」ことが起こりえます。

そこで、各タスクが `Arc` の複製を**所有**します。最後のタスクが終わった時点で、`fetcher` が片付けられます。

| 受け取り方 | spawn できるか | 呼ぶ側の手間 |
| --- | --- | --- |
| `&F` | できない（`'static` でない） | 少ない |
| `F`（所有権ごと） | 1つのタスクにしか渡せない | 少ない |
| `Arc<F>` | できる。全タスクで共有 | `Arc::new` で包む |

別の道として、spawn せずに**1つのタスクの中で**複数の Future を同時に進める方法もあります
（`futures` crate の `join_all` や `buffer_unordered`）。この場合は `&F` のままで済みますが、
全部が1つのタスクの中で進むので、マルチスレッドのランタイムでも1つのスレッドしか使いません。
取得（I/O 待ち）が中心なら、それでも十分なことが多いです。
この教材では、依存を tokio だけにするため、`spawn` を使う形にしています。

</details>

> **問い2**: `fetch_one` で制限時間を過ぎたとき、途中まで進んでいた `fetcher.fetch(path)` はどうなるでしょうか。

<details>
<summary>考え方の例</summary>

`tokio::time::timeout` は、時間切れになると、包んでいた Future を**捨てます**（drop します）。
Future は `poll` されない限り進まないので（Lesson 11-1）、捨てられた時点で処理はそこで終わります。
async では、**drop がそのまま「中断（キャンセル）」になる**のです。

`HttpFetcher` の場合、途中まで使っていた `TcpStream` も一緒に drop され、接続が閉じます。
片付けを自分で書かなくてよいのは、Rust の所有権の良いところです。

ただし、落とし穴もあります。`.await` の**後ろ**に書いた片付けは、中断されると実行されません。

```rust,ignore
self.in_flight.fetch_add(1, SeqCst);
sleep(d).await;                      // ← ここで中断されると…
self.in_flight.fetch_sub(1, SeqCst); // ← この行は実行されない
```

テストの偽物（`FakeFetcher`）では、減らす処理を `Drop` を実装した小さな型（`InFlight`）に任せています。
Future が途中で捨てられても `Drop` は必ず動くので、数がずれません。
ファイルを閉じる・ロックを外すなどの後片付けを `Drop` に任せるのは、Rust でよく使われる書き方です（`MutexGuard` もこの仕組みでロックを外しています）。

</details>

> **問い3**: 同時実行数の空き（Semaphore の permit）は、`tokio::spawn` の**前**に取っています。タスクの**中**で取るのと、何が違うでしょうか。

<details>
<summary>考え方の例</summary>

| 取る場所 | 同時に存在するタスク | 特徴 |
| --- | --- | --- |
| spawn の前（この実装） | 最大 `concurrency` 個 | 1万件渡されても、タスクは少ししか作らない。そのかわり `fetch_all` 自体が途中で待つ |
| タスクの中 | 渡された件数ぶん（1万個） | すぐに全部 spawn できるが、ほとんどのタスクは空き待ちで眠っているだけ |

どちらでも「同時に通信するのは `concurrency` 件まで」は守れます。
違いは、**待っている仕事をどこに溜めるか**です。
spawn の前で待つと、まだ始めていない仕事は `paths` の中に残り、メモリを使いません。
「受け取る側が追いつくまで、送る側を待たせる」このやり方は、**背圧（backpressure）**と呼ばれます。

</details>

> **問い4**: `Fetch` trait のメソッドは、`async fn fetch(...)` ではなく、
> `fn fetch(...) -> impl Future<Output = ...> + Send` と書いています。なぜでしょうか。

<details>
<summary>考え方の例</summary>

trait に `async fn` と書くと、返ってくる Future が `Send` かどうかを、trait の**利用者が指定できません**。
`fetch_all` はその Future を `tokio::spawn`（別のスレッドで動くかもしれない）に渡すので、`Send` であることが必要です。

戻り値の型で `+ Send` と書いておけば、「この trait の実装は、必ず `Send` な Future を返す」という**約束**になります。
約束を破る実装（たとえば中で `Rc` を `.await` をまたいで持つ実装）は、実装した時点でコンパイルエラーになります。

実装する側は、今までどおり `async fn` で書けます。

```rust,ignore
impl Fetch for HttpFetcher {
    async fn fetch(&self, path: &str) -> Result<String, FetchError> {
        // …
    }
}
```

なお、trait の中の `impl Trait`（と `async fn`）は Rust 1.75 から使えるようになった機能です。
また、この形の trait は `dyn Fetch` にはできません（Lesson 04-2）。
`Box<dyn Fetch>` のように実行時に差し替えたい場合は、`async-trait` crate を使うか、
`Pin<Box<dyn Future + Send + '_>>` を返す形にします（Lesson 11-2 の `Pin<Box<...>>` と同じ形です）。

</details>

> **問い5**: `fetch_all` は `Vec<Result<String, FetchError>>` を返します。`Result<Vec<String>, FetchError>` ではない理由は何でしょうか。

<details>
<summary>考え方の例</summary>

| 戻り値 | 意味 | 向いている場面 |
| --- | --- | --- |
| `Result<Vec<String>, FetchError>` | 1件でも失敗したら、全体を失敗にする | 全部そろわないと意味がない（設定ファイルを複数読むなど） |
| `Vec<Result<String, FetchError>>` | 1件ずつの成功・失敗を返す | 取れたものだけでも使いたい（一覧画面の表示など） |

「100件のうち1件がタイムアウトしただけで、残り99件の結果まで捨てる」のはもったいない、という判断です。
P3（ログ解析）の「壊れた行があっても止めない」と同じ考え方ですね。
全体を失敗にしたい利用者は、`results.into_iter().collect::<Result<Vec<_>, _>>()` と1行で変換できます（`Result` の iterator は、`collect` で「全部成功なら `Ok(Vec)`、1つでも失敗なら最初の `Err`」にまとめられます）。
逆方向（全体の失敗から、1件ずつの結果を取り戻す）はできないので、**情報の多いほうを返す**のが安全です。

</details>

> **問い6**: そもそも、この仕事に async は必要でしょうか。

<details>
<summary>考え方の例</summary>

Lesson 11-4 の判断の軸で考えてみましょう。

- 仕事の中身は、ほとんどが**通信の待ち時間**です（async に向いている）
- ただし、同時実行数が4件程度なら、**スレッドを4本**立てても十分に実現できます（P4 のような同期のコードで書ける）

正直に言えば、**数件〜数十件ならスレッドで足ります**。
async が本当に効いてくるのは、同時に数百〜数千の接続を扱うときです。

それでも実務で async を選ぶことが多いのは、
使いたいライブラリ（HTTP クライアント、DB ドライバなど）がすでに async で書かれていることが多いからです。
「async にする理由を説明できる」ことが大切で、「なんとなく速そうだから」で選ばないようにしましょう。

</details>

## テストの工夫: 時計を止める

制限時間や同時実行数をテストするのに、本当に何秒も待つのは困ります。
tokio の `test-util` 機能を使うと、**ランタイムの時計を止めた状態で**テストを始められます。

```rust,ignore
#[tokio::test(start_paused = true)]
async fn fetch_one_times_out() {
    let fake = FakeFetcher::with_delays(&[("slow", 10_000)]); // 10秒かかる偽物
    let start = Instant::now();
    let r = fetch_one(&fake, "slow", Duration::from_secs(1)).await;
    assert_eq!(r, Err(FetchError::Timeout));
    assert_eq!(start.elapsed(), Duration::from_secs(1)); // ぴったり1秒
}
```

時計が止まっていると、全タスクが `sleep` などで待ち状態になったときに、tokio が
**次に起きるべき時刻まで時計を一気に進めます**。
テストは一瞬で終わり、しかも経過時間が毎回ぴったり同じになります（P5 の `Clock` を、ランタイムが用意してくれている形です）。

これで「10件を同時に3件まで走らせると、100ms × 4回 = 400ms かかる」ことも、正確に確かめられます。

一方、本物の `HttpFetcher` のテストは、時計を止めずに、テストの中で `127.0.0.1` にサーバーを立てて確かめています。
ポート番号に `0` を指定すると、OS が空いているポートを選んでくれます。

## 進め方

1. **`parse_response`**（async ではない）: `std::str::from_utf8` → `split_once("\r\n\r\n")` → 1行目を空白で分けて状態コードを取り出す
2. **`HttpFetcher::fetch`**: 次の順に `.await` する。エラーは `?` で `FetchError::Io` に変わる（`From<io::Error>` を用意してある）
   - `TcpStream::connect(self.addr)`
   - `write_all` でリクエストを送る。形は `GET <path> HTTP/1.0` の行、`Host:` の行、空行（各行の終わりは `\r\n`）
   - `read_to_end` で、サーバーが接続を閉じるまで読む
3. **`fetch_one`**: `tokio::time::timeout(timeout, fetcher.fetch(path))` の結果で分ける
4. **`fetch_all`**: 全体の形の図のとおり。
   - `Arc::new(Semaphore::new(n))` を作り、`Arc::clone(&semaphore).acquire_owned().await` で空きを取る
   - 取った permit をタスクの中へ move し、`fetch_one` が終わったら drop する
   - `JoinHandle` を `Vec` に溜め、順に `.await` する。`Err`（panic）なら `FetchError::Panicked`

`acquire_owned` は「Semaphore が閉じられた」ときだけ `Err` を返します。この実装では閉じないので、`expect` で構いません。
`expect` のメッセージには、「なぜ失敗しないと言えるのか」を書いておきましょう。

## 判定

```bash
cd projects && cargo test -p p06_fetch_all
```

コードは [projects/p06_fetch_all](https://github.com/Kz1wg/rust-mastery/tree/main/projects/p06_fetch_all) にあります。書き換えるのは `src/` の中で、判定に使うテストは `tests/tests.rs` です。

## Challenge

- `fetch_all` に「全体の制限時間」を足すとしたら、どこに `timeout` を置けばよいでしょうか。
  時間切れのとき、まだ終わっていないタスクはどうなるべきでしょうか（`JoinHandle` を drop してもタスクは止まりません。`abort` が必要です）
- 失敗したら少し待ってやり直す（P4 の再試行）を、`fetch_one` に足してみましょう。待つのは `std::thread::sleep` ではなく `tokio::time::sleep` です。なぜでしょうか

## 振り返り

- [ ] `tokio::spawn` に渡すものが `Send + 'static` でなければならない理由と、`Arc` で解決する方法を説明できる
- [ ] async では drop が中断になること、片付けを `Drop` に任せる理由を説明できる
- [ ] Semaphore で同時実行数を制限でき、spawn の前後どちらで待つかの違いを説明できる
- [ ] 「1件の失敗」と「全体の失敗」を、戻り値の型で表し分けられる
- [ ] この仕事に async が必要かどうかを、自分の言葉で判断できる

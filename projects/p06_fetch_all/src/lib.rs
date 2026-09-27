//! P6: 複数のデータを、同時に取得する（tokio を使う）。
//!
//! 組み立て:
//! - `Fetch` trait: 「path を渡すと、本文が返ってくる」非同期の取得処理
//! - `HttpFetcher`: 本物。tokio の TcpStream で HTTP/1.0 の GET を送る（http のみ）
//! - `fetch_all`: たくさんの path を、同時に取得する。同時に走らせる数と、1件あたりの制限時間を守る
//!
//! テストでは、Fetch を偽物に差し替え、tokio の時計を止めて「時間」を自由に進める。

use std::fmt;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Semaphore;

/// 取得の失敗。
///
/// `Clone + PartialEq` にしたいので、io::Error そのものではなく `io::ErrorKind` だけを持つ
/// （io::Error は Clone でも PartialEq でもない）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchError {
    /// 制限時間内に終わらなかった。
    Timeout,
    /// 接続できない、途中で切れた、など。
    Io(io::ErrorKind),
    /// サーバーが 200 以外を返した。
    Status(u16),
    /// 応答の形が HTTP として読めない。
    BadResponse,
    /// 取得処理が panic した（そのタスクだけが失敗し、他は続く）。
    Panicked,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::Timeout => write!(f, "制限時間内に終わりませんでした"),
            FetchError::Io(kind) => write!(f, "通信に失敗しました: {kind}"),
            FetchError::Status(code) => write!(f, "サーバーがエラーを返しました: {code}"),
            FetchError::BadResponse => write!(f, "応答を読めませんでした"),
            FetchError::Panicked => write!(f, "取得処理が panic しました"),
        }
    }
}

impl std::error::Error for FetchError {}

impl From<io::Error> for FetchError {
    fn from(e: io::Error) -> Self {
        FetchError::Io(e.kind())
    }
}

/// path を受け取り、本文を返す非同期の取得処理。
///
/// `async fn fetch(...)` と書くこともできるが、そうすると返ってくる Future が
/// `Send` かどうかを trait の利用者が指定できない（tokio::spawn に渡せるか分からない）。
/// そこで「`Send` な Future を返す」と戻り値の型で約束している。
/// 実装する側は、`async fn` で書いてよい（中身が Send であればコンパイルが通る）。
pub trait Fetch {
    fn fetch(&self, path: &str) -> impl Future<Output = Result<String, FetchError>> + Send;
}

/// 取得の設定。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// 同時に走らせる数の上限。0 だと何も進まないので、型で 0 を禁止している（Lesson 02-1）。
    pub concurrency: NonZeroUsize,
    /// 1件あたりの制限時間。
    pub timeout: Duration,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            concurrency: NonZeroUsize::new(4).expect("4 は 0 ではない"),
            timeout: Duration::from_secs(5),
        }
    }
}

/// HTTP 応答（ヘッダーと本文を含むバイト列全体）から、本文を取り出す。
///
/// - 1行目は `HTTP/1.x <状態コード> <説明>` の形
/// - ヘッダーと本文は空行（`\r\n\r\n`）で区切られている
/// - 状態コードが 200 なら本文を返す。それ以外は `Status(code)`
/// - 形が崩れている、本文が UTF-8 でない、などは `BadResponse`
pub fn parse_response(raw: &[u8]) -> Result<String, FetchError> {
    todo!("UTF-8 として読み、空行で head と body に分け、1行目から状態コードを取り出してください")
}

/// tokio の TcpStream で HTTP/1.0 の GET を送る、本物の取得処理。
///
/// HTTP/1.0 にしているのは、サーバーが応答を送り終えたら接続を閉じてくれるから。
/// 「接続が閉じるまで読む」（read_to_end）だけで応答全体が手に入る。
#[derive(Debug, Clone)]
pub struct HttpFetcher {
    addr: SocketAddr,
}

impl HttpFetcher {
    pub fn new(addr: SocketAddr) -> Self {
        HttpFetcher { addr }
    }
}

impl Fetch for HttpFetcher {
    async fn fetch(&self, path: &str) -> Result<String, FetchError> {
        // 手順: TcpStream::connect → リクエストの文字列を write_all → read_to_end → parse_response
        // リクエストの形は上のコメントと本文ページを参照（最後に空行が必要）
        todo!("connect, write_all, read_to_end の順に await し、最後に parse_response を呼んでください")
    }
}

/// 1件を、制限時間つきで取得する。
///
/// 制限時間を過ぎると、`fetcher.fetch(path)` の Future は**途中で捨てられる**（drop される）。
/// async では、これが「処理の中断」になる。
pub async fn fetch_one<F: Fetch>(
    fetcher: &F,
    path: &str,
    timeout: Duration,
) -> Result<String, FetchError> {
    todo!("tokio::time::timeout で fetcher.fetch(path) を包み、時間切れを FetchError::Timeout に変えてください")
}

/// たくさんの path を同時に取得する。
///
/// - 結果は `paths` と**同じ順番**で返す（終わった順ではない）
/// - 同時に走るのは、最大で `options.concurrency` 件
/// - 1件ごとに `options.timeout` の制限時間がある
/// - 1件が失敗（panic を含む）しても、他の取得は続ける
///
/// `fetcher` を `Arc` で受け取るのは、tokio::spawn に渡す Future が `'static` でなければならず
/// （Lesson 11-3）、`&F` を持ち込めないから。各タスクが Arc の複製を持つ。
pub async fn fetch_all<F>(
    fetcher: Arc<F>,
    paths: Vec<String>,
    options: &Options,
) -> Vec<Result<String, FetchError>>
where
    F: Fetch + Send + Sync + 'static,
{
    // 手順の例（本文ページの「進め方」も参照）:
    // 1. Semaphore を concurrency 個の空きで作り、Arc で包む
    // 2. path ごとに: 空きを acquire_owned で1つ取ってから tokio::spawn する
    //    （タスクの中で fetch_one を呼び、終わったら permit を手放す）
    // 3. JoinHandle を起こした順に await し、panic していたら FetchError::Panicked にする
    todo!("Semaphore で同時実行数を制限しながら tokio::spawn し、起こした順に結果を集めてください")
}

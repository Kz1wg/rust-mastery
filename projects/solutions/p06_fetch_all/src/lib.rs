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
    let text = std::str::from_utf8(raw).map_err(|_| FetchError::BadResponse)?;
    let (head, body) = text.split_once("\r\n\r\n").ok_or(FetchError::BadResponse)?;
    let status_line = head.lines().next().ok_or(FetchError::BadResponse)?;

    let mut parts = status_line.split_whitespace();
    let version = parts.next().ok_or(FetchError::BadResponse)?;
    if !version.starts_with("HTTP/1.") {
        return Err(FetchError::BadResponse);
    }
    let status: u16 = parts
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or(FetchError::BadResponse)?;

    if status == 200 {
        Ok(body.to_string())
    } else {
        Err(FetchError::Status(status))
    }
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
        let mut stream = TcpStream::connect(self.addr).await?;
        let request = format!("GET {path} HTTP/1.0\r\nHost: {}\r\n\r\n", self.addr);
        stream.write_all(request.as_bytes()).await?;

        let mut raw = Vec::new();
        stream.read_to_end(&mut raw).await?;
        parse_response(&raw)
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
    match tokio::time::timeout(timeout, fetcher.fetch(path)).await {
        Ok(result) => result,
        Err(_elapsed) => Err(FetchError::Timeout),
    }
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
    let semaphore = Arc::new(Semaphore::new(options.concurrency.get()));
    let mut handles = Vec::with_capacity(paths.len());

    for path in paths {
        // 空きが出るまで、ここで待つ。タスクを起こす前に待つので、
        // 1万件渡されても、同時に存在するタスクは concurrency 件まで。
        let permit = Arc::clone(&semaphore)
            .acquire_owned()
            .await
            .expect("semaphore は close しないので、acquire は失敗しない");
        let fetcher = Arc::clone(&fetcher);
        let timeout = options.timeout;

        handles.push(tokio::spawn(async move {
            let result = fetch_one(fetcher.as_ref(), &path, timeout).await;
            drop(permit); // 終わったら空きを返す（書かなくてもタスクの終わりで drop される）
            result
        }));
    }

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        // JoinHandle を起こした順に待つので、結果の順番は paths と同じになる。
        // タスクが panic すると Err(JoinError) が返る。
        results.push(handle.await.unwrap_or(Err(FetchError::Panicked)));
    }
    results
}

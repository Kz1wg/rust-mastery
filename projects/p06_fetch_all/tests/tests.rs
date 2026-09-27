use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use p06_fetch_all::{
    fetch_all, fetch_one, parse_response, Fetch, FetchError, HttpFetcher, Options,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::time::Instant;

// ---------------------------------------------------------------------------
// parse_response（ここは async ではない。ふつうの #[test] で確かめられる）
// ---------------------------------------------------------------------------

#[test]
fn parse_ok_returns_body() {
    let raw = b"HTTP/1.0 200 OK\r\nContent-Type: text/plain\r\n\r\nhello";
    assert_eq!(parse_response(raw), Ok("hello".to_string()));
}

#[test]
fn parse_accepts_http_1_1_and_empty_body() {
    assert_eq!(
        parse_response(b"HTTP/1.1 200 OK\r\n\r\n"),
        Ok(String::new())
    );
}

#[test]
fn parse_non_200_is_status_error() {
    let raw = b"HTTP/1.0 404 Not Found\r\n\r\nnot here";
    assert_eq!(parse_response(raw), Err(FetchError::Status(404)));
}

#[test]
fn parse_broken_responses_are_bad_response() {
    // 区切りの空行がない
    assert_eq!(
        parse_response(b"HTTP/1.0 200 OK\r\n"),
        Err(FetchError::BadResponse)
    );
    // HTTP ではない
    assert_eq!(
        parse_response(b"SSH-2.0\r\n\r\n"),
        Err(FetchError::BadResponse)
    );
    // 状態コードが数でない
    assert_eq!(
        parse_response(b"HTTP/1.0 OK\r\n\r\n"),
        Err(FetchError::BadResponse)
    );
    // UTF-8 でない
    assert_eq!(
        parse_response(b"HTTP/1.0 200 OK\r\n\r\n\xff"),
        Err(FetchError::BadResponse)
    );
}

// ---------------------------------------------------------------------------
// 偽物の Fetch。時間は tokio の時計（start_paused で止めてある）で進む
// ---------------------------------------------------------------------------

/// path ごとに、かかる時間を決めておける偽物。同時に何件走ったかも記録する。
#[derive(Default)]
struct FakeFetcher {
    delays_ms: HashMap<String, u64>,
    in_flight: AtomicUsize,
    max_in_flight: AtomicUsize,
}

impl FakeFetcher {
    fn with_delays(pairs: &[(&str, u64)]) -> Self {
        FakeFetcher {
            delays_ms: pairs.iter().map(|(p, ms)| (p.to_string(), *ms)).collect(),
            ..Default::default()
        }
    }
}

/// 「走っている数」を減らすのを Drop に任せる。
/// timeout で Future が途中で捨てられても、減らし忘れない（問い2 を参照）。
struct InFlight<'a>(&'a AtomicUsize);

impl Drop for InFlight<'_> {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}

impl Fetch for FakeFetcher {
    async fn fetch(&self, path: &str) -> Result<String, FetchError> {
        let now = self.in_flight.fetch_add(1, Ordering::SeqCst) + 1;
        self.max_in_flight.fetch_max(now, Ordering::SeqCst);
        let _guard = InFlight(&self.in_flight);

        let ms = self.delays_ms.get(path).copied().unwrap_or(100);
        tokio::time::sleep(Duration::from_millis(ms)).await;

        match path {
            "panic" => panic!("偽物がわざと panic しました"),
            p if p.starts_with("err") => Err(FetchError::Status(500)),
            p => Ok(format!("body of {p}")),
        }
    }
}

fn paths(names: &[&str]) -> Vec<String> {
    names.iter().map(|s| s.to_string()).collect()
}

fn options(concurrency: usize, timeout_ms: u64) -> Options {
    Options {
        concurrency: NonZeroUsize::new(concurrency).unwrap(),
        timeout: Duration::from_millis(timeout_ms),
    }
}

#[tokio::test(start_paused = true)]
async fn fetch_one_returns_the_result() {
    let fake = FakeFetcher::default();
    let r = fetch_one(&fake, "a", Duration::from_secs(1)).await;
    assert_eq!(r, Ok("body of a".to_string()));
}

#[tokio::test(start_paused = true)]
async fn fetch_one_times_out() {
    let fake = FakeFetcher::with_delays(&[("slow", 10_000)]);
    let start = Instant::now();
    let r = fetch_one(&fake, "slow", Duration::from_secs(1)).await;
    assert_eq!(r, Err(FetchError::Timeout));
    // 10秒を待たず、制限時間の1秒で返ってくる
    assert_eq!(start.elapsed(), Duration::from_secs(1));
}

#[tokio::test(start_paused = true)]
async fn timed_out_future_is_dropped() {
    let fake = FakeFetcher::with_delays(&[("slow", 10_000)]);
    let _ = fetch_one(&fake, "slow", Duration::from_secs(1)).await;
    // 途中で捨てられた Future の Drop（InFlight）が動いている
    assert_eq!(fake.in_flight.load(Ordering::SeqCst), 0);
}

#[tokio::test(start_paused = true)]
async fn fetch_all_keeps_input_order() {
    // 最初の path がいちばん遅い。終わった順ではなく、渡した順で返ってくること
    let fake = Arc::new(FakeFetcher::with_delays(&[
        ("a", 300),
        ("b", 200),
        ("c", 100),
    ]));
    let results = fetch_all(fake, paths(&["a", "b", "c"]), &options(3, 1_000)).await;
    assert_eq!(
        results,
        vec![
            Ok("body of a".to_string()),
            Ok("body of b".to_string()),
            Ok("body of c".to_string()),
        ]
    );
}

#[tokio::test(start_paused = true)]
async fn fetch_all_runs_concurrently() {
    // 100ms かかるものが5件。同時に走れば、全体でも 100ms で終わる
    let fake = Arc::new(FakeFetcher::default());
    let start = Instant::now();
    let results = fetch_all(fake, paths(&["a", "b", "c", "d", "e"]), &options(5, 1_000)).await;
    assert_eq!(results.len(), 5);
    assert_eq!(start.elapsed(), Duration::from_millis(100));
}

#[tokio::test(start_paused = true)]
async fn fetch_all_respects_concurrency_limit() {
    // 10件を、同時に3件まで → 3, 3, 3, 1 の4回に分かれて 400ms
    let fake = Arc::new(FakeFetcher::default());
    let names: Vec<String> = (0..10).map(|i| format!("item{i}")).collect();
    let start = Instant::now();
    let results = fetch_all(Arc::clone(&fake), names, &options(3, 1_000)).await;

    assert!(results.iter().all(|r| r.is_ok()));
    assert_eq!(fake.max_in_flight.load(Ordering::SeqCst), 3);
    assert_eq!(start.elapsed(), Duration::from_millis(400));
}

#[tokio::test(start_paused = true)]
async fn one_failure_does_not_stop_the_others() {
    let fake = Arc::new(FakeFetcher::with_delays(&[("slow", 10_000)]));
    let results = fetch_all(fake, paths(&["a", "err1", "slow", "b"]), &options(4, 1_000)).await;
    assert_eq!(
        results,
        vec![
            Ok("body of a".to_string()),
            Err(FetchError::Status(500)),
            Err(FetchError::Timeout),
            Ok("body of b".to_string()),
        ]
    );
}

#[tokio::test(start_paused = true)]
async fn a_panicking_task_becomes_an_error() {
    let fake = Arc::new(FakeFetcher::default());
    let results = fetch_all(fake, paths(&["a", "panic", "b"]), &options(2, 1_000)).await;
    assert_eq!(
        results,
        vec![
            Ok("body of a".to_string()),
            Err(FetchError::Panicked),
            Ok("body of b".to_string()),
        ]
    );
}

#[tokio::test(start_paused = true)]
async fn empty_input_gives_empty_output() {
    let fake = Arc::new(FakeFetcher::default());
    let results = fetch_all(fake, Vec::new(), &Options::default()).await;
    assert!(results.is_empty());
}

// ---------------------------------------------------------------------------
// 本物の HttpFetcher。テストの中でローカルのサーバーを立てる（外部には出ない）
// ---------------------------------------------------------------------------

/// 127.0.0.1 の空いているポートで、小さな HTTP サーバーを起動する。
/// - /hello → 200 "hello"
/// - それ以外 → 404
async fn start_local_server() -> std::net::SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                // リクエストの終わり（空行）まで読む
                let mut buf = Vec::new();
                let mut chunk = [0u8; 256];
                while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
                    let n = socket.read(&mut chunk).await.unwrap();
                    if n == 0 {
                        return;
                    }
                    buf.extend_from_slice(&chunk[..n]);
                }
                let request = String::from_utf8_lossy(&buf);
                let response: &[u8] = if request.starts_with("GET /hello ") {
                    b"HTTP/1.0 200 OK\r\nContent-Type: text/plain\r\n\r\nhello"
                } else {
                    b"HTTP/1.0 404 Not Found\r\n\r\nnot found"
                };
                socket.write_all(response).await.unwrap();
                // ここで socket が drop され、接続が閉じる（HTTP/1.0 の約束）
            });
        }
    });
    addr
}

#[tokio::test]
async fn http_fetcher_talks_to_a_local_server() {
    let addr = start_local_server().await;
    let fetcher = HttpFetcher::new(addr);
    assert_eq!(fetcher.fetch("/hello").await, Ok("hello".to_string()));
    assert_eq!(fetcher.fetch("/nope").await, Err(FetchError::Status(404)));
}

#[tokio::test]
async fn http_fetcher_reports_connection_errors() {
    // 一度ポートを取って、すぐ閉じる → そこには誰もいない
    let addr = {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        listener.local_addr().unwrap()
    };
    let fetcher = HttpFetcher::new(addr);
    assert!(matches!(
        fetcher.fetch("/hello").await,
        Err(FetchError::Io(_))
    ));
}

#[tokio::test]
async fn fetch_all_with_http_fetcher() {
    let addr = start_local_server().await;
    let fetcher = Arc::new(HttpFetcher::new(addr));
    let results = fetch_all(
        fetcher,
        paths(&["/hello", "/x", "/hello"]),
        &Options::default(),
    )
    .await;
    assert_eq!(
        results,
        vec![
            Ok("hello".to_string()),
            Err(FetchError::Status(404)),
            Ok("hello".to_string()),
        ]
    );
}

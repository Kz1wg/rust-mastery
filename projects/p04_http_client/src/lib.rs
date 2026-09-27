//! P4: 再試行つきの小さな HTTP クライアント。
//!
//! 通信そのものは `Transport` trait の向こう側に隠してある。
//! - テストでは、決まった応答を返す偽物（テストファイルの FakeTransport）に差し替える
//! - 本物として、標準ライブラリの TcpStream で HTTP/1.1 を話す `StdTransport` を用意してある（http のみ。https は扱わない）
//!
//! Client が気にするのは「送って、応答を受け取る」ことだけで、どうやって送るかは知らない。

use std::fmt;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

/// HTTP のリクエスト（この教材で必要な分だけ）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: &'static str,
    pub url: String,
}

/// HTTP の応答。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

/// 通信そのものの失敗（つながらない、途中で切れた、など）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportError(pub String);

/// リクエストを送り、応答を受け取るもの。
///
/// send が `&self` なのは、1つの Transport を複数の場所から共有して使えるようにするため。
/// そのかわり、状態を変えたい実装（テスト用の記録など）は interior mutability を使う（Lesson 10-3）。
pub trait Transport {
    fn send(&self, request: &Request) -> Result<Response, TransportError>;
}

/// Client の失敗。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError {
    /// サーバーがエラーの状態コードを返した（4xx、または再試行しても 5xx だった）。
    Status(u16),
    /// 再試行しても通信に失敗した（最後の失敗の内容を持つ）。
    Transport(TransportError),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::Status(code) => write!(f, "サーバーがエラーを返しました: {code}"),
            ClientError::Transport(e) => write!(f, "通信に失敗しました: {}", e.0),
        }
    }
}

impl std::error::Error for ClientError {}

/// 再試行つきのクライアント。T はどんな Transport でもよい（generic、Lesson 04-2）。
pub struct Client<T: Transport> {
    transport: T,
    base_url: String,
    max_retries: u32,
}

impl<T: Transport> Client<T> {
    /// base_url は "http://example.com/api" のような形。末尾の / はあってもなくてもよい。
    pub fn new(transport: T, base_url: &str) -> Self {
        Client {
            transport,
            base_url: base_url.to_string(),
            max_retries: 2,
        }
    }

    /// 失敗したときに、最大で何回やり直すか（既定は 2）。
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// base_url と path をつないだ URL を返す。間の / はちょうど1つにする。
    ///
    /// 例: "http://h/api" と "users" → "http://h/api/users"
    ///     "http://h/api/" と "/users" → "http://h/api/users"
    pub fn url_for(&self, path: &str) -> String {
        todo!("base_url の末尾の / と path の先頭の / を取り除いてから、/ 1つでつないでください")
    }

    /// GET を送る。
    ///
    /// - 2xx・3xx: Ok で応答を返す
    /// - 4xx: 呼び出し側の間違いなので、再試行せずに ClientError::Status を返す
    /// - 5xx・通信の失敗: 一時的な問題かもしれないので、max_retries 回まで再試行する
    ///   （最初の1回 + 再試行 = 最大 1 + max_retries 回送る）
    /// - 再試行しても駄目なら、最後の失敗を ClientError として返す
    pub fn get(&self, path: &str) -> Result<Response, ClientError> {
        let request = Request {
            method: "GET",
            url: self.url_for(path),
        };
        let mut last_error = ClientError::Transport(TransportError("未送信".to_string()));

        for _attempt in 0..=self.max_retries {
            let _ = (&request, &mut last_error);
            todo!("transport.send の結果で分け、上のルールどおりに return するか、last_error を更新して次の試行へ進んでください")
        }
        Err(last_error)
    }
}

/// 本物の Transport。標準ライブラリの TcpStream で HTTP/1.1 を話す（http のみ）。
///
/// この実装は完成している。読み物として、また「差し替えられる」ことの確認用に用意してある。
/// 実務では reqwest や ureq のような crate を使う（https、リダイレクト、タイムアウトなどが必要になるため）。
pub struct StdTransport;

impl Transport for StdTransport {
    fn send(&self, request: &Request) -> Result<Response, TransportError> {
        let err = |e: std::io::Error| TransportError(e.to_string());

        let rest = request.url.strip_prefix("http://").ok_or_else(|| {
            TransportError("http:// で始まる URL だけに対応しています".to_string())
        })?;
        let (host, path) = match rest.find('/') {
            Some(i) => (&rest[..i], &rest[i..]),
            None => (rest, "/"),
        };

        let mut stream = TcpStream::connect(host).map_err(err)?;
        // リクエストは1つの文字列に組み立ててから、write_all で一度に送る。
        // write!(stream, ...) だと、書式の部品ごとに別々に送られることがあり、
        // 受け取る側が途中までしか読まないうちに次の処理へ進んでしまう（テストが不安定になる）。
        let head = format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            request.method, path, host
        );
        stream.write_all(head.as_bytes()).map_err(err)?;

        let mut reader = BufReader::new(stream);
        let mut status_line = String::new();
        reader.read_line(&mut status_line).map_err(err)?;
        let status: u16 = status_line
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| TransportError(format!("状態行を読めません: {status_line:?}")))?;

        // ヘッダーは読み飛ばす（空行まで）
        loop {
            let mut line = String::new();
            let n = reader.read_line(&mut line).map_err(err)?;
            if n == 0 || line == "\r\n" || line == "\n" {
                break;
            }
        }

        let mut body = String::new();
        reader.read_to_string(&mut body).map_err(err)?;
        Ok(Response { status, body })
    }
}

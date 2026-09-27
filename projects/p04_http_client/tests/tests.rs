use p04_http_client::{
    Client, ClientError, Request, Response, StdTransport, Transport, TransportError,
};
use std::cell::RefCell;
use std::collections::VecDeque;

/// テスト用の偽物。決まった順に応答を返し、送られたリクエストを記録する。
/// send は &self なので、記録するために RefCell を使う（Lesson 10-3）。
struct FakeTransport {
    replies: RefCell<VecDeque<Result<Response, TransportError>>>,
    sent: RefCell<Vec<Request>>,
}

impl FakeTransport {
    fn new(replies: Vec<Result<Response, TransportError>>) -> Self {
        FakeTransport {
            replies: RefCell::new(replies.into()),
            sent: RefCell::new(Vec::new()),
        }
    }
}

impl Transport for FakeTransport {
    fn send(&self, request: &Request) -> Result<Response, TransportError> {
        self.sent.borrow_mut().push(request.clone());
        self.replies
            .borrow_mut()
            .pop_front()
            .expect("用意した応答より多く送られた")
    }
}

// Client は Transport を所有するので、テストでは &FakeTransport に対しても Transport を実装して、
// 送られたリクエストを後から確かめられるようにする。
impl Transport for &FakeTransport {
    fn send(&self, request: &Request) -> Result<Response, TransportError> {
        (*self).send(request)
    }
}

fn ok(status: u16, body: &str) -> Result<Response, TransportError> {
    Ok(Response {
        status,
        body: body.to_string(),
    })
}

fn down() -> Result<Response, TransportError> {
    Err(TransportError("connection refused".to_string()))
}

#[test]
fn url_for_joins_with_exactly_one_slash() {
    let fake = FakeTransport::new(vec![]);
    let c = Client::new(&fake, "http://h/api/");
    assert_eq!(c.url_for("/users"), "http://h/api/users");
    assert_eq!(c.url_for("users"), "http://h/api/users");
    let c2 = Client::new(&fake, "http://h/api");
    assert_eq!(c2.url_for("users"), "http://h/api/users");
}

#[test]
fn success_on_first_try() {
    let fake = FakeTransport::new(vec![ok(200, "hello")]);
    let c = Client::new(&fake, "http://h");
    assert_eq!(c.get("/x").unwrap().body, "hello");
    assert_eq!(fake.sent.borrow().len(), 1);
    assert_eq!(fake.sent.borrow()[0].url, "http://h/x");
}

#[test]
fn client_error_is_not_retried() {
    let fake = FakeTransport::new(vec![ok(404, "not found")]);
    let c = Client::new(&fake, "http://h");
    assert_eq!(c.get("/x"), Err(ClientError::Status(404)));
    assert_eq!(fake.sent.borrow().len(), 1, "4xx は再試行しない");
}

#[test]
fn server_error_is_retried_until_success() {
    let fake = FakeTransport::new(vec![ok(503, ""), down(), ok(200, "finally")]);
    let c = Client::new(&fake, "http://h").with_max_retries(2);
    assert_eq!(c.get("/x").unwrap().body, "finally");
    assert_eq!(fake.sent.borrow().len(), 3);
}

#[test]
fn gives_up_after_max_retries_with_last_error() {
    let fake = FakeTransport::new(vec![down(), ok(500, ""), ok(502, "")]);
    let c = Client::new(&fake, "http://h").with_max_retries(2);
    assert_eq!(c.get("/x"), Err(ClientError::Status(502)));
    assert_eq!(fake.sent.borrow().len(), 3, "最初の1回 + 再試行2回");

    let fake2 = FakeTransport::new(vec![ok(500, ""), down()]);
    let c2 = Client::new(&fake2, "http://h").with_max_retries(1);
    assert_eq!(
        c2.get("/x"),
        Err(ClientError::Transport(TransportError(
            "connection refused".to_string()
        )))
    );
}

#[test]
fn zero_retries_means_one_attempt() {
    let fake = FakeTransport::new(vec![ok(500, "")]);
    let c = Client::new(&fake, "http://h").with_max_retries(0);
    assert_eq!(c.get("/x"), Err(ClientError::Status(500)));
    assert_eq!(fake.sent.borrow().len(), 1);
}

/// 本物の Transport に差し替えても、Client のコードは同じまま動く。
/// 外部には通信せず、自分のマシン（127.0.0.1）で小さなサーバーを立てて確かめる。
#[test]
fn real_transport_against_a_local_server() {
    use std::io::{Read, Write};
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut conn, _) = listener.accept().unwrap();
        let mut buf = [0u8; 1024];
        let n = conn.read(&mut buf).unwrap();
        let request = String::from_utf8_lossy(&buf[..n]).to_string();
        conn.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello")
            .unwrap();
        request
    });

    let c = Client::new(StdTransport, &format!("http://{addr}/api"));
    let response = c.get("greeting").unwrap();
    assert_eq!(
        response,
        Response {
            status: 200,
            body: "hello".to_string()
        }
    );

    let request = server.join().unwrap();
    assert!(request.starts_with("GET /api/greeting HTTP/1.1"));
}

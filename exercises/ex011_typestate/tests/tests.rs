use ex011_typestate::{Request, RequestBuilder};

#[test]
fn build_with_url_only() {
    let req = RequestBuilder::new().url("https://example.com").build();
    assert_eq!(
        req,
        Request {
            url: "https://example.com".to_string(),
            headers: vec![],
        }
    );
}

#[test]
fn build_with_headers() {
    let req = RequestBuilder::new()
        .url("https://example.com")
        .header("Accept", "application/json")
        .header("X-Trace", "abc")
        .build();

    assert_eq!(req.url, "https://example.com");
    assert_eq!(
        req.headers,
        vec![
            ("Accept".to_string(), "application/json".to_string()),
            ("X-Trace".to_string(), "abc".to_string()),
        ]
    );
}

#[test]
fn default_starts_with_no_url() {
    // Default は NoUrl から始まる。url() を呼ばないと build() を呼べないことは
    // 型レベルで保証されている（compile_fail doctest を参照）。
    let req = RequestBuilder::default().url("https://a.example").build();
    assert_eq!(req.url, "https://a.example");
}

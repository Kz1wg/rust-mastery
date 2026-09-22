//! Lesson 03-3: typestate pattern
//!
//! `url()` を呼ぶ前は `build()` が**存在しない**（コンパイルエラーになる）。
//! URL は `Option<String>` ではなく `HasUrl` 状態の中に持たせているので、
//! `build()` で `unwrap` する必要が無い。「URLがある」ことを型で表した結果である。

/// URL がまだ設定されていない状態。
pub struct NoUrl;

/// URL が設定済みの状態。URL 自体をこの状態が持つ。
/// フィールドは private なので、外部から `HasUrl` を偽造することはできない。
pub struct HasUrl {
    url: String,
}

pub struct RequestBuilder<S> {
    state: S,
    headers: Vec<(String, String)>,
}

#[derive(Debug, PartialEq)]
pub struct Request {
    pub url: String,
    pub headers: Vec<(String, String)>,
}

impl RequestBuilder<NoUrl> {
    pub fn new() -> Self {
        RequestBuilder {
            state: NoUrl,
            headers: Vec::new(),
        }
    }

    /// url を設定すると、`HasUrl` 状態に遷移する（以降 `build()` が使える）。
    ///
    /// `url()` を呼ばずに `build()` を呼ぼうとすると、コンパイルできない
    /// （`RequestBuilder<NoUrl>` に `build` は存在しない）:
    ///
    /// ```compile_fail
    /// use ex011_typestate::RequestBuilder;
    ///
    /// let req = RequestBuilder::new().build(); // NoUrl には build がない
    /// ```
    pub fn url(self, url: &str) -> RequestBuilder<HasUrl> {
        todo!("state を HasUrl（url を保持）にした RequestBuilder を返してください（headers は引き継ぐ）")
    }
}

impl Default for RequestBuilder<NoUrl> {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestBuilder<HasUrl> {
    /// ヘッダーを追加する。`HasUrl` のままなので、何度でも呼べる。
    pub fn header(self, key: &str, value: &str) -> Self {
        todo!("headers に (key, value) を追加した Self を返してください")
    }

    /// `HasUrl` 状態が URL を持っているので、unwrap は不要。
    pub fn build(self) -> Request {
        todo!("self.state の url と self.headers から Request を組み立ててください")
    }
}

/// URL がまだ設定されていない状態。
pub struct NoUrl;

/// URL が設定済みの状態。URL 自体をこの状態が持つ。
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

    /// ```compile_fail
    /// use ex011_typestate::RequestBuilder;
    ///
    /// let req = RequestBuilder::new().build(); // NoUrl には build がない
    /// ```
    pub fn url(self, url: &str) -> RequestBuilder<HasUrl> {
        RequestBuilder {
            state: HasUrl {
                url: url.to_string(),
            },
            headers: self.headers,
        }
    }
}

impl Default for RequestBuilder<NoUrl> {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestBuilder<HasUrl> {
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    pub fn build(self) -> Request {
        Request {
            url: self.state.url,
            headers: self.headers,
        }
    }
}

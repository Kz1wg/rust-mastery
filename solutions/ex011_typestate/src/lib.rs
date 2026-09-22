use std::marker::PhantomData;

pub struct NoUrl;
pub struct HasUrl;

pub struct RequestBuilder<S> {
    url: Option<String>,
    headers: Vec<(String, String)>,
    _state: PhantomData<S>,
}

#[derive(Debug, PartialEq)]
pub struct Request {
    pub url: String,
    pub headers: Vec<(String, String)>,
}

impl RequestBuilder<NoUrl> {
    pub fn new() -> Self {
        RequestBuilder {
            url: None,
            headers: Vec::new(),
            _state: PhantomData,
        }
    }

    /// ```compile_fail
    /// use ex011_typestate::RequestBuilder;
    ///
    /// let req = RequestBuilder::new().build(); // NoUrl には build がない
    /// ```
    pub fn url(self, url: &str) -> RequestBuilder<HasUrl> {
        RequestBuilder {
            url: Some(url.to_string()),
            headers: self.headers,
            _state: PhantomData,
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
            url: self.url.unwrap(),
            headers: self.headers,
        }
    }
}

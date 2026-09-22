//! Lesson 03-3: typestate pattern
//!
//! `url()` を呼ぶ前は `build()` が**存在しない**（コンパイルエラーになる）。
//! 状態を型パラメータ（`NoUrl` / `HasUrl`）で表しているため、
//! `PhantomData` を使っている（詳しくは Chapter 09-1 で扱う。
//! ここでは「型だけの目印を持たせるための仕組み」とだけ理解すれば十分）。

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
        todo!("url を設定した RequestBuilder<HasUrl> を返してください（headers は引き継ぐ）")
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

    pub fn build(self) -> Request {
        todo!("url は unwrap してよい（HasUrl の時点で必ず Some）。Request を組み立てて返してください")
    }
}

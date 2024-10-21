use axum::{extract::Request, response::Response};
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};

use crate::Internationalization;

#[derive(Clone)]
pub struct I18nifyLayer<T: Internationalization + Clone> {
    locale: T,
    default_lang: String,
}

impl<T: Internationalization + Clone> I18nifyLayer<T> {
    pub fn new<S: AsRef<str>>(locale: T, default_lang: S) -> Self {
        Self {
            locale,
            default_lang: default_lang.as_ref().to_string(),
        }
    }
}

impl<S, T> Layer<S> for I18nifyLayer<T>
where
    T: Internationalization + Clone,
{
    type Service = I18nifyExtractor<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        I18nifyExtractor {
            inner,
            locale: self.locale.clone(),
            default_lang: self.default_lang.clone(),
        }
    }
}

#[derive(Clone)]
pub struct I18nifyExtractor<S, T> {
    inner: S,
    default_lang: String,
    locale: T,
}

impl<S, T> Service<Request> for I18nifyExtractor<S, T>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
    T: Internationalization + Clone,
    T::Item: Clone + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        let lang = request
            .headers()
            .get("Accept-Language")
            .and_then(|h| h.to_str().ok())
            .unwrap_or(&self.default_lang);

        let lang = parse_language(lang).unwrap_or(self.default_lang.clone());

        let r = self.locale.i(&lang);

        request.extensions_mut().insert(r);

        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            Ok(response)
        })
    }
}

/// 解析语言标识
/// zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6
fn parse_language(lang: &str) -> Option<String> {
    if lang.is_empty() {
        return None;
    }
    if lang == "*" {
        return None;
    }

    if lang.contains(",") {
        let languages = lang.split(",").collect::<Vec<_>>();

        return languages.first().map(|l| l.replace(";q=", ""));
    }

    None
}

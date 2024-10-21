use axum::{response::IntoResponse, routing::get, Extension, Router};
use demo::{DocLocale, Locale};
use i18nify::axum::I18nifyLayer;

mod demo {
    use i18nify::{Internationalization, I18N};

    #[derive(I18N,Clone)]
    #[i18n(folder = "$CARGO_MANIFEST_DIR/tests/zh_locales")]
    pub struct DocLocale;

    impl Internationalization for DocLocale {
        type Item = Locale;
        
        fn i(&self,lang:&str)->Self::Item {
            match lang.to_lowercase().as_str() {
                "en"=> Locale::En,
                "zh-cn"=> Locale::ZhCn,
                _=> Locale::En, 
            }
        }
    }

}


#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root))
        .layer(I18nifyLayer::new(DocLocale, "en"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(Extension(locale):Extension<Locale>) -> impl IntoResponse {
    locale.greeting()
}
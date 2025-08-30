# [i18nify](https://github.com/kingzcheung/i18nify)
[简体中文](./README_CN.md)| English

[![crates.io](https://img.shields.io/crates/v/i18nify.svg)](https://crates.io/crates/i18nify) [![Documentation](https://docs.rs/i18nify/badge.svg)](https://docs.rs/i18nify)


Internationalization library for Rust based on code generation.

> The original repository https://github.com/davidpdrsn/i18n_codegen was implemented by David Pedersen. However, it has some outdated dependencies and has not been maintained for as long as five years.

By leveraging code generation we are able to prevent common bugs like typos in i18n keys,
missing interpolations, or various mistakes between locales.

## Adding

```bash
cargo add i18nify #default features=['json']
```

or add `Cargo.toml`:

```bash
i18nify = { version = "0.3", features = ["json"] } #json
i18nify = { version = "0.3", features = ["toml"] } #toml
```

## Usage

It requires a directory (based on `CARGO_MANIFEST_DIR`) with one JSON file per locale. Here is an example with English and
Danish translations:

```javascript
// tests/doc_locales/en.json
{
    "hello_world": "Hello, World!",
    "greeting": "Hello {name}"
}

// tests/doc_locales/da.json
{
    "hello_world": "Hej, Verden!",
    "greeting": "Hej {name}"
}
```

And in Rust:


In `Rust` ：

```rust
use demo::Internationalize;

mod demo {
    use i18nify::I18N;
    #[derive(I18N)]
    #[i18n(folder = "tests/doc_locales")]
    pub struct DocLocale;

}

fn main() {
    // Based on the `Locale` enum type to retrieve internationalized text
    let hello = demo::Locale::En.hello_world();
    assert_eq!("Hello, World!",hello);
    println!("{}",hello);

    // Based on the `Internationalize` trait implemented with `DocLocale` to retrieve internationalized text
    let hello = demo::DocLocale.da().hello_world();
    println!("{}",hello);
}

```

Allow environment variables to be used in the folder path. Example:

In `Rust` ：

```rust
use demo::Internationalize;

mod demo {
    use i18nify::I18N;
    #[derive(I18N)]
    #[i18n(folder = "$CARGO_MANIFEST_DIR/tests/doc_locales")]
    pub struct DocLocale;
}

```


## Using in the `Axum` Framework

First, define an `Internationalization` trait implementation

```rust
use i18nify::{Internationalization, I18N};

#[derive(I18N, Clone)]
#[i18n(folder = "$CARGO_MANIFEST_DIR/tests/zh_locales")]
pub struct DocLocale;

impl Internationalization for DocLocale {
    type Item = Locale;

    fn i(&self, lang: &str) -> Self::Item {
        match lang.to_lowercase().as_str() {
            "en" => Locale::En,
            "zh-cn" => Locale::ZhCn,
            _ => Locale::En,
        }
    }
}

```

Then add the middleware `I18nifyLayer`:

```rust 
let app = Router::new()
    .route("/", get(root))
    .layer(I18nifyLayer::new(DocLocale, "en"));
```
Finally, you can use Locale to get internationalized text in your handler
```rust 
async fn root(Extension(locale): Extension<Locale>) -> impl IntoResponse {
    locale.greeting() // Hello, world
}
```

You can find more details on <https://docs.rs/i18nify>.

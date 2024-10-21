# [i18nify](https://github.com/kingzcheung/i18nify)

简体中文| [English](./README.md)

[![crates.io](https://img.shields.io/crates/v/i18nify.svg)](https://crates.io/crates/i18nify) [![Documentation](https://docs.rs/i18nify/badge.svg)](https://docs.rs/i18nify)

[i18nify](https://github.com/kingzcheung/i18nify) 是一款基于代码生成的 `Rust` 国际化库。

> 原仓库 [https://github.com/davidpdrsn/i18n_codegen](https://github.com/davidpdrsn/i18n_codegen) 是 [David Pedersen](https://github.com/davidpdrsn) 实现的。然而它有一些老旧的依赖，并且已经长达 5 年不维护。

通过利用代码生成，我们能够防止代码中的拼写错误、缺少插值或各语言环境之间各种错误的常见问题。

## 添加

```
cargo add i18nify #default features=['json']
```

or add `Cargo.toml`:

```
i18nify = { version = "0.3", features = ["json"] } #json
i18nify = { version = "0.3", features = ["toml"] } #toml
```

## 使用方式

它需要一个目录(目录基于`CARGO_MANIFEST_DIR`)，其中每个语言环境有一个 JSON 文件。以下是一个包含英语和汉语翻译的例子：

```javascript
// tests/doc_locales/en.json
{
    "hello_world": "Hello, World!",
    "greeting": "Hello {name}"
}

// tests/doc_locales/da.json
{
    "hello_world": "你好,世界！",
    "greeting": "你好, {name}"
}
```

在 `Rust` 中：

```rust
use demo::Internationalize;

mod demo {
    use i18nify::I18N;
    #[derive(I18N)]
    #[i18n(folder = "tests/locales")]
    pub struct DocLocale;

}

fn main() {
    // 基于 Locale 枚举类型获取国际化文本
    let hello = demo::Locale::En.hello();
    assert_eq!("Hello, World!",hello);
    println!("{}",hello);

    // 基于 `DocLocale` 实现的`Internationalize` trait 获取国际化文本
    let hello = demo::DocLocale.da().hello();
    println!("{}",hello);
}

```

`folder` 路径可以使用环境变量,比如：

```rust
use demo::Internationalize;

mod demo {
    use i18nify::I18N;
    #[derive(I18N)]
    #[i18n(folder = "$CARGO_MANIFEST_DIR/tests/doc_locales")]
    pub struct DocLocale;
}

```

## 在 `Axum` 框架中使用

先定义一个 `Internationalization` trait 实现

```rust
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
```

然后添加中间件`I18nifyLayer`：

```rust
let app = Router::new()
        .route("/", get(root))
        .layer(I18nifyLayer::new(DocLocale, "en"));
```

最后在 handler 中就可以使用 `Locale` 获取国际化文本

```rust
async fn root(Extension(locale):Extension<Locale>) -> impl IntoResponse {
    locale.greeting() // 你好，世界
}
```

你可以在 <[文档](https://docs.rs/i18nify)> 获取更多细节。

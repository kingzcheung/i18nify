# [i18nify](https://github.com/kingzcheung/i18nify)
简体中文| [English](./README.md)

[![crates.io](https://img.shields.io/crates/v/i18nify.svg)](https://crates.io/crates/i18nify) [![Documentation](https://docs.rs/i18nify/badge.svg)](https://docs.rs/i18nify)


[i18nify](https://github.com/kingzcheung/i18nify) 是一款基于代码生成的 `Rust` 国际化库。

> 原仓库 [https://github.com/davidpdrsn/i18nify](https://github.com/davidpdrsn/i18nify) 是 [David Pedersen](https://github.com/davidpdrsn) 实现的。然而它有一些老旧的依赖，并且已经长达5年不维护。

通过利用代码生成，我们能够防止代码中的拼写错误、缺少插值或各语言环境之间各种错误的常见问题。

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

你可以在 <[文档](https://docs.rs/i18nify)> 获取更多细节。
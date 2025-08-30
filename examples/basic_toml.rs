#![cfg(feature="toml")]

use demo::Internationalize;

mod demo {
    use i18nify::I18N;
    #[derive(I18N)]
    #[i18n(folder = "tests/toml_locales")]
    pub struct TomlLocale;

}

fn main() {
    let hello = demo::Locale::En.hello_world();
    println!("{hello}");

    let hello = demo::TomlLocale.zh_cn().hello_world();
    println!("{hello}");

}

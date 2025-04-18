#![cfg(feature="json")]

use demo::Internationalize;


mod demo {
    use i18nify::I18N;
    #[derive(I18N)]
    #[i18n(folder = "$CARGO_MANIFEST_DIR/tests/zh_locales")]
    pub struct DocLocale;

}

fn main() {
    let hello = demo::Locale::En.hello_world();
    println!("{}",hello);
    demo::Locale::ZhCn.hello_world();

    let hello = demo::DocLocale.zh_cn().hello_world();
    println!("{}",hello);
    
    let addr = demo::DocLocale.zh_cn().addressed_email(demo::Name("kingzcheung@gmail.com"));
    println!("{addr}");
}

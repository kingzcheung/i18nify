use demo::Internationalize;

mod demo {
    use i18nify::I18N;
    #[derive(I18N)]
    #[i18n(folder = "tests/zh_locales")]
    pub struct DocLocale;

}

fn main() {
    let hello = demo::Locale::En.hello_world();
    println!("{}",hello);

    let hello = demo::DocLocale.zh_cn().hello_world();
    println!("{}",hello);
}

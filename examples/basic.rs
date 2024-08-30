mod demo {
    use i18n_codegen::I18N;

    #[derive(I18N)]
    #[i18n(folder = "tests/locales")]
    pub struct DocLocale;
}

fn main() {
    let hello = demo::Locale::En.hello();
    assert_eq!("Hello, World!",hello);
    println!("{}",hello);
}

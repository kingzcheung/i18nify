mod demo {
    use i18n_codegen::i18n;

    #[i18n(
        folder = "/Users/kingzcheung/rust/i18n_codegen/tests/doc_locales",
        start = "{",
        end = "}"
    )]
    pub struct DocLocales;
}

fn main() {
    let name = demo::Name("John");
    println!("{:?}", demo::Locale::En.hello_world());
    dbg!(demo::Locale::En.greeting(name));
}

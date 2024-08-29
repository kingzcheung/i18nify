use i18n_codegen::i18n;


fn main() {
    i18n!("tests/doc_locales");

    let r = Locale::En.hello_world();
    
    println!("{r}");
}


fn example() {
    // let r = Locale::En.hello_world();
}
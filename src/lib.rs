//! Internationalization library based on code generation.
//!
//! By leveraging code generation we are able to prevent common bugs like typos in i18n keys,
//! missing interpolations, or various mistakes between locales.
//!
//! It requires a directory with one JSON file per locale. Here is an example with English and
//! Danish translations:
//!
//! ```json
//! // tests/doc_locales/en.json
//! {
//!     "hello_world": "Hello, World!",
//!     "greeting": "Hello {name}"
//! }
//!
//! // tests/doc_locales/da.json
//! {
//!     "hello_world": "Hej, Verden!",
//!     "greeting": "Hej {name}"
//! }
//! ```
//!
//! And in Rust:
//!
//! ```rust
//! use demo::Internationalize;
//! mod demo {
//!     use i18nify::I18N;
//!     #[derive(I18N)]
//!     #[i18n(folder = "tests/doc_locales")]
//!     pub struct DocLocale;
//! }
//! 
//! fn main() {
//!     // Based on the `Locale` enum type to retrieve internationalized text
//!     let hello = demo::Locale::En.hello_world();
//!     println!("{}",hello);// Hello, World!
//!     
//!     // Based on the `Internationalize` trait implemented with `DocLocale` to retrieve internationalized text
//!     let greeting = DocLocale::da().greeting(Name("John"));
//!     println!("{}",greeting);// Hej John
//!}
//! ```
//! 

#[allow(unused_imports)]
#[macro_use]
extern crate i18nify_macro;
pub use i18nify_macro::*;


#[cfg(feature = "axum")]
pub mod axum;


pub trait Internationalization {
    type Item;
    fn i(&self,lang:&str)->Self::Item;
}

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
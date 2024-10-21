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
#![deny(
    unused_imports,
    dead_code,
    unused_variables,
    unknown_lints,
    missing_docs,
    unused_must_use
)]
#![doc(html_root_url = "https://docs.rs/i18nify/0.2")]

// extern crate proc_macro;
// extern crate proc_macro2;

mod error;
mod placeholder_parsing;
mod schema;
mod utils;

use error::{Error, MissingKeysInLocale, Result};
use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use placeholder_parsing::find_placeholders;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use rayon::prelude::*;
use schema::{Config, I18nKey, Key, LocaleName, Placeholders, Translation, Translations};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};
use syn::{Attribute, DeriveInput, Expr, LitStr};
use utils::{locale_name_from_translations_file_path, parse_translations_file};

/// Generates the code for the `Locale` enum and such as the `Locale::hello_world()` methods.
/// 
/// ```rust
/// 
/// use i18nify::I18N;
/// 
/// #[derive(I18N)]
/// #[i18n(folder = "tests/doc_locales")]
/// pub struct DocLocale;
/// 
/// ```
/// 
/// `tests/doc_locales` is the folder where the translations are located.
/// 
/// ```javascript
/// //tests/doc_locales/en.json
/// {
///     "hello_world": "Hello World!"
/// }
/// ```
/// 
/// ```javascript
///  // tests/doc_locales/da.json
/// {
///     "hello_world": "Hej Verden!"
/// }
/// ```
/// 
/// ```rust
/// use i18nify::{I18N, Locale};
///
/// fn main() {
///     let locale = DocLocale::en();
///     assert_eq!(locale.hello_world(), "Hello World!");
/// 
///     let locale = DocLocale::da();
///     assert_eq!(locale.hello_world(), "Hej Verden!");
///}
#[proc_macro_derive(I18N, attributes(i18n))]
pub fn try_i18n(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { attrs, ident, .. } = syn::parse_macro_input!(input);
    
    match try_i18n_with_folder2(ident, attrs) {
        Ok(tokens) => tokens,
        Err(err) => panic!("{}", err),
    }
}
fn try_i18n_with_folder2(ident: Ident, attrs: Vec<Attribute>) -> Result<proc_macro::TokenStream> {
    let mut folder = None;
    let mut start = None;
    let mut end = None;
    
    attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident("i18n"))
        .try_for_each(|attr| {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("folder") {
                    folder = Some(meta.value()?.parse::<LitStr>()?);
                } else if meta.path.is_ident("start") {
                    start = Some(meta.value()?.parse::<LitStr>()?);
                } else if meta.path.is_ident("end") {
                    end = Some(meta.value()?.parse::<LitStr>()?);
                } else {
                    let _: Option<Expr> = meta.value().and_then(|v| v.parse()).ok();
                }

                Ok(())
            })
        })?;
    let folder = folder.map(|x| x.value()).ok_or_else(|| {
        syn::Error::new(
            ident.span(),
            "expected #[i18n(...)] attribute to be present when used with Locale derive trait",
        )
    })?;

    let folder_path = shellexpand::full(&folder).map_err(|e| syn::Error::new(
        ident.span(),
        e.to_string(),
    ))?.to_string();

    let locale_folder = if Path::new(&folder_path).is_relative() {
        let crate_root_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        crate_root_path.join(folder)
    }else {
        Path::new(&folder_path).to_path_buf()
    };
    
    
    if !locale_folder.is_dir() || !locale_folder.exists() {
        return Err(error::Error::ProcMacroInput(syn::Error::new(
            ident.span(),
            "`folder` must be a relative path.",
        )));
    }

    let start = start.map(|x| x.value()).unwrap_or("{".into());
    let end = end.map(|x| x.value()).unwrap_or("}".into());
    let config = Config {
        open: start,
        close: end,
    };
    
    let file_paths = crate::utils::find_locale_files(locale_folder)?;
    
    let paths_and_contents = file_paths
        .iter()
        .map(|path| {
            let contents = std::fs::read_to_string(path)?;
            Ok((path, contents))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    
    let translations = build_translations_from_files(&paths_and_contents, &config)?;
    validate_translations(&translations)?;
    
    let locales = build_locale_names_from_files(&file_paths)?;
    
    let mut output = TokenStream::new();
    gen_code(ident, locales, translations, &mut output);
    // let syntax_tree: syn::File = syn::parse2(output.clone()).unwrap();
    // let pretty = prettyplease::unparse(&syntax_tree);

    Ok(output.into())
}

fn gen_code(
    ident: Ident,
    locales: Vec<LocaleName>,
    translations: Translations,
    out: &mut TokenStream,
) {
    gen_impl_internationalize(&locales, out);
    gen_locale_enum(&locales, out);
    gen_i18n_struct(translations, out);
    out.extend(quote! {
        impl Internationalize for #ident {}
    })
}



fn gen_impl_internationalize(locales: &[LocaleName], out: &mut TokenStream) {
    let variants = locales.iter().map(|key| ident(&key.0));
    let fn_names = locales
        .iter()
        .map(|key| ident(&key.0.to_lower_camel_case()));

    let methods = fn_names.zip(variants).map(|(fn_name, variant)| {
        let fn_name = ident(&fn_name.to_string().to_snake_case());
        let variant = ident(&variant.to_string().to_upper_camel_case());
        quote! {
            fn #fn_name(&self) -> Locale {
                Locale::#variant
            }
        }
    });
    out.extend(quote! {
        pub trait Internationalize {
            #(#methods)*
        }
    });
}

fn gen_locale_enum(locales: &[LocaleName], out: &mut TokenStream) {
    let variants = locales.iter().map(|key| {
        let key = key.0.to_upper_camel_case();
        ident(&key)
    });

    out.extend(quote! {
        /// Locale enum generated by "i18nify"
        #[derive(Copy, Clone, Debug)]
        pub enum Locale {
            #(#variants),*
        }
    });
}

fn gen_i18n_struct(translations: Translations, out: &mut TokenStream) {
    let mut all_unique_placeholders = HashSet::<Ident>::new();

    let methods = translations
        .iter()
        .map(|(key, translations)| {
            let name = ident(&key.0);

            let mut placeholders = translations
                .iter()
                .flat_map(|(_, (_, placeholders))| placeholders.0.iter().map(|p| ident(p)))
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            placeholders.sort();

            for placeholder in &placeholders {
                all_unique_placeholders.insert(placeholder.clone());
            }

            let args = placeholders.iter().map(|placeholder| {
                let type_name = ident(&placeholder.to_string().to_upper_camel_case());
                quote! { #placeholder: #type_name<'_> }
            });

            let match_arms = translations.iter().map(|(locale_name, (translation, _))| {
                let locale_name = ident(&locale_name.0.to_upper_camel_case());
                let translation = translation.0.to_string();

                let body = if placeholders.is_empty() {
                    quote! { format!(#translation) }
                } else {
                    let fields = placeholders.iter().filter_map(|placeholder| {
                        let mut format_key = placeholder.to_string();
                        format_key.truncate(format_key.len() - 1);

                        let placehoder_with_open_close = format!(
                            "{open}{placeholder}{close}",
                            open = "{",
                            placeholder = format_key,
                            close = "}",
                        );
                        if translation.contains(&placehoder_with_open_close) {
                            let format_key = ident(&format_key);
                            Some(quote! { #format_key = #placeholder.0 })
                        } else {
                            None
                        }
                    });
                    quote! { format!(#translation, #(#fields),*) }
                };

                quote! {
                    Locale::#locale_name => #body
                }
            });
            quote! {
                #[allow(missing_docs)]
                pub fn #name(self, #(#args),*) -> String {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let placeholder_newtypes = all_unique_placeholders.into_iter().map(|placeholder| {
        let placeholder = ident(&placeholder.to_string().to_upper_camel_case());
        quote! {
            #[allow(missing_docs)]
            pub struct #placeholder<'a>(pub &'a str);
        }
    });

    out.extend(quote! {
        #(#placeholder_newtypes)*

        impl Locale {
            #(#methods)*
        }
    });
}

fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

fn build_translations_from_files(
    paths_and_contents: &[(&PathBuf, String)],
    config: &Config,
) -> Result<Translations> {
    
    let keys_per_locale = paths_and_contents
        .iter()
        .map(|(path, contents)| {
            let locale_name = locale_name_from_translations_file_path(path)?;
            
            let map = parse_translations_file(contents)?;
            
            let keys_in_file = build_keys_from_json(map, config, &locale_name)?;

            let locale_and_keys = keys_in_file
                .into_iter()
                .map(|key| (locale_name.clone(), key))
                .collect::<Vec<(LocaleName, I18nKey)>>();
            Ok(locale_and_keys)
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let keys_per_locale: HashMap<(LocaleName, Key), (Translation, Placeholders)> = keys_per_locale
        .into_iter()
        .flatten()
        .map(|(locale, key)| ((locale, key.key), (key.translation, key.placeholders)))
        .collect();

    let number_of_keys_per_locale = keys_per_locale.len() / paths_and_contents.len();
    let mut acc: Translations = HashMap::with_capacity(number_of_keys_per_locale);

    for ((locale_name, key), (translation, placeholders)) in keys_per_locale {
        let entry = acc
            .entry(key)
            .or_insert_with(|| HashMap::with_capacity(paths_and_contents.len()));
        entry.insert(locale_name, (translation, placeholders));
    }

    Ok(acc)
}

fn build_locale_names_from_files(file_paths: &[PathBuf]) -> Result<Vec<LocaleName>> {
    file_paths
        .iter()
        .map(locale_name_from_translations_file_path)
        .collect()
}

fn validate_translations(translations: &Translations) -> Result<()> {
    let all_keys = all_keys(translations);
    let keys_per_locale = keys_per_locale(translations);

    let mut errors = Vec::new();
    for (locale_name, keys) in keys_per_locale {
        let keys_missing = all_keys.difference(&keys).collect::<HashSet<_>>();
        if !keys_missing.is_empty() {
            let keys = keys_missing.iter().map(|key| (**key).clone()).collect();

            errors.push(MissingKeysInLocale {
                locale_name: locale_name.clone(),
                keys,
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error::MissingKeysInLocale(errors))
    }
}

fn all_keys(translations: &Translations) -> HashSet<&Key> {
    translations.keys().collect()
}

fn keys_per_locale(translations: &Translations) -> HashMap<&LocaleName, HashSet<&Key>> {
    let mut acc = HashMap::new();

    for (key, translations_for_key) in translations {
        for (locale_name, (_translation, _placeholders)) in translations_for_key {
            acc.entry(locale_name)
                .or_insert_with(HashSet::new)
                .insert(key);
        }
    }

    acc
}

fn build_keys_from_json(
    map: HashMap<String, String>,
    config: &Config,
    locale_name: &LocaleName,
) -> Result<Vec<I18nKey>> {
    map.into_par_iter()
        .map(|(key, value)| {
            let placeholders = find_placeholders(&value, &config.open, &config.close, locale_name)?;
            let value = value.replace(&config.open, "{").replace(&config.close, "}");
            let key = key.replace(".", "_").replace("-", "_");

            Ok(I18nKey {
                key: Key(key),
                translation: Translation(value),
                placeholders: Placeholders(placeholders),
            })
        })
        .collect()
}

#[allow(
    unused_imports,
    dead_code,
    unused_variables,
    unknown_lints,
    missing_docs,
    unused_must_use
)]
#[cfg(test)]
mod test {
    use std::path::Path;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[cfg(feature="json")]
    fn test_reading_files() {
        let input = "tests/locales";
        let crate_root_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let locale_path = crate_root_path.join(input).join(PathBuf::from("en.json"));

        let contents = std::fs::read_to_string(&locale_path).unwrap();
        let map = parse_translations_file(&contents).unwrap();
        let mut keys =
            build_keys_from_json(map, &Config::default(), &LocaleName::new("test")).unwrap();
        keys.sort_by_key(|key| key.key.0.clone());

        assert_eq!(keys[0].key.0, "duplicate_placeholders");
        assert_eq!(keys[0].translation.0, "Hey {name}. Is your name {name}?");
        assert_eq!(to_vec(keys[0].placeholders.0.clone()), vec!["name_"]);
    }

    #[test]
    #[cfg(feature="json")]
    fn test_finding_locale_names() {
        let input = "tests/locales";
        let crate_root_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let locale_path = crate_root_path.join(input).join(PathBuf::from("en.json"));

        let locale_name = locale_name_from_translations_file_path(&locale_path).unwrap();

        assert_eq!(locale_name.0, "En");
    }

    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_fail/*.rs");
    }

    #[test]
    fn test_html_root_url() {
        version_sync::assert_html_root_url_updated!("src/lib.rs");
    }

    fn to_vec<T: std::hash::Hash + Eq>(set: HashSet<T>) -> Vec<T> {
        set.into_iter().collect()
    }

    #[test]
    #[cfg(feature="json")]
    fn test_build_locale_names_from_files()->Result<(), Box<dyn std::error::Error>> {

        let file_paths = &[
            ("zh_cn",PathBuf::from("tests/zh_locales/zh_CN.json")),
           ("en",PathBuf::from("tests/zh_locales/en.json")),
        ];

        let paths = file_paths.iter().map(|f| f.1.clone()).collect::<Vec<_>>();
        let names = file_paths.iter().map(|f| f.0.to_string()).collect::<Vec<_>>();

        let locales = super::build_locale_names_from_files(&paths).unwrap();
        locales
        .iter()
        .enumerate()
        // .map(|key| ident(&key.0.to_lower_camel_case())).collect::<Vec<_>>();
        .for_each(|(index,name)| {
            assert_eq!(name.0.to_snake_case(),names[index])
        });

        Ok(())
    }

    #[test]
    #[cfg(feature="toml")]
    fn test_build_locale_names_from_files()->Result<(), Box<dyn std::error::Error>> {

        let file_paths = &[
            ("zh_cn",PathBuf::from("tests/toml_locales/zh_CN.toml")),
           ("en",PathBuf::from("tests/toml_locales/en.toml")),
        ];

        let paths = file_paths.iter().map(|f| f.1.clone()).collect::<Vec<_>>();
        let names = file_paths.iter().map(|f| f.0.to_string()).collect::<Vec<_>>();

        let locales = super::build_locale_names_from_files(&paths).unwrap();
        locales
        .iter()
        .enumerate()
        // .map(|key| ident(&key.0.to_lower_camel_case())).collect::<Vec<_>>();
        .for_each(|(index,name)| {
            assert_eq!(name.0.to_snake_case(),names[index])
        });

        Ok(())
    }
}

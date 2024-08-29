
use std::{collections::HashSet, fmt::Display};

use crate::schema::{Key, LocaleName};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("{0}")]
    JsonParsing(#[from] serde_json::error::Error),
    #[error("{0}")]
    ProcMacroInput(#[from] syn::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    // #[error("Missing environment variable `{name}`,`{inner_error}`")]
    // MissingEnvVar {
    //     name: String,
    //     inner_error: std::env::VarError,
    // },
    // #[error("Missing folder path")]
    // MissingFolderPath,
    #[error("The locales directory should not contain other folders")]
    DirectoryInLocalesFolder,
    #[error("Failed to get file stem of locale file")]
    NoFileStem,
    #[error("File name contained invalid UTF-8")]
    InvalidUtf8InFileName,
    #[error("Unbalanced placeholders in string, Locale: {locale_name}, String: {string}")]
    UnbalancedPlaceholders {
        locale_name: LocaleName,
        string: String,
    },
    #[error("Missing keys in locale: {0:?}")]
    MissingKeysInLocale(Vec<MissingKeysInLocale>),
}

#[derive(Debug)]
pub(crate) struct MissingKeysInLocale {
    pub(crate) locale_name: LocaleName,
    pub(crate) keys: HashSet<Key>,
}

impl Display for MissingKeysInLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_lists: Vec<String> = self.keys.iter().map(|f| f.0.to_string()).collect();
        let key_lists = key_lists.join(",");

        write!(f, "[{}],{}", key_lists, self.locale_name)
    }
}


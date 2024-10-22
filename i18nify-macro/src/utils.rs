use crate::{
    error::{Error, Result},
    schema::LocaleName,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub(crate) fn find_locale_files<P>(full_locales_path: P) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let file_ext = if cfg!(feature = "json") {
        "json"
    } else if cfg!(feature = "toml") {
        "toml"
    } else {
        "json"
    };
    let paths = std::fs::read_dir(full_locales_path)?
        .map(|entry| {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                Err(Error::DirectoryInLocalesFolder)
            } else {
                Ok(path)
            }
        })
        .filter(|path| match path {
            Ok(path) => path
                .extension()
                .map(|ext| ext == file_ext)
                .unwrap_or_else(|| false),
            // don't throw errors away
            Err(_) => true,
        })
        .collect::<Result<_, Error>>()?;
    Ok(paths)
}

#[allow(clippy::ptr_arg)]
pub(crate) fn locale_name_from_translations_file_path(path: &PathBuf) -> Result<LocaleName> {
    let file_stem = path
        .file_stem()
        .ok_or_else(|| Error::NoFileStem)?
        .to_str()
        .ok_or_else(|| Error::InvalidUtf8InFileName)?;
    let name = uppercase_first_letter(file_stem);
    Ok(LocaleName(name))
}

pub(crate) fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub(crate) fn parse_translations_file(contents: &str) -> Result<HashMap<String, String>> {
    
    #[cfg(feature = "json")]
    {
        serde_json::from_str(contents).map_err(From::from)
    }

    #[cfg(feature = "toml")]
    {
        toml::from_str(contents).map_err(crate::Error::TomlParsing)
    }
    
    #[cfg(not(any(feature = "json", feature = "toml")))]
    {
        Err(Error::UnsupportedFormat)
    }
}

#[cfg(feature = "toml")]
#[cfg(test)]
mod test {

    #[test]
    fn test_parse_translations_file() {
        let contents = r#"
        hello_world = "Hello, World!"
        greeting = "Hello {name}"
        "#;
        let r = super::parse_translations_file(contents).unwrap();
        assert!(r.contains_key("hello_world"));
        assert!(r.contains_key("greeting"));
    }
}

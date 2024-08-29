use std::{collections::{HashMap, HashSet}, fmt::Display};


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct Key(pub(crate) String);

#[derive(Debug)]
pub(crate) struct Translation(pub(crate) String);

#[derive(Debug, Clone)]
pub(crate) struct Placeholders(pub(crate) HashSet<String>);

#[derive(Debug)]
pub(crate) struct I18nKey {
    pub(crate) key: Key,
    pub(crate) translation: Translation,
    pub(crate) placeholders: Placeholders,
}

pub(crate) type Translations = HashMap<Key, HashMap<LocaleName, (Translation, Placeholders)>>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct LocaleName(pub(crate) String);

impl LocaleName {
    #[cfg(test)]
    pub(crate) fn new<T: Into<String>>(t: T) -> LocaleName {
        LocaleName(t.into())
    }
}

impl Display for LocaleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) open: String,
    pub(crate) close: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            open: "{".to_string(),
            close: "}".to_string(),
        }
    }
}
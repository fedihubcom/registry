pub mod handlebars_helpers;

use std::collections::HashMap;

use fluent_bundle::{FluentError, FluentResource};
use fluent_bundle::concurrent::FluentBundle;
use unic_langid::LanguageIdentifier;

pub struct I18n(HashMap<String, FluentBundle<FluentResource>>);

pub struct L10n<'a>(&'a FluentBundle<FluentResource>);

#[derive(Debug)]
pub enum I18nError {
    // For `I18n::new`
    InvalidLocale(String),
    InvalidPath,
    CantReadFile(String),
    CantBuildResource(String),
    CantAddResource(String),

    // For `I18n.l10n`
    NoSuchLocale(String),

    // For `L10n.translate`
    NoSuchTranslation(String),
    InvalidTranslation(String),
    FormattingFailed(Vec<FluentError>),
}

impl I18n {
    pub fn new(path: &str, locales: &[&str]) -> Result<Self, I18nError> {
        let locales_and_lang_ids: Vec<(&str, Result<LanguageIdentifier, _>)> =
            locales.iter().map(|locale| {
                (*locale, locale.parse::<LanguageIdentifier>())
            }).collect();

        if let Some((locale, _)) =
            locales_and_lang_ids.iter().find(|(_, lang_id)| lang_id.is_err())
        {
            return Err(I18nError::InvalidLocale(locale.to_string()));
        }

        let lang_ids: Vec<&LanguageIdentifier> = locales_and_lang_ids.iter()
            .map(|(_, lang_id)| lang_id.as_ref().unwrap())
            .collect();

        let mut hash_map = HashMap::new();

        for lang_id in lang_ids {
            let locale = lang_id.to_string();

            let mut path_buf = std::path::PathBuf::from(path);
            path_buf.push(&locale);
            path_buf.set_extension("ftl");

            let path_str = match path_buf.to_str() {
                Some(path_str) => path_str,
                None => return Err(I18nError::InvalidPath),
            };

            let data = match std::fs::read_to_string(path_str) {
                Ok(data) => data,
                Err(_) => return Err(I18nError::CantReadFile(
                    path_str.to_string(),
                )),
            };

            let resource = match FluentResource::try_new(data) {
                Ok(resource) => resource,
                Err(_) => return Err(I18nError::CantBuildResource(
                    path_str.to_string(),
                )),
            };

            let mut bundle = FluentBundle::default();

            if let Err(_) = bundle.add_resource(resource) {
                return Err(I18nError::CantAddResource(path_str.to_string()));
            };

            hash_map.insert(locale, bundle);
        }

        Ok(Self(hash_map))
    }

    pub fn l10n<'a>(&'a self, locale: &'a str) -> Result<L10n<'a>, I18nError> {
        match self.0.get(locale) {
            None => Err(I18nError::NoSuchLocale(locale.to_string())),
            Some(bundle) => Ok(L10n(bundle)),
        }
    }
}

impl L10n<'_> {
    pub fn translate(&self, key: &str) -> Result<String, I18nError> {
        let msg = match self.0.get_message(key) {
            None => return Err(I18nError::NoSuchTranslation(key.to_string())),
            Some(msg) => msg,
        };

        let val = match msg.value {
            None => return Err(I18nError::InvalidTranslation(key.to_string())),
            Some(val) => val,
        };

        let mut errors = vec![];

        let out = self.0.format_pattern(val, None, &mut errors).to_string();

        if !errors.is_empty() {
            return Err(I18nError::FormattingFailed(errors));
        }

        Ok(out)
    }
}

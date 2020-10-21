use std::collections::HashMap;

use fluent_bundle::FluentResource;
use fluent_bundle::concurrent::FluentBundle;
use unic_langid::LanguageIdentifier;

pub struct I18n(HashMap<String, FluentBundle<FluentResource>>);

pub struct L10n<'a>(&'a FluentBundle<FluentResource>);

impl I18n {
    pub fn new(path: &str, locales: &[&str]) -> Result<Self, ()> {
        let lang_ids: Vec<Result<LanguageIdentifier, _>> =
            locales.iter().map(|locale| {
                locale.parse::<LanguageIdentifier>()
            }).collect();

        if let Some(_) = lang_ids.iter().find(|lang_id| lang_id.is_err()) {
            return Err(());
        }

        let lang_ids: Vec<&LanguageIdentifier> =
            lang_ids.iter().map(|lang_id| lang_id.as_ref().unwrap()).collect();

        let mut hash_map = HashMap::new();

        for lang_id in lang_ids {
            let locale = lang_id.to_string();

            let mut path_buf = std::path::PathBuf::from(path);
            path_buf.push(&locale);
            path_buf.set_extension("ftl");

            let data = match std::fs::read_to_string(path_buf) {
                Ok(data) => data,
                Err(_) => return Err(()),
            };

            let resource = match FluentResource::try_new(data) {
                Ok(resource) => resource,
                Err(_) => return Err(()),
            };

            let mut bundle = FluentBundle::default();

            if let Err(_) = bundle.add_resource(resource) {
                return Err(());
            };

            hash_map.insert(locale, bundle);
        }

        Ok(Self(hash_map))
    }

    pub fn l10n<'a>(&'a self, locale: &'a str) -> Option<L10n<'a>> {
        match self.0.get(locale) {
            None => None,
            Some(bundle) => Some(L10n(bundle)),
        }
    }
}

impl L10n<'_> {
    pub fn dummy_translate(&self, key: &str) -> String {
        let msg = self.0.get_message(key).unwrap();
        let val = msg.value.unwrap();
        let mut errors = vec![];
        self.0.format_pattern(val, None, &mut errors).to_string()
    }
}

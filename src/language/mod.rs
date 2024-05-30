pub mod dictionaries;
pub mod translators;

#[derive(Debug)]
pub enum Error {
    Deserialization(String),
    TranslatorNotAvailable(String),
    Request(String),
    Google(u16),
    DeeplX(u16),
    NoTranslation,
}

#[derive(Debug)]
pub struct TranslatorResponse {
    pub translation: String,
    pub alternatives: Option<Vec<String>>,
}

pub fn get_tranlators_codes() -> Vec<&'static str> {
    ["google_api_v1", "google_api_v2", "google_scrape", "deeplx"].to_vec()
}

pub mod prelude {
    use std::collections::BTreeMap;

    pub trait Translator {
        fn new(url_override: Option<String>) -> Self
        where
            Self: Sized;
        fn translate(
            &self,
            text: &str,
            in_lang_code: &str,
            out_lang_code: &str,
        ) -> Result<super::TranslatorResponse, super::Error>;
        fn get_url(&self) -> String;
        fn get_name() -> String;
        fn get_api_key_url() -> Option<String>;
        fn get_language_map() -> BTreeMap<&'static str, &'static str>;
    }

    // pub trait Dictionary {
    //     fn new() -> Self;
    //     fn lookup(&self, word: &str);
    // }

    pub trait ApiKey {
        fn set_key(&mut self, key: String);
        fn get_key(&self) -> Option<String>;
    }
}

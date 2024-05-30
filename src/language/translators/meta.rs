use std::collections::BTreeMap;

use crate::language::{
    prelude::*,
    translators::{deeplx, google},
    Error, TranslatorResponse,
};

pub struct MetaTranslator {
    url_override: Option<String>,
    key: Option<String>,
    translator: String,
}

impl MetaTranslator {
    pub fn new(translator: String, url_override: Option<String>) -> Self {
        MetaTranslator {
            url_override,
            key: None,
            translator,
        }
    }

    pub fn translate(
        &self,
        text: &str,
        in_lang_code: &str,
        out_lang_code: &str,
    ) -> Result<TranslatorResponse, Error> {
        match self.translator.as_str() {
            "google_api_v1" => google::GoogleApiV1::new(self.url_override.clone()).translate(
                text,
                in_lang_code,
                out_lang_code,
            ),
            "google_scrape" => google::GoogleScrape::new(self.url_override.clone()).translate(
                text,
                in_lang_code,
                out_lang_code,
            ),
            "google_api_v2" => {
                let mut google_api_v2 = google::GoogleApiV2::new(self.url_override.clone());
                if let Some(key) = self.key.clone() {
                    google_api_v2.set_key(key);
                }
                google_api_v2.translate(text, in_lang_code, out_lang_code)
            }
            "deeplx" => {
                let mut deeplx = deeplx::DeeplX::new(self.url_override.clone());
                if let Some(key) = self.key.clone() {
                    deeplx.set_key(key);
                }
                deeplx.translate(text, in_lang_code, out_lang_code)
            }
            _ => Err(Error::TranslatorNotAvailable(self.translator.clone())),
        }
    }

    pub fn get_url(&self) -> String {
        match self.translator.as_str() {
            "google_api_v1" => google::GoogleApiV1::new(self.url_override.clone()).get_url(),
            "google_api_v2" => google::GoogleApiV2::new(self.url_override.clone()).get_url(),
            "google_scrape" => google::GoogleScrape::new(self.url_override.clone()).get_url(),
            "deeplx" => deeplx::DeeplX::new(self.url_override.clone()).get_url(),
            _ => "".to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        match self.translator.as_str() {
            "google_api_v1" => google::GoogleApiV1::get_name(),
            "google_api_v2" => google::GoogleApiV2::get_name(),
            "google_scrape" => google::GoogleScrape::get_name(),
            "deeplx" => deeplx::DeeplX::get_name(),
            _ => "".to_string(),
        }
    }

    pub fn get_api_key_url(&self) -> Option<String> {
        match self.translator.as_str() {
            "google_api_v1" => google::GoogleApiV1::get_api_key_url(),
            "google_api_v2" => google::GoogleApiV2::get_api_key_url(),
            "google_scrape" => google::GoogleScrape::get_api_key_url(),
            "deeplx" => deeplx::DeeplX::get_api_key_url(),
            _ => None,
        }
    }

    pub fn get_language_map(&self) -> BTreeMap<&'static str, &'static str> {
        match self.translator.as_str() {
            "google_api_v1" => google::GoogleApiV1::get_language_map(),
            "google_api_v2" => google::GoogleApiV2::get_language_map(),
            "google_scrape" => google::GoogleScrape::get_language_map(),
            "deeplx" => deeplx::DeeplX::get_language_map(),
            _ => BTreeMap::new(),
        }
    }
}

impl ApiKey for MetaTranslator {
    fn get_key(&self) -> Option<String> {
        self.key.clone()
    }

    fn set_key(&mut self, key: String) {
        self.key = Some(key);
    }
}
